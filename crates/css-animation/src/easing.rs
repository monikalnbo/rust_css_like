//! 缓动函数与三次贝塞尔曲线算法 (Easing & Cubic Bezier)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    /// 计算指定进度 t (0.0 <= t <= 1.0) 经过缓动曲线后的插值进度
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match *self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::CubicBezier(_p1x, p1y, _p2x, p2y) => {
                // 简化的贝塞尔多项式逼近
                let u = 1.0 - t;
                3.0 * u * u * t * p1y + 3.0 * u * t * t * p2y + t * t * t
            }
        }
    }
}
