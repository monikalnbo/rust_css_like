//! 原生输入法 (IME) 拼音候选框绝对物理锚点定位底座

/// 输入法光标物理位置信息（用于通知 OS 将候选词悬浮窗挂载在光标下方）
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ImeCursorAnchor {
    pub physical_x: f32,
    pub physical_y: f32,
    pub line_height: f32,
}

impl ImeCursorAnchor {
    pub const fn new(physical_x: f32, physical_y: f32, line_height: f32) -> Self {
        Self {
            physical_x,
            physical_y,
            line_height,
        }
    }

    /// 候选框建议弹出锚点 (通常在输入文字行底)
    pub fn candidate_popup_position(&self) -> (f32, f32) {
        (self.physical_x, self.physical_y + self.line_height)
    }
}
