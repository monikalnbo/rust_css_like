//! 在线实时热重载事件定义 (Live Events)

#[derive(Clone, Debug, PartialEq)]
pub enum LiveEvent {
    /// 编辑器中源文本变动
    SourceTextChanged { new_source: String },
    /// 括号配对状态机触发：一个完整作用域 `{ ... }` 闭合！
    ScopeBlockClosed { depth: usize },
    /// 触发重新绘制帧
    RequestRepaint,
    /// 触发重新布局求解
    RequestRelayout,
}
