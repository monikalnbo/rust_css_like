//! 顶级声明与控制流语句解析器 (Declaration & Control Flow Parser)

use crate::ast::{ScopeBlock, ScopeKind};
use crate::parser::Parser;
use crate::token::{Token, TokenKind};

pub struct DeclParser;

impl DeclParser {
    /// 尝试解析控制流与声明语句 (@import, let, component, for, if, else)
    pub fn try_parse_decl(parser: &mut Parser) -> Result<Option<ScopeBlock>, String> {
        let Some(tok) = parser.tokens.get(parser.cursor) else {
            return Ok(None);
        };
        let start_span = tok.span;

        match &tok.kind {
            TokenKind::KwImport => {
                parser.cursor += 1;
                if let Some(Token {
                    kind: TokenKind::StringLiteral(path),
                    ..
                }) = parser.tokens.get(parser.cursor)
                {
                    let path = path.clone();
                    parser.cursor += 1;
                    if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::Semicolon) {
                        parser.cursor += 1;
                    }
                    return Ok(Some(ScopeBlock::new(ScopeKind::Import(path), start_span)));
                }
                Ok(None)
            }
            TokenKind::KwLet => {
                parser.cursor += 1;
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
                            if t.kind == TokenKind::Semicolon || t.kind == TokenKind::Eof {
                                break;
                            }
                            match &t.kind {
                                TokenKind::StringLiteral(s) => expr.push_str(&format!("\"{}\"", s)),
                                TokenKind::Ident(id) => expr.push_str(id),
                                TokenKind::Number(n) => expr.push_str(&n.to_string()),
                                TokenKind::HexColor(h) => expr.push_str(h),
                                _ => {}
                            }
                            parser.cursor += 1;
                        }
                        if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::Semicolon) {
                            parser.cursor += 1;
                        }
                        return Ok(Some(ScopeBlock::new(
                            ScopeKind::StateLet {
                                name: var_name,
                                init_expr: expr,
                            },
                            start_span,
                        )));
                    }
                }
                Ok(None)
            }
            TokenKind::KwComponent => {
                parser.cursor += 1;
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
                    let mut block = ScopeBlock::new(ScopeKind::ComponentDef { name, params }, start_span);
                    parser.parse_block_body(&mut block)?;
                    return Ok(Some(block));
                }
                Ok(None)
            }
            TokenKind::KwFor => {
                parser.cursor += 1;
                if let Some(Token {
                    kind: TokenKind::Ident(item_var),
                    ..
                }) = parser.tokens.get(parser.cursor)
                {
                    let item_var = item_var.clone();
                    parser.cursor += 1;
                    if parser.tokens.get(parser.cursor).map(|t| &t.kind) == Some(&TokenKind::KwIn) {
                        parser.cursor += 1;
                        if let Some(Token {
                            kind: TokenKind::Ident(iterable),
                            ..
                        }) = parser.tokens.get(parser.cursor)
                        {
                            let iterable = iterable.clone();
                            parser.cursor += 1;
                            let mut block = ScopeBlock::new(
                                ScopeKind::ForLoop { item_var, iterable },
                                start_span,
                            );
                            parser.parse_block_body(&mut block)?;
                            return Ok(Some(block));
                        }
                    }
                }
                Ok(None)
            }
            TokenKind::KwIf => {
                parser.cursor += 1;
                let mut condition = String::new();
                while let Some(t) = parser.tokens.get(parser.cursor) {
                    if t.kind == TokenKind::OpenBrace || t.kind == TokenKind::Eof {
                        break;
                    }
                    if let TokenKind::Ident(id) = &t.kind {
                        condition.push_str(id);
                    }
                    parser.cursor += 1;
                }
                let mut block = ScopeBlock::new(ScopeKind::IfBranch { condition }, start_span);
                parser.parse_block_body(&mut block)?;
                Ok(Some(block))
            }
            TokenKind::KwElse => {
                parser.cursor += 1;
                let mut block = ScopeBlock::new(ScopeKind::ElseBranch, start_span);
                parser.parse_block_body(&mut block)?;
                Ok(Some(block))
            }
            _ => Ok(None),
        }
    }
}
