//! 顶级声明与控制流语句解析器 (Declaration & Control Flow Parser)

use crate::ast::{ScopeBlock, ScopeKind};
use crate::parser::Parser;
use crate::token::{Token, TokenKind};

pub struct DeclParser;

/// 将 Token 逆向格式化为表达式或动作文本字符串
pub fn format_token_to_expr(kind: &TokenKind, out: &mut String) {
    match kind {
        TokenKind::StringLiteral(s) => out.push_str(&format!("\"{}\"", s)),
        TokenKind::Ident(id) => {
            if !out.is_empty()
                && !out.ends_with(' ')
                && !out.ends_with('(')
                && !out.ends_with('[')
                && !out.ends_with('.')
                && !out.ends_with('!')
            {
                out.push(' ');
            }
            out.push_str(id);
        }
        TokenKind::Number(n) => {
            if !out.is_empty()
                && !out.ends_with(' ')
                && !out.ends_with('(')
                && !out.ends_with('[')
                && !out.ends_with(',')
            {
                out.push(' ');
            }
            if n.fract() == 0.0 {
                out.push_str(&format!("{}", *n as i64));
            } else {
                out.push_str(&n.to_string());
            }
        }
        TokenKind::Dimension(n, u) => {
            if !out.is_empty()
                && !out.ends_with(' ')
                && !out.ends_with('(')
                && !out.ends_with('[')
                && !out.ends_with(',')
            {
                out.push(' ');
            }
            if n.fract() == 0.0 {
                out.push_str(&format!("{}{}", *n as i64, u));
            } else {
                out.push_str(&format!("{}{}", n, u));
            }
        }
        TokenKind::HexColor(h) => {
            if !out.is_empty() && !out.ends_with(' ') {
                out.push(' ');
            }
            out.push_str(h);
        }
        TokenKind::Variable(v) => {
            if !out.is_empty() && !out.ends_with(' ') {
                out.push(' ');
            }
            out.push_str(v);
        }
        TokenKind::Dot => out.push('.'),
        TokenKind::Comma => out.push_str(", "),
        TokenKind::Colon => out.push_str(": "),
        TokenKind::Question => out.push_str(" ? "),
        TokenKind::Exclamation => out.push('!'),
        TokenKind::OpenParen => out.push('('),
        TokenKind::CloseParen => out.push(')'),
        TokenKind::OpenBracket => out.push('['),
        TokenKind::CloseBracket => out.push(']'),
        TokenKind::Equals => out.push_str(" = "),
        TokenKind::DoubleEquals => out.push_str(" == "),
        TokenKind::NotEquals => out.push_str(" != "),
        TokenKind::PlusEquals => out.push_str(" += "),
        TokenKind::MinusEquals => out.push_str(" -= "),
        TokenKind::Plus => out.push_str(" + "),
        TokenKind::Minus => out.push_str(" - "),
        TokenKind::Star => out.push_str(" * "),
        TokenKind::Slash => out.push_str(" / "),
        _ => {}
    }
}

