//! 词法分词扫描辅助工具 (Lexer Scanning Helpers)

use crate::lexer::Lexer;
use crate::token::TokenKind;

pub struct ScanHelpers;

impl ScanHelpers {
    pub fn scan_hex_color(lexer: &mut Lexer) -> TokenKind {
        lexer.bump();
        let mut hex = String::from("#");
        while let Some(c) = lexer.peek() {
            if c.is_ascii_hexdigit() {
                hex.push(c);
                lexer.bump();
            } else {
                break;
            }
        }
        TokenKind::HexColor(hex)
    }

    pub fn scan_string(quote: char, lexer: &mut Lexer) -> TokenKind {
        lexer.bump();
        let mut s = String::new();
        while let Some((_, c)) = lexer.bump() {
            if c == quote {
                break;
            }
            s.push(c);
        }
        TokenKind::StringLiteral(s)
    }

    pub fn scan_variable(lexer: &mut Lexer) -> TokenKind {
        lexer.bump();
        let mut var = String::new();
        while let Some(c) = lexer.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
                var.push(c);
                lexer.bump();
            } else {
                break;
            }
        }
        TokenKind::Variable(var)
    }

    pub fn scan_ident(lexer: &mut Lexer) -> TokenKind {
        let mut ident = String::new();
        while let Some(c) = lexer.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                ident.push(c);
                lexer.bump();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "component" => TokenKind::KwComponent,
            "for" => TokenKind::KwFor,
            "in" => TokenKind::KwIn,
            "if" => TokenKind::KwIf,
            "else" => TokenKind::KwElse,
            "let" => TokenKind::KwLet,
            _ => TokenKind::Ident(ident),
        }
    }

    pub fn scan_number(lexer: &mut Lexer) -> TokenKind {
        let mut num_str = String::new();
        while let Some(c) = lexer.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                lexer.bump();
            } else {
                break;
            }
        }
        let val = num_str.parse::<f32>().unwrap_or(0.0);
        TokenKind::Number(val)
    }
}
