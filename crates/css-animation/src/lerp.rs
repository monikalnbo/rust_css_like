//! 通用插值器 Trait (Lerp)

use css_types::Color;

pub trait Lerp {
    fn lerp(&self, target: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, target: &Self, t: f32) -> Self {
        self + (target - self) * t
    }
}

impl Lerp for Color {
    fn lerp(&self, target: &Self, t: f32) -> Self {
        Color::lerp(*self, *target, t)
    }
}
