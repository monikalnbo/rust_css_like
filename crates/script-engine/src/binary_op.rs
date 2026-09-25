//! 二元运算与内建函数求值内核 (Binary Operations & Builtins)

use crate::ast::BinaryOp;
use crate::value::ScriptValue;

pub struct BinaryEvaluator;

impl BinaryEvaluator {
    pub fn eval_binary(op: BinaryOp, left: &ScriptValue, right: &ScriptValue) -> ScriptValue {
        match (left, right) {
            (ScriptValue::Int(a), ScriptValue::Int(b)) => match op {
                BinaryOp::Add => ScriptValue::Int(a + b),
                BinaryOp::Sub => ScriptValue::Int(a - b),
                BinaryOp::Mul => ScriptValue::Int(a * b),
                BinaryOp::Div => ScriptValue::Int(if *b != 0 { a / b } else { 0 }),
                BinaryOp::Mod => ScriptValue::Int(if *b != 0 { a % b } else { 0 }),
                BinaryOp::Eq => ScriptValue::Bool(a == b),
                BinaryOp::Ne => ScriptValue::Bool(a != b),
                BinaryOp::Lt => ScriptValue::Bool(a < b),
                BinaryOp::Le => ScriptValue::Bool(a <= b),
                BinaryOp::Gt => ScriptValue::Bool(a > b),
                BinaryOp::Ge => ScriptValue::Bool(a >= b),
                _ => ScriptValue::Null,
            },
            (ScriptValue::Float(a), ScriptValue::Float(b)) => match op {
                BinaryOp::Add => ScriptValue::Float(a + b),
                BinaryOp::Sub => ScriptValue::Float(a - b),
                BinaryOp::Mul => ScriptValue::Float(a * b),
                BinaryOp::Div => ScriptValue::Float(if *b != 0.0 { a / b } else { 0.0 }),
                BinaryOp::Eq => ScriptValue::Bool(a == b),
                BinaryOp::Ne => ScriptValue::Bool(a != b),
                BinaryOp::Lt => ScriptValue::Bool(a < b),
                BinaryOp::Le => ScriptValue::Bool(a <= b),
                BinaryOp::Gt => ScriptValue::Bool(a > b),
                BinaryOp::Ge => ScriptValue::Bool(a >= b),
                _ => ScriptValue::Null,
            },
            (ScriptValue::String(a), ScriptValue::String(b)) => match op {
                BinaryOp::Add => ScriptValue::String(format!("{}{}", a, b)),
                BinaryOp::Eq => ScriptValue::Bool(a == b),
                BinaryOp::Ne => ScriptValue::Bool(a != b),
                _ => ScriptValue::Null,
            },
            _ => match op {
                BinaryOp::Eq => ScriptValue::Bool(left == right),
                BinaryOp::Ne => ScriptValue::Bool(left != right),
                _ => ScriptValue::Null,
            },
        }
    }
}