impl DeclParser {
    /// 尝试解析控制流与声明语句 (@import, let, component, for, if, else)
    pub fn try_parse_decl(parser: &mut Parser) -> Result<Option<ScopeBlock>, String> {
        parser.skip_newlines();
        let Some(tok) = parser.tokens.get(parser.cursor) else {
            return Ok(None);
        };
        let start_span = tok.span;

        match &tok.kind {
            TokenKind::KwImport => {
                parser.cursor += 1;
                parser.skip_newlines();
                if let Some(Token {
                    kind: TokenKind::StringLiteral(path),
                    ..
                }) = parser.tokens.get(parser.cursor)
                {
                    let path = path.clone();
                    parser.cursor += 1;
                    if parser.peek() == &TokenKind::Semicolon || parser.peek() == &TokenKind::Newline {
                        parser.bump();
                    }
                    return Ok(Some(ScopeBlock::new(ScopeKind::Import(path), start_span)));
                }
                Ok(None)
            }
            TokenKind::KwLet => {
                parser.cursor += 1;
                parser.skip_newlines();
                if let Some(Token {
                    kind: TokenKind::Ident(var_name),
                    ..
                }) = parser.tokens.get(parser.cursor)
                {
                    let var_name = var_name.clone();
                    parser.cursor += 1;
                    if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::Equals) {
                        parser.cursor += 1;
                        let mut expr = String::new();
                        while let Some(t) = parser.tokens.get(parser.cursor) {
                            if t.kind == TokenKind::Semicolon
                                || t.kind == TokenKind::Newline
                                || t.kind == TokenKind::CloseBrace
                                || t.kind == TokenKind::Eof
                            {
                                break;
                            }
                            format_token_to_expr(&t.kind, &mut expr);
                            parser.cursor += 1;
                        }
                        if parser.peek() == &TokenKind::Semicolon || parser.peek() == &TokenKind::Newline {
                            parser.bump();
                        }
                        return Ok(Some(ScopeBlock::new(
                            ScopeKind::StateLet {
                                name: var_name,
                                init_expr: expr.trim().to_string(),
                            },
                            start_span,
                        )));
                    }
                }
                Ok(None)
            }
            TokenKind::KwComponent => {
                parser.cursor += 1;
                parser.skip_newlines();
                if let Some(Token {
                    kind: TokenKind::Ident(name),
                    ..
                }) = parser.tokens.get(parser.cursor)
                {
                    let name = name.clone();
                    parser.cursor += 1;
                    let mut params = Vec::new();
                    if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::OpenParen) {
                        parser.cursor += 1;
                        while let Some(t) = parser.tokens.get(parser.cursor) {
                            if t.kind == TokenKind::CloseParen || t.kind == TokenKind::Eof {
                                break;
                            }
                            if let TokenKind::Ident(p) = &t.kind {
                                params.push(p.clone());
                                parser.cursor += 1;
                                if parser.tokens.get(parser.cursor).map(|tk| &tk.kind) == Some(&TokenKind::Comma) {
                                    parser.cursor += 1;
                                }
                            } else {
                                break;
                            }
                        }
                        if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::CloseParen) {
                            parser.cursor += 1;
                        }
                    }
                    parser.skip_newlines();
                    let mut block = ScopeBlock::new(ScopeKind::ComponentDef { name, params }, start_span);
                    parser.parse_block_body(&mut block)?;
                    return Ok(Some(block));
                }
                Ok(None)
            }
            TokenKind::KwFor => {
                parser.cursor += 1;
                parser.skip_newlines();
                if let Some(Token {
                    kind: TokenKind::Ident(item_var),
                    ..
                }) = parser.tokens.get(parser.cursor)
                {
                    let item_var = item_var.clone();
                    parser.cursor += 1;
                    parser.skip_newlines();
                    if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::KwIn) {
                        parser.cursor += 1;
                        parser.skip_newlines();
                        let mut iterable = String::new();
                        while let Some(t) = parser.tokens.get(parser.cursor) {
                            if t.kind == TokenKind::OpenBrace || t.kind == TokenKind::Newline || t.kind == TokenKind::Eof {
                                break;
                            }
                            format_token_to_expr(&t.kind, &mut iterable);
                            parser.cursor += 1;
                        }
                        parser.skip_newlines();
                        let mut block = ScopeBlock::new(
                            ScopeKind::ForLoop {
                                item_var,
                                iterable: iterable.trim().to_string(),
                            },
                            start_span,
                        );
                        parser.parse_block_body(&mut block)?;
                        return Ok(Some(block));
                    }
                }
                Ok(None)
            }
            TokenKind::KwIf => {
                parser.cursor += 1;
                parser.skip_newlines();
                let mut condition = String::new();
                while let Some(t) = parser.tokens.get(parser.cursor) {
                    if t.kind == TokenKind::OpenBrace || t.kind == TokenKind::Newline || t.kind == TokenKind::Eof {
                        break;
                    }
                    format_token_to_expr(&t.kind, &mut condition);
                    parser.cursor += 1;
                }
                parser.skip_newlines();
                let mut block = ScopeBlock::new(
                    ScopeKind::IfBranch {
                        condition: condition.trim().to_string(),
                    },
                    start_span,
                );
                parser.parse_block_body(&mut block)?;
                Ok(Some(block))
            }
            TokenKind::KwElse => {
                parser.cursor += 1;
                parser.skip_newlines();
                let mut block = ScopeBlock::new(ScopeKind::ElseBranch, start_span);
                parser.parse_block_body(&mut block)?;
                Ok(Some(block))
            }
            _ => Ok(None),
        }
    }
}
