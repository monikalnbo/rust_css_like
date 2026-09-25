//! 具备树形作用域继承的 CSS 变量符号表 (Scoped CSS Variable Table)

use std::collections::HashMap;
use std::sync::Arc;

/// 具备层级作用域继承特性的变量符号表
#[derive(Clone, Debug, Default)]
pub struct VariableTable {
    vars: HashMap<String, String>,
    parent: Option<Arc<VariableTable>>,
}

impl VariableTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// 基于当前作用域派生子级变量符号表
    pub fn child(parent: Arc<VariableTable>) -> Self {
        Self {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// 在当前作用域设置变量 (子节点可自由覆盖而不污染父级)
    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.vars.insert(key.into(), val.into());
    }

    /// 沿作用域链递归向上查找变量
    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(v) = self.vars.get(key) {
            return Some(v.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get(key);
        }
        None
    }

    /// 解析变量名，例如输入 "$primary" 或 "--primary-color"
    pub fn resolve(&self, expression: &str) -> Option<String> {
        let clean = expression
            .strip_prefix('$')
            .or_else(|| expression.strip_prefix("--"))
            .unwrap_or(expression);
        self.get(clean)
    }
}
