//! 紧凑型计算样式结构体 (ComputedStyle POD 布局)

use css_types::{
    AlignItems, BorderRadius, Color, Dimension, Display, FlexDirection, JustifyContent, Rect, Size,
};

/// 最终用于排版与 GPU 渲染的紧凑计算样式
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ComputedStyle {
    // 盒模型与布局（给 Taffy 消费）
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub size: Size<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,

    // 视觉与边框（给 GPU 渲染层消费）
    pub background: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: BorderRadius,
    pub opacity: f32,
    pub z_index: i32,

    // 文本属性（可继承）
    pub text_color: Color,
    pub font_size: f32,
}

impl Default for ComputedStyle {
    fn default() -> Self {
        Self {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            size: Size::new(Dimension::Auto, Dimension::Auto),
            margin: Rect::all(Dimension::Px(0.0)),
            padding: Rect::all(Dimension::Px(0.0)),
            background: Color::TRANSPARENT,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            border_radius: BorderRadius::ZERO,
            opacity: 1.0,
            z_index: 0,
            text_color: Color::WHITE,
            font_size: 14.0,
        }
    }
}
