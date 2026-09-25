//! 动态脚本运行时值系统 (Script Value System)

use std::collections::HashMap;

/// 前端脚本动态值枚举（用于支撑无需 JS 虚拟机的极轻量运行时运算）
#[derive(Clone, Debug, PartialEq)]
pub enum ScriptValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<ScriptValue>),
    Map(HashMap<String, ScriptValue>),
}

impl Default for ScriptValue {
    fn default() -> Self {
        Self::Null
    }
}

impl ScriptValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            Self::Int(i) => *i != 0,
            Self::Float(f) => *f != 0.0 && !f.is_nan(),
            Self::String(s) => !s.is_empty(),
            Self::List(l) => !l.is_empty(),
            Self::Map(m) => !m.is_empty(),
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Int(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::String(s) => s.clone(),
            Self::List(l) => format!("[{} items]", l.len()),
            Self::Map(m) => format!("{{{} entries}}", m.len()),
        }
    }
}
