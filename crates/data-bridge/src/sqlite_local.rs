//! 嵌入式原生 SQLite 本地数据驱动 (Embedded SQLite Native Driver)

use crate::database::{DatabaseClient, DbRow, DbValue};
use rusqlite::types::ValueRef;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 基于原生 SQLite 的嵌入式数据库连接包装
#[derive(Clone)]
pub struct EmbeddedDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl Default for EmbeddedDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl EmbeddedDatabase {
    /// 创建内存数据库连接 (适合快速单测与临时缓存)
    pub fn new() -> Self {
        Self::new_in_memory().expect("初始化 SQLite 内存数据库失败")
    }

    /// 明确创建内存模式数据库
    pub fn new_in_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// 打开或新建本地磁盘 SQLite 单文件数据库
    pub fn open_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// 执行带参数的 SQL 变更命令
    pub fn execute_params(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> Result<usize, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(sql, params).map_err(|e| e.to_string())
    }

    /// 执行带参数的 SQL 查询
    pub fn query_params(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> Result<Vec<DbRow>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;

        let col_count = stmt.column_count();
        let col_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let row_iter = stmt
            .query_map(params, |row| {
                let mut cols = Vec::with_capacity(col_count);
                for i in 0..col_count {
                    let name = col_names[i].clone();
                    let val_ref = row.get_ref(i)?;
                    let db_val = match val_ref {
                        ValueRef::Null => DbValue::Null,
                        ValueRef::Integer(v) => DbValue::Integer(v),
                        ValueRef::Real(v) => DbValue::Real(v),
                        ValueRef::Text(v) => DbValue::Text(String::from_utf8_lossy(v).into_owned()),
                        ValueRef::Blob(v) => DbValue::Blob(v.to_vec()),
                    };
                    cols.push((name, db_val));
                }
                Ok(DbRow { columns: cols })
            })
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for r in row_iter {
            results.push(r.map_err(|e| e.to_string())?);
        }
        Ok(results)
    }
}

impl DatabaseClient for EmbeddedDatabase {
    fn execute(&self, sql: &str) -> Result<usize, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(sql, []).map_err(|e| e.to_string())
    }

    fn query(&self, sql: &str) -> Result<Vec<DbRow>, String> {
        self.query_params(sql, &[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_ddl_dml_and_query() {
        let db = EmbeddedDatabase::new_in_memory().unwrap();

        // 1. 建表
        db.execute(
            "CREATE TABLE tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                priority INTEGER DEFAULT 1
            );",
        )
        .unwrap();

        // 2. 插入 3 条记录
        db.execute("INSERT INTO tasks (id, title, priority) VALUES (1, '任务一', 1);")
            .unwrap();
        db.execute("INSERT INTO tasks (id, title, priority) VALUES (2, '完成 SQLite 替换', 5);")
            .unwrap();
        db.execute("INSERT INTO tasks (id, title, priority) VALUES (3, '任务三', 2);")
            .unwrap();

        // 3. 精确查出 id = 2 的记录
        let rows = db.query("SELECT * FROM tasks WHERE id = 2;").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get_text("title"), Some("完成 SQLite 替换"));

        // 4. 校验整数列
        let priority_col = rows[0]
            .columns
            .iter()
            .find(|(k, _)| k == "priority")
            .unwrap();
        assert_eq!(priority_col.1, DbValue::Integer(5));
    }
}
