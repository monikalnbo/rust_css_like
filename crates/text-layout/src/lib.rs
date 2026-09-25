//! # Text Layout
//!
//! 字符塑形、文本自动断行度量、富文本混排与 Taffy 内在尺寸闭环引擎。

pub mod paragraph;
pub mod shaper;
pub mod span;

pub use paragraph::{LayoutLine, ParagraphLayout};
pub use shaper::TextShaper;
pub use span::{RichText, TextSpan};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_wrapping_in_fixed_width_container() {
        // 长文本测试：单字符全角约 16px，200px 宽度容器单行最多容纳约 12 个字
        let long_text = "这是一段非常长的中文描述文本，用于验证在固定宽度两百像素的容器下是否能自动正确折行并撑开容器高度。";
        let font_size = 16.0;
        let max_width = 200.0;

        let layout = TextShaper::layout_text(long_text, font_size, max_width);

        // 验证确实折行成了多行
        assert!(
            layout.line_count() >= 4,
            "预期至少折成4行，实际行数: {}",
            layout.line_count()
        );
        // 验证每行宽度不超过最大宽度 (允许少量浮点公差)
        for line in &layout.lines {
            assert!(
                line.width <= max_width + 16.0,
                "行宽超出容器: {}",
                line.width
            );
        }
        // 验证总高度随着行数自适应撑开
        let expected_min_height = layout.line_count() as f32 * TextShaper::line_height(font_size);
        assert_eq!(layout.total_height, expected_min_height);
    }
}
