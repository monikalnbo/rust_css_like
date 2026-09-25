//! DOM 三阶段事件派发体系与冒泡/捕获控制 (DOM Event Dispatcher)

use element_core::{ElementTree, NodeId};

/// 事件传播阶段
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventPhase {
    None,
    Capturing,
    Target,
    Bubbling,
}

/// 标准 DOM 事件对象
#[derive(Clone, Debug, PartialEq)]
pub struct DomEvent {
    pub event_type: String,
    pub target: NodeId,
    pub current_target: Option<NodeId>,
    pub phase: EventPhase,
    pub stopped: bool,
}

impl DomEvent {
    pub fn new(event_type: impl Into<String>, target: NodeId) -> Self {
        Self {
            event_type: event_type.into(),
            target,
            current_target: None,
            phase: EventPhase::None,
            stopped: false,
        }
    }

    /// 阻止事件在捕获或冒泡阶段继续向上/向下传播
    pub fn stop_propagation(&mut self) {
        self.stopped = true;
    }
}

pub type EventHandler = Box<dyn Fn(&mut DomEvent) + Send + Sync>;

/// DOM 事件调度中枢
pub struct EventDispatcher;

impl EventDispatcher {
    /// 计算从根节点至目标节点的完整祖先递送路径
    pub fn build_ancestor_path(target: NodeId, tree: &ElementTree) -> Vec<NodeId> {
        let mut path = Vec::new();
        let mut cur = Some(target);
        while let Some(nid) = cur {
            path.push(nid);
            cur = tree.get_node(nid).and_then(|n| n.parent);
        }
        path.reverse();
        path
    }

    /// 执行标准三阶段事件递送调度
    pub fn dispatch(
        event: &mut DomEvent,
        tree: &ElementTree,
        mut listener_hook: impl FnMut(NodeId, &mut DomEvent),
    ) {
        let path = Self::build_ancestor_path(event.target, tree);
        if path.is_empty() {
            return;
        }

        // 1. 捕获阶段 (Capturing Phase)：从根节点向下至目标父级
        event.phase = EventPhase::Capturing;
        for &nid in &path[..path.len().saturating_sub(1)] {
            if event.stopped {
                return;
            }
            event.current_target = Some(nid);
            listener_hook(nid, event);
        }

        // 2. 目标阶段 (Target Phase)
        if !event.stopped {
            event.phase = EventPhase::Target;
            event.current_target = Some(event.target);
            listener_hook(event.target, event);
        }

        // 3. 冒泡阶段 (Bubbling Phase)：从目标父级向上至根节点
        if !event.stopped {
            event.phase = EventPhase::Bubbling;
            for &nid in path[..path.len().saturating_sub(1)].iter().rev() {
                if event.stopped {
                    return;
                }
                event.current_target = Some(nid);
                listener_hook(nid, event);
            }
        }
    }
}
