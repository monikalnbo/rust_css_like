//! 字符串模版变量插值器 (Template String Interpolator)

use crate::eval::Evaluator;
use crate::scope::ScriptScope;

pub struct Interpolator;

impl Interpolator {
    /// 字符串插值：如 "Hello $name, count: ${count * 2}"
    pub fn interpolate(template: &str, scope: &ScriptScope) -> String {
        let mut result = String::new();
        let chars: Vec<char> = template.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '$' && i + 1 < chars.len() {
                if chars[i + 1] == '{' {
                    if let Some(end) = chars[i + 2..].iter().position(|&c| c == '}') {
                        let expr_str: String = chars[i + 2..i + 2 + end].iter().collect();
                        let val = Evaluator::eval_expr(&expr_str, scope);
                        result.push_str(&val.to_display_string());
                        i += 2 + end + 1;
                        continue;
                    }
                } else {
                    let mut var_name = String::new();
                    let mut j = i + 1;
                    while j < chars.len()
                        && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '.')
                    {
                        var_name.push(chars[j]);
                        j += 1;
                    }
                    if !var_name.is_empty() {
                        let val = Evaluator::eval_expr(&var_name, scope);
                        result.push_str(&val.to_display_string());
                        i = j;
                        continue;
                    }
                }
            }
            result.push(chars[i]);
            i += 1;
        }
        result
    }
}
