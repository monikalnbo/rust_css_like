//! 响应式前端状态作用域环境 (Script Scope Environment)

use crate::value::ScriptValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 具备父子层级继承的脚本变量环境表
#[derive(Clone, Debug, Default)]
pub struct ScriptScope {
    bindings: Arc<Mutex<HashMap<String, ScriptValue>>>,
    parent: Option<Arc<ScriptScope>>,
}

impl ScriptScope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(parent: Arc<ScriptScope>) -> Self {
        Self {
            bindings: Arc::new(Mutex::new(HashMap::new())),
            parent: Some(parent),
        }
    }

    /// 声明或更新变量
    pub fn set(&self, name: impl Into<String>, val: ScriptValue) {
        self.bindings.lock().unwrap().insert(name.into(), val);
    }

    /// 递归查找变量
    pub fn get(&self, name: &str) -> Option<ScriptValue> {
        if let Some(val) = self.bindings.lock().unwrap().get(name).cloned() {
            return Some(val);
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        None
    }
}
