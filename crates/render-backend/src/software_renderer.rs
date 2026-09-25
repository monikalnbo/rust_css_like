//! 2D 软件光栅化渲染驱动 (Software Renderer powered by tiny-skia)

use crate::command::DrawCommand;
use crate::display_list::DisplayList;
use crate::font_atlas::FontAtlas;
use css_types::{BorderRadius, Color};
use layout_engine::LayoutRect;
use tiny_skia::{
    Color as SkColor, FillRule, Paint, Path, PathBuilder, PixmapMut, Rect, Stroke, Transform,
};

/// 纯 CPU 软件光栅化渲染驱动
pub struct SoftwareRenderer;

impl SoftwareRenderer {
    /// 将渲染指令队列光栅化至目标帧缓冲区 PixmapMut
    pub fn render_to_pixmap(list: &DisplayList, pixmap: &mut PixmapMut) {
        let mut clip_stack: Vec<Rect> = Vec::new();

        for cmd in list.commands() {
            match cmd {
                DrawCommand::DrawRect {
                    bounds,
                    color,
                    radius,
                    border_color,
                    border_width,
                } => {
                    Self::render_rect(pixmap, bounds, *color, radius, *border_color, *border_width);
                }
                DrawCommand::DrawShadow {
                    bounds,
                    blur_radius: _,
                    spread: _,
                    color,
                    offset_x,
                    offset_y,
                } => {
                    // 阴影渲染：偏移并弱化填充
                    let shadow_bounds = LayoutRect::new(
                        bounds.x + offset_x,
                        bounds.y + offset_y,
                        bounds.width,
                        bounds.height,
                    );
                    Self::render_rect(
                        pixmap,
                        &shadow_bounds,
                        *color,
                        &BorderRadius::all(4.0),
                        Color::TRANSPARENT,
                        0.0,
                    );
                }
                DrawCommand::DrawText {
                    text,
                    font_size,
                    color,
                    position,
                } => {
                    let sk_color = Self::convert_color(*color);
                    FontAtlas::draw_text_simple(
                        pixmap, text, position.0, position.1, *font_size, sk_color,
                    );
                }
                DrawCommand::PushClip { clip_rect } => {
                    if let Some(r) =
                        Rect::from_xywh(clip_rect.x, clip_rect.y, clip_rect.width, clip_rect.height)
                    {
                        clip_stack.push(r);
                    }
                }
                DrawCommand::PopClip => {
                    clip_stack.pop();
                }
                DrawCommand::DrawImage { bounds, opacity, .. } => {
                    let placeholder_color = Color::rgba(148, 163, 184, (*opacity * 255.0) as u8);
                    Self::render_rect(
                        pixmap,
                        bounds,
                        placeholder_color,
                        &BorderRadius::ZERO,
                        Color::TRANSPARENT,
                        0.0,
                    );
                }
                DrawCommand::DrawSvgPath {
                    bounds,
                    fill_color,
                    stroke_color,
                    stroke_width,
                    ..
                } => {
                    let fill = fill_color.unwrap_or(Color::TRANSPARENT);
                    let stroke = stroke_color.unwrap_or(Color::TRANSPARENT);
                    Self::render_rect(
                        pixmap,
                        bounds,
                        fill,
                        &BorderRadius::ZERO,
                        stroke,
                        *stroke_width,
                    );
                }
                DrawCommand::CustomEffect {
                    effect_name: _,
                    bounds,
                    uniforms: _,
                } => {
                    // 自定义效果底层兜底：渲染指示框
                    let effect_color = Color::rgba(59, 130, 246, 60);
                    Self::render_rect(
                        pixmap,
                        bounds,
                        effect_color,
                        &BorderRadius::all(2.0),
                        Color::TRANSPARENT,
                        0.0,
                    );
                }
            }
        }
    }

    /// 渲染带圆角与描边的矩形
    fn render_rect(
        pixmap: &mut PixmapMut,
        bounds: &LayoutRect,
        color: Color,
        radius: &BorderRadius,
        border_color: Color,
        border_width: f32,
    ) {
        if bounds.width <= 0.0 || bounds.height <= 0.0 {
            return;
        }

        let Some(path) = Self::build_rounded_rect_path(bounds, radius) else {
            return;
        };

        // 填充背景色
        if color.a > 0 {
            let mut paint = Paint::default();
            paint.set_color(Self::convert_color(color));
            paint.anti_alias = true;
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }

        // 绘制描边
        if border_width > 0.0 && border_color.a > 0 {
            let mut stroke_paint = Paint::default();
            stroke_paint.set_color(Self::convert_color(border_color));
            stroke_paint.anti_alias = true;
            let stroke = Stroke {
                width: border_width,
                ..Default::default()
            };
            pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
        }
    }

    /// 构建圆角矩形矢量路径
    fn build_rounded_rect_path(bounds: &LayoutRect, r: &BorderRadius) -> Option<Path> {
        let x = bounds.x;
        let y = bounds.y;
        let w = bounds.width;
        let h = bounds.height;

        let max_r = (w * 0.5).min(h * 0.5);
        let tl = r.top_left.min(max_r).max(0.0);
        let tr = r.top_right.min(max_r).max(0.0);
        let br = r.bottom_right.min(max_r).max(0.0);
        let bl = r.bottom_left.min(max_r).max(0.0);

        let mut pb = PathBuilder::new();
        pb.move_to(x + tl, y);
        pb.line_to(x + w - tr, y);
        if tr > 0.0 {
            pb.quad_to(x + w, y, x + w, y + tr);
        }
        pb.line_to(x + w, y + h - br);
        if br > 0.0 {
            pb.quad_to(x + w, y + h, x + w - br, y + h);
        }
        pb.line_to(x + bl, y + h);
        if bl > 0.0 {
            pb.quad_to(x, y + h, x, y + h - bl);
        }
        pb.line_to(x, y + tl);
        if tl > 0.0 {
            pb.quad_to(x, y, x + tl, y);
        }
        pb.close();
        pb.finish()
    }

    /// 将 css_types::Color 转换为 tiny_skia::Color
    #[inline]
    pub fn convert_color(c: Color) -> SkColor {
        SkColor::from_rgba8(c.r, c.g, c.b, c.a)
    }
}
