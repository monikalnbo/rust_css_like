//! 流式字符分词器 (Lexer with Control Flow & Operators)

use crate::scan_helpers::ScanHelpers;
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    _source: std::marker::PhantomData<&'a str>,
    chars: Vec<(usize, char)>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            _source: std::marker::PhantomData,
            chars: source.char_indices().collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.get(self.cursor).map(|&(_, c)| c)
    }

    pub(crate) fn peek_next(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).map(|&(_, c)| c)
    }

    pub(crate) fn bump(&mut self) -> Option<(usize, char)> {
        let item = self.chars.get(self.cursor).copied();
        if let Some((_, ch)) = item {
            self.cursor += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        item
    }


    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
                continue;
            }
            if ch == '/' && self.peek_next() == Some('/') {
                while let Some((_, c)) = self.bump() {
                    if c == '\n' {
                        break;
                    }
                }
                continue;
            }

            let start = self.cursor;
            let line = self.line;
            let col = self.column;

            let kind = match ch {
                '{' => {
                    self.bump();
                    TokenKind::OpenBrace
                }
                '}' => {
                    self.bump();
                    TokenKind::CloseBrace
                }
                '(' => {
                    self.bump();
                    TokenKind::OpenParen
                }
                ')' => {
                    self.bump();
                    TokenKind::CloseParen
                }
                ':' => {
                    self.bump();
                    TokenKind::Colon
                }
                ';' => {
                    self.bump();
                    TokenKind::Semicolon
                }
                ',' => {
                    self.bump();
                    TokenKind::Comma
                }
                '?' => {
                    self.bump();
                    TokenKind::Question
                }
                '!' => {
                    self.bump();
                    TokenKind::Exclamation
                }
                '+' => {
                    self.bump();
                    TokenKind::Plus
                }
                '*' => {
                    self.bump();
                    TokenKind::Star
                }
                '/' => {
                    self.bump();
                    TokenKind::Slash
                }
                '=' => {
                    self.bump();
                    TokenKind::Equals
                }
                '-' => {
                    self.bump();
                    if self.peek() == Some('>') {
                        self.bump();
                        TokenKind::Arrow
                    } else {
                        TokenKind::Minus
                    }
                }
                '@' => {
                    self.bump();
                    let mut ident = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_alphabetic() {
                            ident.push(c);
                            self.bump();
                        } else {
                            break;
                        }
                    }
                    if ident == "import" {
                        TokenKind::KwImport
                    } else {
                        TokenKind::Ident(format!("@{}", ident))
                    }
                }
                '.' => {
                    self.bump();
                    if self.peek() == Some('.') && self.peek_next() == Some('.') {
                        self.bump();
                        self.bump();
                        TokenKind::Spread
                    } else {
                        TokenKind::Dot
                    }
                }
                '#' => ScanHelpers::scan_hex_color(self),
                '"' | '\'' => ScanHelpers::scan_string(ch, self),
                '$' => ScanHelpers::scan_variable(self),
                _ if ch.is_alphabetic() || ch == '_' => {
                    ScanHelpers::scan_ident(self)
                }
                _ if ch.is_ascii_digit() => {
                    ScanHelpers::scan_number(self)
                }
                _ => {
                    self.bump();
                    continue;
                }
            };

            let end = self.cursor;
            tokens.push(Token::new(kind, Span::new(start, end, line, col)));
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            Span::new(self.cursor, self.cursor, self.line, self.column),
        ));
        Ok(tokens)
    }
}
