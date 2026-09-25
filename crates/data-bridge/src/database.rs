//! 本地与远程数据库连接抽象 (Database Connectors)

/// 数据库查询结果集通用表示
#[derive(Clone, Debug, PartialEq)]
pub enum DbValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(Clone, Debug, Default)]
pub struct DbRow {
    pub columns: Vec<(String, DbValue)>,
}

impl DbRow {
    pub fn get_text(&self, col_name: &str) -> Option<&str> {
        self.columns
            .iter()
            .find(|(k, _)| k == col_name)
            .and_then(|(_, v)| match v {
                DbValue::Text(s) => Some(s.as_str()),
                _ => None,
            })
    }
}

/// 统一数据库客户端接口 (支持本地嵌入式 SQLite 与远程 SQL)
pub trait DatabaseClient: Send + Sync {
    /// 执行变更命令 (INSERT, UPDATE, DELETE)
    fn execute(&self, sql: &str) -> Result<usize, String>;
    /// 执行查询并返回行列表
    fn query(&self, sql: &str) -> Result<Vec<DbRow>, String>;
}
