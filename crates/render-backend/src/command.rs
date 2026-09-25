//! 底层 GPU 绘制图元指令定义 (Draw Commands)

use css_types::{BorderRadius, Color};
use layout_engine::LayoutRect;

/// 底层矢量与栅格渲染图元指令
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCommand {
    /// 绘制填充矩形（支持独立 4 轴圆角与边框）
    DrawRect {
        bounds: LayoutRect,
        color: Color,
        radius: BorderRadius,
        border_color: Color,
        border_width: f32,
    },
    /// 绘制弥散盒阴影
    DrawShadow {
        bounds: LayoutRect,
        blur_radius: f32,
        spread: f32,
        color: Color,
        offset_x: f32,
        offset_y: f32,
    },
    /// 绘制文本图元
    DrawText {
        text: String,
        font_size: f32,
        color: Color,
        position: (f32, f32),
    },
    /// 压入矩形裁剪遮罩 (对标 overflow: hidden)
    PushClip { clip_rect: LayoutRect },
    /// 弹出矩形裁剪遮罩
    PopClip,
    /// 绘制位图图像 (RGBA 纹理缓存)
    DrawImage {
        image_id: u32,
        bounds: LayoutRect,
        opacity: f32,
    },
    /// 绘制 SVG 矢量路径
    DrawSvgPath {
        path_data: String,
        bounds: LayoutRect,
        fill_color: Option<Color>,
        stroke_color: Option<Color>,
        stroke_width: f32,
    },
    /// 执行自定义扩展底层效果 (Custom Shader / Effect Pass)
    CustomEffect {
        effect_name: String,
        bounds: LayoutRect,
        uniforms: Vec<(String, f32)>,
    },
}
