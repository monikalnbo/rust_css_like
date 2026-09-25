//! # Script Engine
//!
//! 基于 Pratt 算法的轻量级前端表达式分析器、AST 解释器与动作执行引擎。

pub mod ast;
pub mod binary_op;
pub mod eval;
pub mod extension;
pub mod interpolate;
pub mod lexer;
pub mod lexer_literals;
pub mod pratt;
pub mod scope;
pub mod stdlib;
pub mod token;
pub mod value;

pub use ast::{Action, BinaryOp, Expr, UnaryOp};
pub use binary_op::BinaryEvaluator;
pub use eval::Evaluator;
pub use extension::{CustomComponentPlugin, ExtensionRegistry};
pub use interpolate::Interpolator;
pub use lexer::Lexer;
pub use pratt::PrattParser;
pub use scope::ScriptScope;
pub use stdlib::Stdlib;
pub use token::Token;
pub use value::ScriptValue;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pratt_precedence_and_ternary() {
        let scope = ScriptScope::new();
        scope.set("count", ScriptValue::Int(3));

        // 3 * 2 + 1 > 5 ? 100 : 0 => 7 > 5 ? 100 : 0 => 100
        let val = Evaluator::eval_expr("count * 2 + 1 > 5 ? 100 : 0", &scope);
        assert_eq!(val, ScriptValue::Int(100));

        // 四则运算优先级校验
        let val2 = Evaluator::eval_expr("1 + 2 * 3", &scope);
        assert_eq!(val2, ScriptValue::Int(7));

        let val3 = Evaluator::eval_expr("(10 - 2) / 4", &scope);
        assert_eq!(val3, ScriptValue::Int(2));
    }

    #[test]
    fn test_action_executor_and_mutation() {
        let scope = ScriptScope::new();
        scope.set("count", ScriptValue::Int(10));
        scope.set("is_dark", ScriptValue::Bool(false));

        // 派发自增动作
        Evaluator::execute_action("-> count += 1", &scope).unwrap();
        assert_eq!(scope.get("count"), Some(ScriptValue::Int(11)));

        // 派发状态取反动作
        Evaluator::execute_action("-> is_dark = !is_dark", &scope).unwrap();
        assert_eq!(scope.get("is_dark"), Some(ScriptValue::Bool(true)));
    }

    #[test]
    fn test_dot_access_and_interpolation() {
        let scope = ScriptScope::new();
        let mut user_map = HashMap::new();
        user_map.insert("name".to_string(), ScriptValue::String("Bob".to_string()));
        scope.set("user", ScriptValue::Map(user_map));

        let res = Evaluator::interpolate("Hello ${user.name}!", &scope);
        assert_eq!(res, "Hello Bob!");
    }

    #[test]
    fn test_stdlib_functions() {
        let scope = ScriptScope::new();
        scope.set("msg", ScriptValue::String("hello world".to_string()));

        // 测试 upper 与 substr
        let v_upper = Evaluator::eval_expr("upper(msg)", &scope);
        assert_eq!(v_upper, ScriptValue::String("HELLO WORLD".to_string()));

        let v_sub = Evaluator::eval_expr("substr(msg, 0, 5)", &scope);
        assert_eq!(v_sub, ScriptValue::String("hello".to_string()));

        // 测试 clamp
        let v_clamp = Evaluator::eval_expr("clamp(120, 0, 100)", &scope);
        assert_eq!(v_clamp, ScriptValue::Float(100.0));
    }
}
