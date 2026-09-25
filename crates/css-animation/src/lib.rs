//! # CSS Animation
//!
//! 全帧率平滑动效、缓动曲线与数学插值引擎。

pub mod easing;
pub mod lerp;
pub mod transition;

pub use easing::Easing;
pub use lerp::Lerp;
pub use transition::Transition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_progress() {
        let mut trans = Transition::new(0.0f32, 100.0f32, 1.0, Easing::Linear);
        let mid = trans.tick(0.5);
        assert!((mid - 50.0).abs() < 0.01);
        let end = trans.tick(0.6);
        assert_eq!(end, 100.0);
        assert!(trans.is_finished);
    }
}
