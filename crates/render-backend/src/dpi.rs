//! 高分屏 (High-DPI / Retina) 物理像素与逻辑像素坐标映射

/// DPI 缩放比率管理器
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DpiScale {
    pub factor: f32, // 例如 1.0 (标准), 1.25, 1.5, 2.0 (macOS Retina / 4K)
}

impl Default for DpiScale {
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

impl DpiScale {
    pub const fn new(factor: f32) -> Self {
        Self {
            factor: if factor <= 0.0 { 1.0 } else { factor },
        }
    }

    /// 逻辑像素 (DIP / CSS Pixel) ──► 操作系统物理像素 (Device Pixel)
    pub fn to_physical(&self, logical_px: f32) -> f32 {
        logical_px * self.factor
    }

    /// 操作系统物理像素 ──► 逻辑像素
    pub fn to_logical(&self, physical_px: f32) -> f32 {
        physical_px / self.factor
    }

    /// 物理坐标点转换
    pub fn point_to_logical(&self, px: f32, py: f32) -> (f32, f32) {
        (self.to_logical(px), self.to_logical(py))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpi_conversion() {
        let dpi = DpiScale::new(2.0); // 视网膜屏
        assert_eq!(dpi.to_physical(100.0), 200.0);
        assert_eq!(dpi.to_logical(200.0), 100.0);
    }
}
