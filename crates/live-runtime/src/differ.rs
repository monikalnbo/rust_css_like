//! 脏标记与差异比对器 (Dirty Invalidation)

use bitflags::bitflags;

bitflags! {
    /// 变动引起的刷新级别
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct DirtyMask: u8 {
        /// 无需刷新
        const CLEAN = 0;
        /// 仅视觉属性变化（颜色、阴影、滤镜）──► 直接重绘 (<1ms)
        const REPAINT = 1 << 0;
        /// 尺寸几何变化（宽度、高度、边距、Flex）──► 触发重排布局 (<3ms)
        const RELAYOUT = 1 << 1;
        /// 节点结构增删 ──► 全量树重建
        const RESTRUCTURE = 1 << 2;
    }
}
