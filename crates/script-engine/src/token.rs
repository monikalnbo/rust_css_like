//! 表达式 Token 词法标记定义 (Expression Tokens)

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Int(i64),
    Float(f64),
    String(String),
    Ident(String),
    ColorHex(String),
    // 算术与赋值
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Arrow,       // ->
    Assign,      // =
    PlusAssign,  // +=
    MinusAssign, // -=
    // 比较与逻辑
    Eq,  // ==
    Ne,  // !=
    Lt,  // <
    Le,  // <=
    Gt,  // >
    Ge,  // >=
    And, // &&
    Or,  // ||
    Not, // !
    // 分隔与括号
    Dot,
    Question,
    Colon,
    OpenParen,
    CloseParen,
    Comma,
    Eof,
}
