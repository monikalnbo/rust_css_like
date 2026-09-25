//! 交互状态位标志掩码 (ElementStateMask)

use bitflags::bitflags;

bitflags! {
    /// 元素当前激活的交互状态集合 (可叠加)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct ElementStateMask: u8 {
        /// 默认静止状态
        const NORMAL = 0;
        /// 鼠标悬停激活 (:hover)
        const HOVERED = 1 << 0;
        /// 鼠标按下激活 (:active)
        const PRESSED = 1 << 1;
        /// 键盘焦点激活 (:focus)
        const FOCUSED = 1 << 2;
        /// 禁用状态 (:disabled)
        const DISABLED = 1 << 3;
    }
}
