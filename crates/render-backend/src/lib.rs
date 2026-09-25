//! # Render Backend
//!
//! 底层 GPU 绘制指令流 (DisplayList)、DPI 视网膜缩放映射、Z-Index 命中测试、原生输入法 (IME) 定位底座，
//! 以及**自定义扩展效果 (CustomEffect) 与底层交互响应中枢**。

pub mod command;
pub mod display_list;
pub mod dpi;
pub mod effect;
pub mod font_atlas;
pub mod hit_test;
pub mod image_pipeline;
pub mod ime;
pub mod ripple_effect;
pub mod software_renderer;
pub mod stacking_context;

pub use command::DrawCommand;
pub use display_list::DisplayList;
pub use dpi::DpiScale;
pub use effect::{CustomEffect, EffectRegistry, InteractionEvent};
pub use ripple_effect::InteractiveRippleEffect;
pub use font_atlas::FontAtlas;
pub use hit_test::{HitCandidate, HitTester};
pub use image_pipeline::{ImagePipeline, RawImage, SvgPathParser, SvgPathSegment};
pub use ime::ImeCursorAnchor;
pub use software_renderer::SoftwareRenderer;
pub use stacking_context::{StackingContextNode, StackingContextTree};

#[cfg(test)]
mod tests {
    use super::*;
    use css_types::{BorderRadius, Color};
    use layout_engine::LayoutRect;

    #[test]
    fn test_display_list_push() {
        let mut list = DisplayList::new();
        list.push(DrawCommand::DrawRect {
            bounds: LayoutRect::new(0.0, 0.0, 100.0, 50.0),
            color: Color::WHITE,
            radius: BorderRadius::ZERO,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        });

        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_software_renderer_draw_rect() {
        use tiny_skia::Pixmap;

        let mut list = DisplayList::new();
        list.push(DrawCommand::DrawRect {
            bounds: LayoutRect::new(10.0, 10.0, 80.0, 40.0),
            color: Color::rgba(255, 0, 0, 255),
            radius: BorderRadius::all(4.0),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        });

        let mut pixmap = Pixmap::new(100, 60).unwrap();
        SoftwareRenderer::render_to_pixmap(&list, &mut pixmap.as_mut());

        // 校验中心像素被染成红色 (RGBA: 255, 0, 0, 255)
        let pixel = pixmap.pixel(50, 30).unwrap();
        assert_eq!(pixel.red(), 255);
        assert_eq!(pixel.green(), 0);
        assert_eq!(pixel.blue(), 0);
        assert_eq!(pixel.alpha(), 255);
    }
}
