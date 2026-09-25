//! CSS 动态混合计算求解器 (Dynamic CSS calc() Solver)

pub struct CalcSolver;

impl CalcSolver {
    /// 求解形如 `calc(100% - 32px)` 或 `calc(50% + 10px)` 的混合算式
    pub fn solve(expr: &str, parent_ref_px: f32) -> Result<f32, String> {
        let trimmed = expr.trim();
        let inner = if let Some(stripped) = trimmed
            .strip_prefix("calc(")
            .and_then(|s| s.strip_suffix(')'))
        {
            stripped.trim()
        } else {
            trimmed
        };

        let tokens = Self::tokenize_calc(inner)?;
        Self::eval_tokens(&tokens, parent_ref_px)
    }

    fn tokenize_calc(expr: &str) -> Result<Vec<CalcToken>, String> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
                continue;
            }

            match ch {
                '+' => {
                    chars.next();
                    tokens.push(CalcToken::Plus);
                }
                '-' => {
                    chars.next();
                    tokens.push(CalcToken::Minus);
                }
                '*' => {
                    chars.next();
                    tokens.push(CalcToken::Mul);
                }
                '/' => {
                    chars.next();
                    tokens.push(CalcToken::Div);
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() || c == '.' {
                            num_str.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let val: f32 = num_str
                        .parse()
                        .map_err(|e| format!("解析数字失败: {:?}", e))?;

                    // 检查单位：% 或 px
                    if chars.peek() == Some(&'%') {
                        chars.next();
                        tokens.push(CalcToken::Percent(val));
                    } else if chars.peek() == Some(&'p') {
                        chars.next();
                        if chars.next() == Some('x') {
                            tokens.push(CalcToken::Px(val));
                        } else {
                            tokens.push(CalcToken::Px(val));
                        }
                    } else {
                        tokens.push(CalcToken::Px(val));
                    }
                }
                _ => return Err(format!("未识别的 calc 表达式字符: '{}'", ch)),
            }
        }

        Ok(tokens)
    }

    fn eval_tokens(tokens: &[CalcToken], parent_px: f32) -> Result<f32, String> {
        let mut current_val = 0.0;
        let mut current_op = CalcToken::Plus;

        for tok in tokens {
            match tok {
                CalcToken::Plus | CalcToken::Minus => {
                    current_op = tok.clone();
                }
                CalcToken::Percent(pct) => {
                    let px = parent_px * (pct / 100.0);
                    if current_op == CalcToken::Plus {
                        current_val += px;
                    } else {
                        current_val -= px;
                    }
                }
                CalcToken::Px(px) => {
                    if current_op == CalcToken::Plus {
                        current_val += px;
                    } else {
                        current_val -= px;
                    }
                }
                _ => {}
            }
        }

        Ok(current_val)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CalcToken {
    Px(f32),
    Percent(f32),
    Plus,
    Minus,
    Mul,
    Div,
}
