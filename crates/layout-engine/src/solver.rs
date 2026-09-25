//! 布局几何求解器输出定义



/// 每个节点由布局引擎解算出的绝对物理像素位置与尺寸
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl LayoutRect {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// 检测点 (px, py) 是否落在矩形内（用于鼠标悬停与点击命中测试）
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}
