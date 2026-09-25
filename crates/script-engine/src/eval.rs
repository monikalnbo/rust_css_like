//! 表达式求值与动作执行解释器 (AST Evaluator & Action Executor)

use crate::ast::{Action, BinaryOp, Expr, UnaryOp};
use crate::binary_op::BinaryEvaluator;
use crate::interpolate::Interpolator;
use crate::lexer::Lexer;
use crate::pratt::PrattParser;
use crate::scope::ScriptScope;
use crate::value::ScriptValue;

pub struct Evaluator;

impl Evaluator {
    /// 评估字符串表达式，经过词法分析、Pratt 语法树构建与求值
    pub fn eval_expr(expr_str: &str, scope: &ScriptScope) -> ScriptValue {
        let trimmed = expr_str.trim();
        if trimmed.is_empty() {
            return ScriptValue::Null;
        }

        let mut lexer = Lexer::new(trimmed);
        let Ok(tokens) = lexer.tokenize() else {
            return ScriptValue::Null;
        };

        let mut parser = PrattParser::new(tokens);
        let Ok(ast) = parser.parse_expression(0) else {
            return ScriptValue::Null;
        };

        Self::eval_node(&ast, scope)
    }

    /// 执行动作流语句并直接修改作用域变量
    pub fn execute_action(action_str: &str, scope: &ScriptScope) -> Result<(), String> {
        let trimmed = action_str.trim();
        let mut lexer = Lexer::new(trimmed);
        let tokens = lexer.tokenize()?;

        let mut parser = PrattParser::new(tokens);
        let action = parser.parse_action()?;

        match action {
            Action::Assign(target, expr) => {
                let val = Self::eval_node(&expr, scope);
                scope.set(target, val);
            }
            Action::CompoundAssign(target, op, expr) => {
                let current = scope.get(&target).unwrap_or(ScriptValue::Int(0));
                let delta = Self::eval_node(&expr, scope);
                let new_val = BinaryEvaluator::eval_binary(op, &current, &delta);
                scope.set(target, new_val);
            }
            Action::Expr(expr) => {
                Self::eval_node(&expr, scope);
            }
        }
        Ok(())
    }

    /// 递归遍历 AST 求值
    pub fn eval_node(expr: &Expr, scope: &ScriptScope) -> ScriptValue {
        match expr {
            Expr::Literal(val) => val.clone(),
            Expr::Ident(name) => scope.get(name).unwrap_or(ScriptValue::Null),
            Expr::Dot(lhs, field) => {
                let left_val = Self::eval_node(lhs, scope);
                if let ScriptValue::Map(map) = left_val {
                    map.get(field).cloned().unwrap_or(ScriptValue::Null)
                } else {
                    ScriptValue::Null
                }
            }
            Expr::Unary(op, inner) => {
                let val = Self::eval_node(inner, scope);
                match op {
                    UnaryOp::Not => ScriptValue::Bool(!val.is_truthy()),
                    UnaryOp::Neg => match val {
                        ScriptValue::Int(i) => ScriptValue::Int(-i),
                        ScriptValue::Float(f) => ScriptValue::Float(-f),
                        _ => ScriptValue::Null,
                    },
                }
            }
            Expr::Binary(op, lhs, rhs) => {
                // 逻辑短路
                if *op == BinaryOp::And {
                    let left_val = Self::eval_node(lhs, scope);
                    if !left_val.is_truthy() {
                        return ScriptValue::Bool(false);
                    }
                    return ScriptValue::Bool(Self::eval_node(rhs, scope).is_truthy());
                }
                if *op == BinaryOp::Or {
                    let left_val = Self::eval_node(lhs, scope);
                    if left_val.is_truthy() {
                        return ScriptValue::Bool(true);
                    }
                    return ScriptValue::Bool(Self::eval_node(rhs, scope).is_truthy());
                }

                let left_val = Self::eval_node(lhs, scope);
                let right_val = Self::eval_node(rhs, scope);
                BinaryEvaluator::eval_binary(*op, &left_val, &right_val)
            }
            Expr::Ternary(cond, then_branch, else_branch) => {
                let cond_val = Self::eval_node(cond, scope);
                if cond_val.is_truthy() {
                    Self::eval_node(then_branch, scope)
                } else {
                    Self::eval_node(else_branch, scope)
                }
            }
            Expr::Call(name, args) => Self::eval_builtin_func(name, args, scope),
        }
    }

    fn eval_builtin_func(name: &str, args: &[Expr], scope: &ScriptScope) -> ScriptValue {
        let evaluated_args: Vec<ScriptValue> =
            args.iter().map(|arg| Self::eval_node(arg, scope)).collect();
        crate::stdlib::Stdlib::call(name, &evaluated_args)
    }

    /// 字符串插值：如 "Hello $name, count: ${count * 2}"
    #[inline]
    pub fn interpolate(template: &str, scope: &ScriptScope) -> String {
        Interpolator::interpolate(template, scope)
    }
}
