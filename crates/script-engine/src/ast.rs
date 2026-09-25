//! 语法抽象树定义 (Abstract Syntax Tree Nodes)

use crate::value::ScriptValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(ScriptValue),
    Ident(String),
    Dot(Box<Expr>, String),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

/// 动作流执行指令 (Action Statement)
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// 直接赋值：`target = expr`
    Assign(String, Expr),
    /// 复合赋值：`target += expr` / `target -= expr`
    CompoundAssign(String, BinaryOp, Expr),
    /// 独立表达式求值
    Expr(Expr),
}
