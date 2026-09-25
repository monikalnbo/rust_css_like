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

    pub(crate) fn peek(&self) -> &TokenKind {
        self.tokens
            .get(self.cursor)
            .map(|t| &t.kind)
            .unwrap_or(&TokenKind::Eof)
    }

    pub(crate) fn bump(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.cursor);
        if self.cursor < self.tokens.len() {
            self.cursor += 1;
        }
        t
    }

    pub(crate) fn skip_newlines(&mut self) {
        while self.peek() == &TokenKind::Newline {
            self.bump();
        }
    }

    /// 解析顶层所有作用域块
    pub fn parse(&mut self) -> Result<Vec<ScopeBlock>, String> {
        let mut blocks = Vec::new();
        self.skip_newlines();
        while self.peek() != &TokenKind::Eof {
            if let Some(block) = self.parse_statement()? {
                blocks.push(block);
            } else {
                // 如果遇到未能识别的 token，消费之以防死循环
                self.bump();
            }
            self.skip_newlines();
        }
        Ok(blocks)
    }

    /// 解析语句（控制流、声明或元素语句）
    pub fn parse_statement(&mut self) -> Result<Option<ScopeBlock>, String> {
        self.skip_newlines();
        if self.peek() == &TokenKind::Eof || self.peek() == &TokenKind::CloseBrace {
            return Ok(None);
        }

        if let Some(b) = DeclParser::try_parse_decl(self)? {
            self.skip_newlines();
            return Ok(Some(b));
        }

        self.parse_element_statement()
    }

    /// 解析 UI 元素语句（支持块级与行内叶子控件、属性与动作直连）
    pub fn parse_element_statement(&mut self) -> Result<Option<ScopeBlock>, String> {
        self.skip_newlines();
        let start_span = self
            .tokens
            .get(self.cursor)
            .map(|t| t.span)
            .unwrap_or_default();

        // 1. 确定标签与 ScopeKind
        let (tag_name, kind) = match self.peek() {
            TokenKind::Ident(name) => {
                let name = name.clone();
                let k = match name.as_str() {
                    "window" | "win" => ScopeKind::Window,
                    "span" => ScopeKind::Span,
                    "theme" => ScopeKind::Theme,
                    other => ScopeKind::Element(other.to_string()),
                };
                self.bump();
                (name, k)
            }
            TokenKind::Colon => {
                self.bump(); // 消耗 ':'
                if let TokenKind::Ident(pseudo) = self.peek().clone() {
                    self.bump();
                    let mut block = ScopeBlock::new(ScopeKind::Pseudo(pseudo), start_span);
                    self.skip_newlines();
                    self.parse_block_body(&mut block)?;
                    return Ok(Some(block));
                } else {
                    return Ok(None);
                }
            }
            TokenKind::Spread => {
                self.bump(); // 消耗 '...'
                if let TokenKind::Ident(mixin) = self.peek().clone() {
                    self.bump();
                    if self.peek() == &TokenKind::Semicolon || self.peek() == &TokenKind::Newline {
                        self.bump();
                    }
                    return Ok(Some(ScopeBlock::new(ScopeKind::MixinSpread(mixin), start_span)));
                } else {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        };

        let mut block = ScopeBlock::new(kind, start_span);

        // 2. 检查组件实参调用语法: StatCard(title="Active Users", value="12,480")
        if self.peek() == &TokenKind::OpenParen {
            if let Some(Token { kind: TokenKind::Ident(_), .. }) = self.tokens.get(self.cursor + 1) {
                if let Some(Token { kind: TokenKind::Equals, .. }) = self.tokens.get(self.cursor + 2) {
                    self.bump(); // 消耗 '('
                    while self.peek() != &TokenKind::CloseParen && self.peek() != &TokenKind::Eof {
                        self.skip_newlines();
                        if let TokenKind::Ident(key) = self.peek().clone() {
                            self.bump();
                            if self.peek() == &TokenKind::Equals {
                                self.bump();
                                let val = self.parse_value()?;
                                block.properties.push(PropertyDecl {
                                    name: key,
                                    value: val,
                                    is_important: false,
                                    span: start_span,
                                });
                            }
                        }
                        if self.peek() == &TokenKind::Comma {
                            self.bump();
                        } else if self.peek() != &TokenKind::CloseParen {
                            self.bump();
                        }
                    }
                    if self.peek() == &TokenKind::CloseParen {
                        self.bump();
                    }
                    self.skip_newlines();
                    if self.peek() == &TokenKind::OpenBrace {
                        self.parse_block_body(&mut block)?;
                    } else if self.peek() == &TokenKind::Semicolon || self.peek() == &TokenKind::Newline {
                        self.bump();
                    }
                    return Ok(Some(block));
                }
            }
        }

        // 3. 可选字符串 / 标题 / 文本 / 变量 / 括号表达式
        match self.peek() {
            TokenKind::StringLiteral(s) => {
                let s = s.clone();
                self.bump();
                let prop_name = if tag_name == "win" || tag_name == "window" {
                    "title"
                } else if tag_name == "plugin" {
                    "name"
                } else {
                    "text"
                };
                block.properties.push(PropertyDecl {
                    name: prop_name.to_string(),
                    value: Value::String(s),
                    is_important: false,
                    span: start_span,
                });
            }
            TokenKind::Variable(v) => {
                let v = v.clone();
                self.bump();
                let prop_name = if tag_name == "win" || tag_name == "window" {
                    "title"
                } else {
                    "text"
                };
                block.properties.push(PropertyDecl {
                    name: prop_name.to_string(),
                    value: Value::Variable(v),
                    is_important: false,
                    span: start_span,
                });
            }
            TokenKind::OpenParen => {
                // 检查是否为 (width, height) 尺寸二元组
                if let (Some(Token { kind: TokenKind::Number(w), .. }),
                        Some(Token { kind: TokenKind::Comma, .. }),
                        Some(Token { kind: TokenKind::Number(h), .. }),
                        Some(Token { kind: TokenKind::CloseParen, .. })) =
                    (self.tokens.get(self.cursor + 1),
                     self.tokens.get(self.cursor + 2),
                     self.tokens.get(self.cursor + 3),
                     self.tokens.get(self.cursor + 4))
                {
                    let w = *w;
                    let h = *h;
                    self.cursor += 5; // 消耗 '(', w, ',', h, ')'
                    block.properties.push(PropertyDecl {
                        name: "width".to_string(),
                        value: Value::Number(w),
                        is_important: false,
                        span: start_span,
                    });
                    block.properties.push(PropertyDecl {
                        name: "height".to_string(),
                        value: Value::Number(h),
                        is_important: false,
                        span: start_span,
                    });
                } else {
                    // 动态括号条件表达式，如 btn (is_dark ? ...)
                    let expr = self.parse_parenthesized_expression()?;
                    block.properties.push(PropertyDecl {
                        name: "text".to_string(),
                        value: Value::Expression(expr),
                        is_important: false,
                        span: start_span,
                    });
                }
            }
            _ => {}
        }

        // 4. 紧随字符串后的尺寸二元组，如 win "Title" (400, 300)
        if self.peek() == &TokenKind::OpenParen {
            if let (Some(Token { kind: TokenKind::Number(w), .. }),
                    Some(Token { kind: TokenKind::Comma, .. }),
                    Some(Token { kind: TokenKind::Number(h), .. }),
                    Some(Token { kind: TokenKind::CloseParen, .. })) =
                (self.tokens.get(self.cursor + 1),
                 self.tokens.get(self.cursor + 2),
                 self.tokens.get(self.cursor + 3),
                 self.tokens.get(self.cursor + 4))
            {
                let w = *w;
                let h = *h;
                self.cursor += 5; // 消耗 '(', w, ',', h, ')'
                block.properties.push(PropertyDecl {
                    name: "width".to_string(),
                    value: Value::Number(w),
                    is_important: false,
                    span: start_span,
                });
                block.properties.push(PropertyDecl {
                    name: "height".to_string(),
                    value: Value::Number(h),
                    is_important: false,
                    span: start_span,
                });
            }
        }

        // 5. 解析同一行内的内联属性、缩写样式与动作流 ->
        while self.peek() != &TokenKind::OpenBrace
            && self.peek() != &TokenKind::Newline
            && self.peek() != &TokenKind::Semicolon
            && self.peek() != &TokenKind::CloseBrace
            && self.peek() != &TokenKind::Eof
        {
            // 动作流绑定: -> count += 1 或 -> { ... }
            if self.peek() == &TokenKind::Arrow {
                self.bump(); // 消耗 '->'
                self.skip_newlines();
                if self.peek() == &TokenKind::OpenBrace {
                    self.bump(); // 消耗 '{'
                    let mut action_code = String::new();
                    let mut brace_depth = 1;
                    while brace_depth > 0 && self.peek() != &TokenKind::Eof {
                        if self.peek() == &TokenKind::OpenBrace {
                            brace_depth += 1;
                        } else if self.peek() == &TokenKind::CloseBrace {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                self.bump();
                                break;
                            }
                        }
                        crate::decl_parser::format_token_to_expr(self.peek(), &mut action_code);
                        self.bump();
                    }
                    block.properties.push(PropertyDecl {
                        name: "on_click".to_string(),
                        value: Value::ActionCode(action_code.trim().to_string()),
                        is_important: false,
                        span: block.span,
                    });
                } else {
                    let mut action_code = String::new();
                    while self.peek() != &TokenKind::Newline
                        && self.peek() != &TokenKind::Semicolon
                        && self.peek() != &TokenKind::CloseBrace
                        && self.peek() != &TokenKind::Eof
                    {
                        crate::decl_parser::format_token_to_expr(self.peek(), &mut action_code);
                        self.bump();
                    }
                    block.properties.push(PropertyDecl {
                        name: "on_click".to_string(),
                        value: Value::ActionCode(action_code.trim().to_string()),
                        is_important: false,
                        span: block.span,
                    });
                }
                continue;
            }

            // 具名属性: key=val 或 key: val
            if let TokenKind::Ident(prop_name) = self.peek().clone() {
                if self.tokens.get(self.cursor + 1).map(|t| &t.kind) == Some(&TokenKind::Equals)
                    || self.tokens.get(self.cursor + 1).map(|t| &t.kind) == Some(&TokenKind::Colon)
                {
                    self.bump(); // 消耗 prop_name
                    self.bump(); // 消耗 '=' 或 ':'
                    let val = self.parse_value()?;
                    block.properties.push(PropertyDecl {
                        name: prop_name,
                        value: val,
                        is_important: false,
                        span: start_span,
                    });
                    continue;
                }
            }

            // 缩写与叶子声明
            match self.peek().clone() {
                TokenKind::HexColor(h) => {
                    self.bump();
                    let prop_name = if tag_name == "txt" || tag_name == "text" {
                        "color"
                    } else {
                        "bg"
                    };
                    block.properties.push(PropertyDecl {
                        name: prop_name.to_string(),
                        value: Value::HexColor(h),
                        is_important: false,
                        span: start_span,
                    });
                }
                TokenKind::Dimension(n, unit) => {
                    self.bump();
                    let val_str = format!(
                        "{}{}",
                        if n.fract() == 0.0 {
                            (n as i64).to_string()
                        } else {
                            n.to_string()
                        },
                        unit
                    );
                    let prop_name = if tag_name == "txt" || tag_name == "text" {
                        "font_size"
                    } else {
                        "height"
                    };
                    block.properties.push(PropertyDecl {
                        name: prop_name.to_string(),
                        value: Value::String(val_str),
                        is_important: false,
                        span: start_span,
                    });
                }
                TokenKind::Ident(id) => {
                    self.bump();
                    if id == "bold" || id == "italic" || id == "normal" {
                        block.properties.push(PropertyDecl {
                            name: "font_weight".to_string(),
                            value: Value::Ident(id),
                            is_important: false,
                            span: start_span,
                        });
                    } else if (tag_name == "txt" || tag_name == "text")
                        && !block.properties.iter().any(|p| p.name == "text")
                    {
                        // 表达式如 txt item.title
                        let mut expr = id;
                        while self.peek() == &TokenKind::Dot {
                            self.bump(); // 消耗 '.'
                            if let TokenKind::Ident(field) = self.peek().clone() {
                                self.bump();
                                expr.push('.');
                                expr.push_str(&field);
                            }
                        }
                        block.properties.push(PropertyDecl {
                            name: "text".to_string(),
                            value: Value::Ident(expr),
                            is_important: false,
                            span: start_span,
                        });
                    } else {
                        block.properties.push(PropertyDecl {
                            name: id.clone(),
                            value: Value::Ident(id),
                            is_important: false,
                            span: start_span,
                        });
                    }
                }
                _ => {
                    self.bump();
                }
            }
        }

        // 6. 检查是否有子元素块 { ... } 或是单行叶子控件
        self.skip_newlines();
        if self.peek() == &TokenKind::OpenBrace {
            self.parse_block_body(&mut block)?;
        } else if self.peek() == &TokenKind::Semicolon || self.peek() == &TokenKind::Newline {
            self.bump();
        }

        Ok(Some(block))
    }

    /// 解析属性值
    pub fn parse_value(&mut self) -> Result<Value, String> {
        let val = match self.peek() {
            TokenKind::StringLiteral(s) => {
                let v = Value::String(s.clone());
                self.bump();
                v
            }
            TokenKind::Number(n) => {
                let v = Value::Number(*n);
                self.bump();
                v
            }
            TokenKind::Dimension(n, u) => {
                let s = format!(
                    "{}{}",
                    if n.fract() == 0.0 {
                        (*n as i64).to_string()
                    } else {
                        n.to_string()
                    },
                    u
                );
                self.bump();
                Value::String(s)
            }
            TokenKind::HexColor(h) => {
                let v = Value::HexColor(h.clone());
                self.bump();
                v
            }
            TokenKind::Variable(v) => {
                let val = Value::Variable(v.clone());
                self.bump();
                val
            }
            TokenKind::Ident(id) => {
                let val = Value::Ident(id.clone());
                self.bump();
                val
            }
            TokenKind::OpenParen => {
                let expr = self.parse_parenthesized_expression()?;
                Value::Expression(expr)
            }
            _ => {
                self.bump();
                Value::String(String::new())
            }
        };
        Ok(val)
    }

    /// 解析括号表达式如 (8, 16) 或 (is_dark ? #0f172a : #f8fafc)
    fn parse_parenthesized_expression(&mut self) -> Result<String, String> {
        if self.peek() != &TokenKind::OpenParen {
            return Ok(String::new());
        }
        self.bump(); // 消耗 '('
        let mut expr = String::from("(");
        let mut paren_depth = 1;
        while paren_depth > 0 && self.peek() != &TokenKind::Eof {
            if self.peek() == &TokenKind::OpenParen {
                paren_depth += 1;
            } else if self.peek() == &TokenKind::CloseParen {
                paren_depth -= 1;
                if paren_depth == 0 {
                    self.bump();
                    expr.push(')');
                    break;
                }
            }
            crate::decl_parser::format_token_to_expr(self.peek(), &mut expr);
            self.bump();
        }
        Ok(expr)
    }

    /// 解析 `{ ... }` 内部属性与子块
    pub(crate) fn parse_block_body(&mut self, block: &mut ScopeBlock) -> Result<(), String> {
        if self.peek() != &TokenKind::OpenBrace {
            return Ok(());
        }
        self.bump(); // 消耗 `{`

        while self.peek() != &TokenKind::CloseBrace && self.peek() != &TokenKind::Eof {
            self.skip_newlines();
            if self.peek() == &TokenKind::CloseBrace || self.peek() == &TokenKind::Eof {
                break;
            }

            if let Some(child) = self.parse_statement()? {
                block.children.push(child);
                continue;
            }

            if self.peek() == &TokenKind::Arrow {
                self.parse_action_binding(block);
                continue;
            }

            if let TokenKind::Ident(prop_name) = self.peek().clone() {
                if self.tokens.get(self.cursor + 1).map(|t| &t.kind) == Some(&TokenKind::Colon)
                    || self.tokens.get(self.cursor + 1).map(|t| &t.kind) == Some(&TokenKind::Equals)
                {
                    self.parse_property_decl(&prop_name, block);
                    continue;
                }
            }

            self.bump();
        }

        if self.peek() == &TokenKind::CloseBrace {
            self.bump(); // 消耗 `}`
        }
        self.skip_newlines();

        Ok(())
    }

    fn parse_action_binding(&mut self, block: &mut ScopeBlock) {
        self.bump(); // 消耗 '->'
        self.skip_newlines();
        if self.peek() == &TokenKind::OpenBrace {
            self.bump(); // 消耗 '{'
            let mut action_code = String::new();
            let mut brace_depth = 1;
            while brace_depth > 0 && self.peek() != &TokenKind::Eof {
                if self.peek() == &TokenKind::OpenBrace {
                    brace_depth += 1;
                } else if self.peek() == &TokenKind::CloseBrace {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        self.bump();
                        break;
                    }
                }
                crate::decl_parser::format_token_to_expr(self.peek(), &mut action_code);
                self.bump();
            }
            block.properties.push(PropertyDecl {
                name: "on_click".to_string(),
                value: Value::ActionCode(action_code.trim().to_string()),
                is_important: false,
                span: block.span,
            });
        } else {
            let mut action_code = String::new();
            while self.peek() != &TokenKind::Semicolon
                && self.peek() != &TokenKind::Newline
                && self.peek() != &TokenKind::CloseBrace
                && self.peek() != &TokenKind::Eof
            {
                crate::decl_parser::format_token_to_expr(self.peek(), &mut action_code);
                self.bump();
            }
            if self.peek() == &TokenKind::Semicolon || self.peek() == &TokenKind::Newline {
                self.bump();
            }
            block.properties.push(PropertyDecl {
                name: "on_click".to_string(),
                value: Value::ActionCode(action_code.trim().to_string()),
                is_important: false,
                span: block.span,
            });
        }
    }

    fn parse_property_decl(&mut self, prop_name: &str, block: &mut ScopeBlock) {
        let prop_name = prop_name.to_string();
        let prop_span = self
            .tokens
            .get(self.cursor)
            .map(|t| t.span)
            .unwrap_or_default();
        self.bump(); // 消耗 prop_name

        if self.peek() == &TokenKind::Colon || self.peek() == &TokenKind::Equals {
            self.bump(); // 消耗 ':' 或 '='
            if let Ok(val) = self.parse_value() {
                if self.peek() == &TokenKind::Semicolon || self.peek() == &TokenKind::Newline {
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
}
