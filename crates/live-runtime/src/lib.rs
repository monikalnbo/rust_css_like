//! # Live Runtime
//!
//! `{}` 作用域闭合监听、脏标记增量预览、标准 DOM 事件三阶段流与全局 Tab 焦点链。

pub mod differ;
pub mod dom_event;
pub mod event;
pub mod focus;

pub use differ::DirtyMask;
pub use dom_event::{DomEvent, EventDispatcher, EventHandler, EventPhase};
pub use event::LiveEvent;
pub use focus::FocusManager;

#[cfg(test)]
mod tests {
    use super::*;
    use element_core::{ElementStateMask, ElementTag, ElementTree};

    #[test]
    fn test_dom_event_capturing_and_bubbling_with_stop() {
        let mut tree = ElementTree::new();
        let parent = tree.create_node(ElementTag::Box);
        let child_btn = tree.create_node(ElementTag::Button);
        tree.append_child(parent, child_btn);

        // 记录访问日志
        let mut trace = Vec::new();
        let mut event = DomEvent::new("click", child_btn);

        // 未阻断情况：parent 捕获 -> child 目标 -> parent 冒泡
        EventDispatcher::dispatch(&mut event, &tree, |nid, ev| {
            trace.push((nid, ev.phase));
        });

        assert_eq!(
            trace,
            vec![
                (parent, EventPhase::Capturing),
                (child_btn, EventPhase::Target),
                (parent, EventPhase::Bubbling),
            ]
        );

        // 阻断冒泡情况
        trace.clear();
        let mut event_blocked = DomEvent::new("click", child_btn);
        EventDispatcher::dispatch(&mut event_blocked, &tree, |nid, ev| {
            trace.push((nid, ev.phase));
            if nid == child_btn && ev.phase == EventPhase::Target {
                ev.stop_propagation();
            }
        });

        // 确认 parent 不会收到 Bubbling 事件
        assert_eq!(
            trace,
            vec![
                (parent, EventPhase::Capturing),
                (child_btn, EventPhase::Target),
            ]
        );
    }

    #[test]
    fn test_tab_focus_chain_rotation() {
        let mut tree = ElementTree::new();
        let win = tree.create_node(ElementTag::Window);
        let inp1 = tree.create_node(ElementTag::Input);
        let inp2 = tree.create_node(ElementTag::Input);
        let inp3 = tree.create_node(ElementTag::Input);
        tree.append_child(win, inp1);
        tree.append_child(win, inp2);
        tree.append_child(win, inp3);

        let mut focus_mgr = FocusManager::new();
        focus_mgr.collect_from_tree(&tree);
        assert_eq!(focus_mgr.focusable_nodes.len(), 3);

        // 连续按 Tab 键：1 -> 2 -> 3 -> 1 轮转
        assert_eq!(focus_mgr.focus_next(&mut tree), Some(inp1));
        assert!(tree
            .get_node(inp1)
            .unwrap()
            .state_mask
            .contains(ElementStateMask::FOCUSED));

        assert_eq!(focus_mgr.focus_next(&mut tree), Some(inp2));
        assert!(!tree
            .get_node(inp1)
            .unwrap()
            .state_mask
            .contains(ElementStateMask::FOCUSED));
        assert!(tree
            .get_node(inp2)
            .unwrap()
            .state_mask
            .contains(ElementStateMask::FOCUSED));

        assert_eq!(focus_mgr.focus_next(&mut tree), Some(inp3));
        assert_eq!(focus_mgr.focus_next(&mut tree), Some(inp1));
    }
}
