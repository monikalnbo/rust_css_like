//! 几何基础类型：点、尺寸、矩形、圆角

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point<T = f32> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect<T = f32> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T: Copy> Rect<T> {
    pub const fn all(val: T) -> Self {
        Self {
            top: val,
            right: val,
            bottom: val,
            left: val,
        }
    }

    pub const fn symmetric(vertical: T, horizontal: T) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    pub const fn trbl(top: T, right: T, bottom: T, left: T) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// 四个角的圆角半径
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadius {
    pub const ZERO: Self = Self::all(0.0);

    pub const fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }
}
