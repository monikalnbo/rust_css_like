//! Unicode 字素族与中日韩 (CJK) 全角/半角排版宽度计算

pub struct UnicodeMetrics;

impl UnicodeMetrics {
    /// 计算字符串在终端或等宽排版时的显示列宽
    ///
    /// 中日韩统一汉字 (CJK) 全角字符占 2 宽，ASCII 英文半角占 1 宽
    pub fn display_width(text: &str) -> usize {
        let mut width = 0;
        for ch in text.chars() {
            if Self::is_cjk(ch) {
                width += 2;
            } else {
                width += 1;
            }
        }
        width
    }

    /// 判断字符是否属于 CJK 中日韩汉字及标点区间
    pub fn is_cjk(ch: char) -> bool {
        matches!(ch,
            '\u{4E00}'..='\u{9FFF}'   // CJK 统一表意文字基本区
            | '\u{3400}'..='\u{4DBF}' // CJK 扩展 A
            | '\u{20000}'..='\u{2A6DF}' // CJK 扩展 B
            | '\u{F900}'..='\u{FAFF}' // CJK 兼容表意文字
            | '\u{3000}'..='\u{303F}' // CJK 标点符号 (如 "，"、"。"、"【"、"】")
            | '\u{FF01}'..='\u{FF60}' // 全角 ASCII 变体
        )
    }
}
