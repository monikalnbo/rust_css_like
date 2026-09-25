//! 将自研 ComputedStyle 转译为 Taffy 布局算法能理解的结构

use css_types::{AlignItems, Dimension, Display, FlexDirection, JustifyContent};
use style_system::ComputedStyle;

pub struct LayoutBridge;

impl LayoutBridge {
    pub fn to_taffy_style(style: &ComputedStyle) -> taffy::style::Style {
        let mut t_style = taffy::style::Style::default();

        // Display
        t_style.display = match style.display {
            Display::Flex => taffy::style::Display::Flex,
            Display::Grid => taffy::style::Display::Grid,
            Display::None => taffy::style::Display::None,
        };

        // Flex Direction
        t_style.flex_direction = match style.flex_direction {
            FlexDirection::Row => taffy::style::FlexDirection::Row,
            FlexDirection::Column => taffy::style::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
        };

        // Justify Content
        t_style.justify_content = match style.justify_content {
            JustifyContent::FlexStart => Some(taffy::style::JustifyContent::FlexStart),
            JustifyContent::Center => Some(taffy::style::JustifyContent::Center),
            JustifyContent::FlexEnd => Some(taffy::style::JustifyContent::FlexEnd),
            JustifyContent::SpaceBetween => Some(taffy::style::JustifyContent::SpaceBetween),
            JustifyContent::SpaceAround => Some(taffy::style::JustifyContent::SpaceAround),
            JustifyContent::SpaceEvenly => Some(taffy::style::JustifyContent::SpaceEvenly),
        };

        // Align Items
        t_style.align_items = match style.align_items {
            AlignItems::Stretch => Some(taffy::style::AlignItems::Stretch),
            AlignItems::FlexStart => Some(taffy::style::AlignItems::FlexStart),
            AlignItems::Center => Some(taffy::style::AlignItems::Center),
            AlignItems::FlexEnd => Some(taffy::style::AlignItems::FlexEnd),
        };

        // Size
        let map_dim = |d: Dimension| match d {
            Dimension::Auto => taffy::style::Dimension::Auto,
            Dimension::Px(px) => taffy::style::Dimension::Length(px),
            Dimension::Percent(pct) => taffy::style::Dimension::Percent(pct / 100.0),
        };

        t_style.size = taffy::geometry::Size {
            width: map_dim(style.size.width),
            height: map_dim(style.size.height),
        };

        t_style
    }
}
