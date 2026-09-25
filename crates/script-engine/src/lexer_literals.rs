//! 表达式字面量词法分词辅助 (Expression Literal Scanner Helpers)

use crate::lexer::Lexer;
use crate::token::Token;

pub struct LiteralScanner;

impl LiteralScanner {
    pub fn scan_number(lexer: &mut Lexer) -> Token {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some(ch) = lexer.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                lexer.bump();
            } else if ch == '.' && !is_float {
                if let Some(next_ch) = lexer.chars.get(lexer.pos + 1).copied() {
                    if next_ch.is_ascii_digit() {
                        is_float = true;
                        num_str.push(ch);
                        lexer.bump();
                        continue;
                    }
                }
                break;
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(num_str.parse().unwrap_or(0.0))
        } else {
            Token::Int(num_str.parse().unwrap_or(0))
        }
    }

    pub fn scan_string(quote: char, lexer: &mut Lexer) -> Result<Token, String> {
        lexer.bump();
        let mut s = String::new();
        while let Some(ch) = lexer.bump() {
            if ch == quote {
                return Ok(Token::String(s));
            }
            s.push(ch);
        }
        Err("未闭合的字符串字面量".to_string())
    }

    pub fn scan_ident(lexer: &mut Lexer) -> Token {
        let mut ident = String::new();
        while let Some(ch) = lexer.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                lexer.bump();
            } else {
                break;
            }
        }
        Token::Ident(ident)
    }
}
