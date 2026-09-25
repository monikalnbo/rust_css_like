//! 文本内容双阶段排版测量桥接器 (Intrinsic Sizing)
//!
//! 对标 CSS 规范：子元素先根据内容汇报自己的最小/最大内容尺寸 (min-content / max-content)，
//! 父容器再根据空间分配约束完成终态排版解算。

use text_layout::shaper::TextShaper;


/// 文本固有内容尺寸
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntrinsicSize {
    pub min_content_width: f32,
    pub max_content_width: f32,
    pub natural_height: f32,
}

/// 文本尺寸测量器
pub struct TextMeasurer;

impl TextMeasurer {
    /// 测量指定文本与字号的内在排版尺寸
    pub fn measure(text: &str, font_size: f32, max_width_constraint: Option<f32>) -> IntrinsicSize {
        if text.is_empty() {
            return IntrinsicSize {
                min_content_width: 0.0,
                max_content_width: 0.0,
                natural_height: font_size * 1.2,
            };
        }

        // 1. 计算单行无换行时的最大内容宽度 (max-content)
        let max_content_width = TextShaper::measure_line_width(text, font_size);

        // 2. 计算最长单词/单字的最小内容宽度 (min-content)
        let mut min_content_width: f32 = 0.0;
        for word in text.split_whitespace() {
            let w = TextShaper::measure_line_width(word, font_size);
            if w > min_content_width {
                min_content_width = w;
            }
        }
        if min_content_width == 0.0 {
            min_content_width = max_content_width;
        }

        // 3. 根据约束宽度计算最终折行排版高度
        let wrap_width = max_width_constraint.unwrap_or(max_content_width);
        let layout_result = TextShaper::layout_text(text, font_size, wrap_width);

        IntrinsicSize {
            min_content_width,
            max_content_width,
            natural_height: layout_result.total_height.max(font_size * 1.2),
        }
    }
}
