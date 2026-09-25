//! # CSS Types
//!
//! 轻量、紧凑、高性能的原生样式与几何基础类型定义。

pub mod color;
pub mod geometry;
pub mod layout_props;
pub mod text_props;
pub mod units;

pub use color::Color;
pub use geometry::{BorderRadius, Point, Rect, Size};
pub use layout_props::{AlignItems, Display, FlexDirection, JustifyContent, Overflow, Position};
pub use text_props::{FontWeight, TextAlign, TextOverflow};
pub use units::{Dimension, Length};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color() {
        let c = Color::from_hex("#4f46e5").unwrap();
        assert_eq!(c.r, 0x4f);
        assert_eq!(c.g, 0x46);
        assert_eq!(c.b, 0xe5);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_color_lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;
        let mid = black.lerp(white, 0.5);
        assert_eq!(mid.r, 128);
        assert_eq!(mid.g, 128);
        assert_eq!(mid.b, 128);
    }
}
