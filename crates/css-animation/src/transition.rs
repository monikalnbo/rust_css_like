//! 交互状态过渡控制器 (Transition State Machine)

use crate::easing::Easing;
use crate::lerp::Lerp;

#[derive(Clone, Debug)]
pub struct Transition<T: Lerp> {
    pub from: T,
    pub to: T,
    pub duration_secs: f32,
    pub elapsed_secs: f32,
    pub easing: Easing,
    pub is_finished: bool,
}

impl<T: Lerp + Clone> Transition<T> {
    pub fn new(from: T, to: T, duration_secs: f32, easing: Easing) -> Self {
        Self {
            from,
            to,
            duration_secs: duration_secs.max(0.001),
            elapsed_secs: 0.0,
            easing,
            is_finished: false,
        }
    }

    /// 帧进更新时间 delta，并返回当前插值结果
    pub fn tick(&mut self, dt_secs: f32) -> T {
        if self.is_finished {
            return self.to.clone();
        }
        self.elapsed_secs += dt_secs;
        let progress = (self.elapsed_secs / self.duration_secs).min(1.0);
        let curved_t = self.easing.evaluate(progress);
        let current = self.from.lerp(&self.to, curved_t);

        if progress >= 1.0 {
            self.is_finished = true;
        }
        current
    }
}
