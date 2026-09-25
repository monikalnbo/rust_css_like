//! 表达式分词词法分析器实现 (Expression Lexer Scanner)

use crate::lexer_literals::LiteralScanner;
use crate::token::Token;

pub struct Lexer<'a> {
    pub(crate) chars: Vec<char>,
    pub(crate) pos: usize,
    _phantom: std::marker::PhantomData<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
                continue;
            }

            match ch {
                '#' => tokens.push(self.lex_color_hex()),
                '0'..='9' => tokens.push(LiteralScanner::scan_number(self)),
                '"' | '\'' => tokens.push(LiteralScanner::scan_string(ch, self)?),
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(LiteralScanner::scan_ident(self))
                }
                '-' => tokens.push(self.lex_minus_or_arrow()),
                '+' => tokens.push(self.lex_plus_or_assign()),
                '*' => {
                    self.bump();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.bump();
                    tokens.push(Token::Slash);
                }
                '%' => {
                    self.bump();
                    tokens.push(Token::Percent);
                }
                '=' => tokens.push(self.lex_assign_or_eq()),
                '!' => tokens.push(self.lex_not_or_ne()),
                '<' => tokens.push(self.lex_lt_or_le()),
                '>' => tokens.push(self.lex_gt_or_ge()),
                '&' => tokens.push(self.lex_logical_and()?),
                '|' => tokens.push(self.lex_logical_or()?),
                '?' => {
                    self.bump();
                    tokens.push(Token::Question);
                }
                ':' => {
                    self.bump();
                    tokens.push(Token::Colon);
                }
                '.' => {
                    self.bump();
                    tokens.push(Token::Dot);
                }
                '(' => {
                    self.bump();
                    tokens.push(Token::OpenParen);
                }
                ')' => {
                    self.bump();
                    tokens.push(Token::CloseParen);
                }
                ',' => {
                    self.bump();
                    tokens.push(Token::Comma);
                }
                _ => return Err(format!("未识别的表达式字符: '{}'", ch)),
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn lex_color_hex(&mut self) -> Token {
        self.bump();
        let mut hex = String::from("#");
        while let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() {
                hex.push(c);
                self.bump();
            } else {
                break;
            }
        }
        Token::ColorHex(hex)
    }

    fn lex_minus_or_arrow(&mut self) -> Token {
        self.bump();
        if self.peek() == Some('>') {
            self.bump();
            Token::Arrow
        } else if self.peek() == Some('=') {
            self.bump();
            Token::MinusAssign
        } else {
            Token::Minus
        }
    }

    fn lex_plus_or_assign(&mut self) -> Token {
        self.bump();
        if self.peek() == Some('=') {
            self.bump();
            Token::PlusAssign
        } else {
            Token::Plus
        }
    }

    fn lex_assign_or_eq(&mut self) -> Token {
        self.bump();
        if self.peek() == Some('=') {
            self.bump();
            Token::Eq
        } else {
            Token::Assign
        }
    }

    fn lex_not_or_ne(&mut self) -> Token {
        self.bump();
        if self.peek() == Some('=') {
            self.bump();
            Token::Ne
        } else {
            Token::Not
        }
    }

    fn lex_lt_or_le(&mut self) -> Token {
        self.bump();
        if self.peek() == Some('=') {
            self.bump();
            Token::Le
        } else {
            Token::Lt
        }
    }

    fn lex_gt_or_ge(&mut self) -> Token {
        self.bump();
        if self.peek() == Some('=') {
            self.bump();
            Token::Ge
        } else {
            Token::Gt
        }
    }

    fn lex_logical_and(&mut self) -> Result<Token, String> {
        self.bump();
        if self.peek() == Some('&') {
            self.bump();
            Ok(Token::And)
        } else {
            Err("未识别的操作符 &，仅支持逻辑与 &&".to_string())
        }
    }

    fn lex_logical_or(&mut self) -> Result<Token, String> {
        self.bump();
        if self.peek() == Some('|') {
            self.bump();
            Ok(Token::Or)
        } else {
            Err("未识别的操作符 |，仅支持逻辑或 ||".to_string())
        }
    }
}
