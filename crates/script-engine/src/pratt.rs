//! Pratt 语法分析器 (Pratt Parser with Operator Precedence)

use crate::ast::{Action, BinaryOp, Expr, UnaryOp};
use crate::token::Token;
use crate::value::ScriptValue;

pub struct PrattParser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl PrattParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.cursor).unwrap_or(&Token::Eof)
    }

    fn bump(&mut self) -> Token {
        let tok = self.peek().clone();
        self.cursor += 1;
        tok
    }

    /// 解析完整表达式
    pub fn parse_expression(&mut self, min_bp: u8) -> Result<Expr, String> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let token = self.peek();
            if *token == Token::Eof
                || *token == Token::CloseParen
                || *token == Token::Colon
                || *token == Token::Comma
            {
                break;
            }

            // 处理三元运算符 cond ? a : b
            if *token == Token::Question {
                let (lbp, rbp) = (3, 2);
                if lbp < min_bp {
                    break;
                }
                self.bump();
                let then_branch = self.parse_expression(0)?;
                if self.bump() != Token::Colon {
                    return Err("三元表达式缺少 ':' 分支".to_string());
                }
                let else_branch = self.parse_expression(rbp)?;
                lhs = Expr::Ternary(Box::new(lhs), Box::new(then_branch), Box::new(else_branch));
                continue;
            }

            // 处理后缀点访问 .field
            if *token == Token::Dot {
                self.bump();
                if let Token::Ident(field) = self.bump() {
                    lhs = Expr::Dot(Box::new(lhs), field);
                    continue;
                } else {
                    return Err("点属性访问符 '.' 后必须紧跟属性标示符".to_string());
                }
            }

            // 处理函数调用 func(arg1, arg2)
            if *token == Token::OpenParen {
                if let Expr::Ident(name) = lhs {
                    self.bump();
                    let mut args = Vec::new();
                    if *self.peek() != Token::CloseParen {
                        loop {
                            args.push(self.parse_expression(0)?);
                            if *self.peek() == Token::Comma {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    if self.bump() != Token::CloseParen {
                        return Err("函数调用参数未闭合 ')'".to_string());
                    }
                    lhs = Expr::Call(name, args);
                    continue;
                }
            }

            // 处理二元中缀运算符
            let Some((op, (lbp, rbp))) = self.infix_binding_power(token) else {
                break;
            };

            if lbp < min_bp {
                break;
            }

            self.bump();
            let rhs = self.parse_expression(rbp)?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, String> {
        let tok = self.bump();
        match tok {
            Token::Int(i) => Ok(Expr::Literal(ScriptValue::Int(i))),
            Token::Float(f) => Ok(Expr::Literal(ScriptValue::Float(f))),
            Token::String(s) => Ok(Expr::Literal(ScriptValue::String(s))),
            Token::ColorHex(hex) => Ok(Expr::Literal(ScriptValue::String(hex))),
            Token::Ident(name) => match name.as_str() {
                "true" => Ok(Expr::Literal(ScriptValue::Bool(true))),
                "false" => Ok(Expr::Literal(ScriptValue::Bool(false))),
                "null" => Ok(Expr::Literal(ScriptValue::Null)),
                _ => Ok(Expr::Ident(name)),
            },
            Token::Not => {
                let inner = self.parse_expression(17)?;
                Ok(Expr::Unary(UnaryOp::Not, Box::new(inner)))
            }
            Token::Minus => {
                let inner = self.parse_expression(17)?;
                Ok(Expr::Unary(UnaryOp::Neg, Box::new(inner)))
            }
            Token::OpenParen => {
                let expr = self.parse_expression(0)?;
                if self.bump() != Token::CloseParen {
                    return Err("表达式中括号未闭合 ')'".to_string());
                }
                Ok(expr)
            }
            _ => Err(format!("非法的表达式起始 Token: {:?}", tok)),
        }
    }

    fn infix_binding_power(&self, tok: &Token) -> Option<(BinaryOp, (u8, u8))> {
        match tok {
            Token::Or => Some((BinaryOp::Or, (3, 4))),
            Token::And => Some((BinaryOp::And, (5, 6))),
            Token::Eq => Some((BinaryOp::Eq, (7, 8))),
            Token::Ne => Some((BinaryOp::Ne, (7, 8))),
            Token::Lt => Some((BinaryOp::Lt, (9, 10))),
            Token::Le => Some((BinaryOp::Le, (9, 10))),
            Token::Gt => Some((BinaryOp::Gt, (9, 10))),
            Token::Ge => Some((BinaryOp::Ge, (9, 10))),
            Token::Plus => Some((BinaryOp::Add, (11, 12))),
            Token::Minus => Some((BinaryOp::Sub, (11, 12))),
            Token::Star => Some((BinaryOp::Mul, (13, 14))),
            Token::Slash => Some((BinaryOp::Div, (13, 14))),
            Token::Percent => Some((BinaryOp::Mod, (13, 14))),
            _ => None,
        }
    }

    /// 解析动作流语句 (-> count += 1 或 is_dark = !is_dark)
    pub fn parse_action(&mut self) -> Result<Action, String> {
        if *self.peek() == Token::Arrow {
            self.bump();
        }

        if let Token::Ident(target) = self.peek().clone() {
            // 查看后续是否是赋值符
            if self.tokens.get(self.cursor + 1) == Some(&Token::Assign) {
                self.bump();
                self.bump();
                let expr = self.parse_expression(0)?;
                return Ok(Action::Assign(target, expr));
            } else if self.tokens.get(self.cursor + 1) == Some(&Token::PlusAssign) {
                self.bump();
                self.bump();
                let expr = self.parse_expression(0)?;
                return Ok(Action::CompoundAssign(target, BinaryOp::Add, expr));
            } else if self.tokens.get(self.cursor + 1) == Some(&Token::MinusAssign) {
                self.bump();
                self.bump();
                let expr = self.parse_expression(0)?;
                return Ok(Action::CompoundAssign(target, BinaryOp::Sub, expr));
            }
        }

        let expr = self.parse_expression(0)?;
        Ok(Action::Expr(expr))
    }
}
