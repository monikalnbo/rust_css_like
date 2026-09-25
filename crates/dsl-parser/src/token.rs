//! 词法单元定义 (Tokens, Delimiters & Keywords)

use crate::span::Span;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // 关键字
    KwImport,    // @import
    KwComponent, // component
    KwFor,       // for
    KwIn,        // in
    KwIf,        // if
    KwElse,      // else
    KwLet,       // let

    // 标识符与字面量
    Ident(String),
    StringLiteral(String),
    Number(f32),
    HexColor(String),
    Variable(String), // 例如 $theme.primary

    // 约束符与界定符
    OpenBrace,   // `{`
    CloseBrace,  // `}`
    OpenParen,   // `(`
    CloseParen,  // `)`
    Colon,       // `:`
    Semicolon,   // `;`
    Comma,       // `,`
    Dot,         // `.`
    Spread,      // `...`
    Arrow,       // `->`
    Question,    // `?`
    Exclamation, // `!`
    Equals,      // `=`

    // 算术操作符
    Plus,  // `+`
    Minus, // `-`
    Star,  // `*`
    Slash, // `/`

    // 状态伪类标记
    PseudoColon, // 紧贴标示符的 `:`，如 `:hover`

    // 结束标记
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
