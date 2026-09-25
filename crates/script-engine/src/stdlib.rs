//! 内建标准工具函数库 (Built-in Standard Library for Script Engine)

use crate::value::ScriptValue;

pub struct Stdlib;

impl Stdlib {
    /// 执行内建函数调用
    pub fn call(name: &str, args: &[ScriptValue]) -> ScriptValue {
        match name {
            // --- 字符串函数族 ---
            "len" => match args.first() {
                Some(ScriptValue::String(s)) => ScriptValue::Int(s.len() as i64),
                Some(ScriptValue::List(l)) => ScriptValue::Int(l.len() as i64),
                Some(ScriptValue::Map(m)) => ScriptValue::Int(m.len() as i64),
                _ => ScriptValue::Int(0),
            },
            "trim" => match args.first() {
                Some(v) => ScriptValue::String(v.to_display_string().trim().to_string()),
                None => ScriptValue::String(String::new()),
            },
            "upper" => match args.first() {
                Some(v) => ScriptValue::String(v.to_display_string().to_uppercase()),
                None => ScriptValue::String(String::new()),
            },
            "lower" => match args.first() {
                Some(v) => ScriptValue::String(v.to_display_string().to_lowercase()),
                None => ScriptValue::String(String::new()),
            },
            "substr" => {
                let s = args
                    .first()
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                let start = args
                    .get(1)
                    .and_then(|v| match v {
                        ScriptValue::Int(i) => Some(*i as usize),
                        _ => None,
                    })
                    .unwrap_or(0);
                let len = args.get(2).and_then(|v| match v {
                    ScriptValue::Int(i) => Some(*i as usize),
                    _ => None,
                });
                let chars: Vec<char> = s.chars().collect();
                if start >= chars.len() {
                    return ScriptValue::String(String::new());
                }
                let end = match len {
                    Some(l) => (start + l).min(chars.len()),
                    None => chars.len(),
                };
                ScriptValue::String(chars[start..end].iter().collect())
            }
            "replace" => {
                let s = args
                    .first()
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                let from = args
                    .get(1)
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                let to = args
                    .get(2)
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                ScriptValue::String(s.replace(&from, &to))
            }
            "split" => {
                let s = args
                    .first()
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                let sep = args
                    .get(1)
                    .map(|v| v.to_display_string())
                    .unwrap_or_else(|| ",".to_string());
                let parts: Vec<ScriptValue> = s
                    .split(&sep)
                    .map(|p| ScriptValue::String(p.to_string()))
                    .collect();
                ScriptValue::List(parts)
            }
            "starts_with" => {
                let s = args
                    .first()
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                let prefix = args
                    .get(1)
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                ScriptValue::Bool(s.starts_with(&prefix))
            }
            "ends_with" => {
                let s = args
                    .first()
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                let suffix = args
                    .get(1)
                    .map(|v| v.to_display_string())
                    .unwrap_or_default();
                ScriptValue::Bool(s.ends_with(&suffix))
            }

            // --- 集合与列表函数族 ---
            "contains" => match (args.first(), args.get(1)) {
                (Some(ScriptValue::List(l)), Some(item)) => ScriptValue::Bool(l.contains(item)),
                (Some(ScriptValue::String(s)), Some(item)) => {
                    let needle = item.to_display_string();
                    ScriptValue::Bool(s.contains(&needle))
                }
                _ => ScriptValue::Bool(false),
            },

            // --- 数学函数族 ---
            "abs" => match args.first() {
                Some(ScriptValue::Int(i)) => ScriptValue::Int(i.abs()),
                Some(ScriptValue::Float(f)) => ScriptValue::Float(f.abs()),
                _ => ScriptValue::Int(0),
            },
            "clamp" => {
                let v = args
                    .first()
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(0.0);
                let min = args
                    .get(1)
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(0.0);
                let max = args
                    .get(2)
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(1.0);
                ScriptValue::Float(v.clamp(min, max))
            }
            "min" => {
                let a = args
                    .first()
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(0.0);
                let b = args
                    .get(1)
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(0.0);
                ScriptValue::Float(a.min(b))
            }
            "max" => {
                let a = args
                    .first()
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(0.0);
                let b = args
                    .get(1)
                    .and_then(|x| match x {
                        ScriptValue::Float(f) => Some(*f),
                        ScriptValue::Int(i) => Some(*i as f64),
                        _ => None,
                    })
                    .unwrap_or(0.0);
                ScriptValue::Float(a.max(b))
            }

            // --- 时间函数 ---
            "now" => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0);
                ScriptValue::Int(now)
            }

            _ => ScriptValue::Null,
        }
    }
}
