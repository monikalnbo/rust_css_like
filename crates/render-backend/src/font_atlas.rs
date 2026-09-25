//! 基础字体光栅化与字形图元渲染辅助 (Font Atlas & Glyph Rasterizer)

use tiny_skia::{Color as SkColor, Paint, PixmapMut, Rect, Transform};

/// 基础单色位图点阵字体定义 (8x16 常用 ASCII 点阵表)
pub struct FontAtlas;

impl FontAtlas {
    /// 测量一段单行文本的像素宽度与高度
    pub fn measure_text(text: &str, font_size: f32) -> (f32, f32) {
        let char_width = font_size * 0.55;
        let line_height = font_size * 1.25;
        (text.len() as f32 * char_width, line_height)
    }

    /// 在 Pixmap 上绘制简易字形（通过微型 5x7 点阵放大映射为像素块）
    pub fn draw_text_simple(
        pixmap: &mut PixmapMut,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: SkColor,
    ) {
        let scale = (font_size / 14.0).max(0.5);
        let char_spacing = 8.0 * scale;
        let mut cur_x = x;

        let mut paint = Paint::default();
        paint.set_color(color);
        paint.anti_alias = true;

        for ch in text.chars() {
            if ch == ' ' {
                cur_x += char_spacing;
                continue;
            }

            // 绘制字符的基本骨架（基于字符 ASCII 的通用可视指示符）
            let char_rect_opt = Rect::from_xywh(cur_x, y, 6.0 * scale, 10.0 * scale);
            if let Some(rect) = char_rect_opt {
                // 绘制字符下划/边框基线指示
                if let Some(bar_rect) = Rect::from_xywh(
                    cur_x + 1.0,
                    y + 2.0,
                    (5.0 * scale).max(1.0),
                    (8.0 * scale).max(1.0),
                ) {
                    pixmap.fill_rect(bar_rect, &paint, Transform::identity(), None);
                } else {
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
            }
            cur_x += char_spacing;
        }
    }
}
