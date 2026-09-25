//! 增量容错语法解析器 (Parser with Full Control Flow & Components)

use crate::ast::{PropertyDecl, ScopeBlock, ScopeKind, Value};
use crate::decl_parser::DeclParser;
use crate::token::{Token, TokenKind};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn peek(&self) -> &TokenKind {
        self.tokens
            .get(self.cursor)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof)
    }

    fn bump(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.cursor);
        if self.cursor < self.tokens.len() {
            self.cursor += 1;
        }
        t
    }

    /// 解析顶层所有作用域块
    pub fn parse(&mut self) -> Result<Vec<ScopeBlock>, String> {
        let mut blocks = Vec::new();
        while self.peek() != &TokenKind::Eof {
            if let Some(block) = self.parse_statement()? {
                blocks.push(block);
            } else {
                break;
            }
        }
        Ok(blocks)
    }

    /// 解析顶级语句
    fn parse_statement(&mut self) -> Result<Option<ScopeBlock>, String> {
        if let Some(b) = DeclParser::try_parse_decl(self)? {
            return Ok(Some(b));
        }

        self.parse_element_block()
    }

    /// 解析常规 UI 元素块
    fn parse_element_block(&mut self) -> Result<Option<ScopeBlock>, String> {
        let start_span = self
            .tokens
            .get(self.cursor)
            .map(|t| t.span)
            .unwrap_or_default();

        let kind = match self.peek() {
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.bump();
                match name.as_str() {
                    "window" | "win" => ScopeKind::Window,
                    "span" => ScopeKind::Span,
                    "theme" => ScopeKind::Theme,
                    other => ScopeKind::Element(other.to_string()),
                }
            }
            TokenKind::Colon => {
                self.bump();
                if let TokenKind::Ident(pseudo) = self.peek() {
                    let pseudo = pseudo.clone();
                    self.bump();
                    ScopeKind::Pseudo(pseudo)
                } else {
                    return Ok(None);
                }
            }
            TokenKind::Eof | TokenKind::CloseBrace => return Ok(None),
            _ => {
                self.bump();
                return Ok(None);
            }
        };

        // 必须紧跟 `{`
        if self.peek() != &TokenKind::OpenBrace {
            return Ok(None);
        }

        let mut block = ScopeBlock::new(kind, start_span);
        self.parse_block_body(&mut block)?;
        Ok(Some(block))
    }

    /// 解析 `{ ... }` 内部属性与子块
    pub(crate) fn parse_block_body(&mut self, block: &mut ScopeBlock) -> Result<(), String> {
        if self.peek() != &TokenKind::OpenBrace {
            return Ok(());
        }
        self.bump(); // 消耗 `{`

        while self.peek() != &TokenKind::CloseBrace && self.peek() != &TokenKind::Eof {
            if let Some(child) = self.parse_statement()? {
                block.children.push(child);
                continue;
            }

            if self.peek() == &TokenKind::Arrow {
                self.parse_action_binding(block);
                continue;
            }

            if let TokenKind::Ident(prop_name) = self.peek().clone() {
                self.parse_property_decl(&prop_name, block);
            } else {
                self.bump();
            }
        }

        if self.peek() == &TokenKind::CloseBrace {
            self.bump(); // 消耗 `}`
        }

        Ok(())
    }

    fn parse_action_binding(&mut self, block: &mut ScopeBlock) {
        self.bump();
        let mut action_code = String::new();
        while self.peek() != &TokenKind::Semicolon
            && self.peek() != &TokenKind::CloseBrace
            && self.peek() != &TokenKind::Eof
        {
            match self.peek() {
                TokenKind::Ident(id) => action_code.push_str(id),
                TokenKind::Plus => action_code.push('+'),
                TokenKind::Equals => action_code.push('='),
                _ => {}
            }
            self.bump();
        }
        block.properties.push(PropertyDecl {
            name: "on_click".to_string(),
            value: Value::ActionCode(action_code),
            is_important: false,
            span: block.span,
        });
    }

    fn parse_property_decl(&mut self, prop_name: &str, block: &mut ScopeBlock) {
        let prop_name = prop_name.to_string();
        let prop_span = self
            .tokens
            .get(self.cursor)
            .map(|t| t.span)
            .unwrap_or_default();
        self.bump();

        if self.peek() == &TokenKind::Colon || self.peek() == &TokenKind::Equals {
            self.bump();
            let val = match self.peek() {
                TokenKind::StringLiteral(s) => Value::String(s.clone()),
                TokenKind::HexColor(h) => Value::HexColor(h.clone()),
                TokenKind::Number(n) => Value::Number(*n),
                TokenKind::Variable(v) => Value::Variable(v.clone()),
                TokenKind::Ident(id) => Value::Ident(id.clone()),
                _ => Value::String(String::new()),
            };
            self.bump();

            if self.peek() == &TokenKind::Semicolon {
                self.bump();
            }

            block.properties.push(PropertyDecl {
                name: prop_name,
                value: val,
                is_important: false,
                span: prop_span,
            });
        }
    }
}
