//! 尺寸度量单位系统 (Px, Percentage, Dimension)

/// 绝对长度与相对度量
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Px(f32),
    Em(f32),
    Rem(f32),
}

impl Default for Length {
    fn default() -> Self {
        Self::Px(0.0)
    }
}

/// 支持百分比与自动计算的统一维度表达
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension {
    /// 自动尺寸 (对标 CSS auto)
    Auto,
    /// 绝对物理像素 (px)
    Px(f32),
    /// 百分比 (0.0 ~ 100.0)
    Percent(f32),
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Auto
    }
}

impl Dimension {
    pub const fn px(val: f32) -> Self {
        Self::Px(val)
    }

    pub const fn percent(val: f32) -> Self {
        Self::Percent(val)
    }

    /// 尝试计算绝对像素值
    pub fn resolve_or(&self, parent_size: f32, default: f32) -> f32 {
        match *self {
            Self::Px(v) => v,
            Self::Percent(p) => parent_size * (p / 100.0),
            Self::Auto => default,
        }
    }
}
