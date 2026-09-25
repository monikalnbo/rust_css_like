//! Web Canvas 2D 上下文渲染驱动 (Canvas 2D Backend for WebAssembly)

use css_types::Color;
use render_backend::{DisplayList, DrawCommand};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Canvas2dRenderer;

impl Canvas2dRenderer {
    /// 将 DisplayList 指令逐项渲染到 HTML5 Canvas
    pub fn render(context: &CanvasRenderingContext2d, list: &DisplayList) -> Result<(), JsValue> {
        for cmd in list.commands() {
            match cmd {
                DrawCommand::DrawRect {
                    bounds,
                    color,
                    radius,
                    border_color,
                    border_width,
                } => {
                    let fill_css = Self::color_to_css(*color);
                    context.set_fill_style(&JsValue::from_str(&fill_css));

                    let x = bounds.x as f64;
                    let y = bounds.y as f64;
                    let w = bounds.width as f64;
                    let h = bounds.height as f64;
                    let r = radius.top_left.max(0.0) as f64;

                    context.begin_path();
                    if r > 0.0 {
                        // 绘制圆角路径
                        let _ = context.round_rect_with_f64(x, y, w, h, r);
                    } else {
                        context.rect(x, y, w, h);
                    }

                    if color.a > 0 {
                        context.fill();
                    }

                    if *border_width > 0.0 && border_color.a > 0 {
                        let stroke_css = Self::color_to_css(*border_color);
                        context.set_stroke_style(&JsValue::from_str(&stroke_css));
                        context.set_line_width(*border_width as f64);
                        context.stroke();
                    }
                }
                DrawCommand::DrawText {
                    text,
                    font_size,
                    color,
                    position,
                } => {
                    let fill_css = Self::color_to_css(*color);
                    context.set_fill_style(&JsValue::from_str(&fill_css));
                    context.set_font(&format!("{}px sans-serif", font_size));
                    let _ =
                        context.fill_text(text, position.0 as f64, (position.1 + font_size) as f64);
                }
                DrawCommand::PushClip { clip_rect } => {
                    context.save();
                    context.begin_path();
                    context.rect(
                        clip_rect.x as f64,
                        clip_rect.y as f64,
                        clip_rect.width as f64,
                        clip_rect.height as f64,
                    );
                    context.clip();
                }
                DrawCommand::PopClip => {
                    context.restore();
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn color_to_css(c: Color) -> String {
        format!("rgba({}, {}, {}, {})", c.r, c.g, c.b, c.a as f32 / 255.0)
    }
}
