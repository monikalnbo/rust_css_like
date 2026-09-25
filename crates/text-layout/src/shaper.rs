//! 文本字符塑形与自动断行度量引擎 (Text Shaper & Word Wrapping Engine)

use crate::paragraph::{LayoutLine, ParagraphLayout};
use crate::span::RichText;

pub struct TextShaper;

impl TextShaper {
    /// 估算单个字符的排版步进宽度 (基于标准字形度量常数)
    #[inline]
    pub fn char_advance(ch: char, font_size: f32) -> f32 {
        if ch.is_ascii() {
            if ch.is_ascii_punctuation() || ch == ' ' {
                font_size * 0.35
            } else if ch.is_ascii_uppercase() {
                font_size * 0.65
            } else {
                font_size * 0.52
            }
        } else {
            // CJK / 宽字符全角宽度
            font_size * 1.0
        }
    }

    /// 行高计算 (标准 1.35x 基线倍率)
    #[inline]
    pub fn line_height(font_size: f32) -> f32 {
        (font_size * 1.35).ceil()
    }

    /// 测量单行非换行文本的总宽度
    pub fn measure_line_width(text: &str, font_size: f32) -> f32 {
        text.chars().map(|c| Self::char_advance(c, font_size)).sum()
    }

    /// 对纯文本按指定容器最大可用宽度执行自动换行与行高求解
    pub fn layout_text(text: &str, font_size: f32, max_width: f32) -> ParagraphLayout {
        let mut lines = Vec::new();
        let line_h = Self::line_height(font_size);
        let mut cur_line_text = String::new();
        let mut cur_line_w = 0.0;
        let mut max_observed_w: f32 = 0.0;
        let mut y_offset = 0.0;

        for ch in text.chars() {
            if ch == '\n' {
                lines.push(LayoutLine::new(
                    cur_line_text.clone(),
                    cur_line_w,
                    line_h,
                    y_offset,
                ));
                max_observed_w = max_observed_w.max(cur_line_w);
                y_offset += line_h;
                cur_line_text.clear();
                cur_line_w = 0.0;
                continue;
            }

            let adv = Self::char_advance(ch, font_size);
            if cur_line_w + adv > max_width && !cur_line_text.is_empty() {
                // 触发折行
                lines.push(LayoutLine::new(
                    cur_line_text.clone(),
                    cur_line_w,
                    line_h,
                    y_offset,
                ));
                max_observed_w = max_observed_w.max(cur_line_w);
                y_offset += line_h;
                cur_line_text.clear();
                cur_line_w = 0.0;
            }

            cur_line_text.push(ch);
            cur_line_w += adv;
        }

        if !cur_line_text.is_empty() || lines.is_empty() {
            lines.push(LayoutLine::new(cur_line_text, cur_line_w, line_h, y_offset));
            max_observed_w = max_observed_w.max(cur_line_w);
            y_offset += line_h;
        }

        ParagraphLayout {
            lines,
            total_width: max_observed_w,
            total_height: y_offset,
        }
    }

    /// 对富文本段落进行排版
    pub fn layout_rich_text(rich: &RichText, max_width: f32) -> ParagraphLayout {
        let plain = rich.to_plain_string();
        let base_font_size = rich.spans.first().map(|s| s.font_size).unwrap_or(14.0);
        Self::layout_text(&plain, base_font_size, max_width)
    }

    /// 计算内在排版尺寸（用于给 Taffy measure_func 回调闭环）
    pub fn measure_intrinsic_size(text: &str, font_size: f32, max_width: f32) -> (f32, f32) {
        let layout = Self::layout_text(text, font_size, max_width);
        (layout.total_width, layout.total_height)
    }
}
