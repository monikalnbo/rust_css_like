//! 全局 Tab 焦点链与光标闪烁状态管理 (Focus Chain & Caret Manager)

use element_core::{ElementStateMask, ElementTag, ElementTree, NodeId};

pub struct FocusManager {
    pub focused_node: Option<NodeId>,
    pub focusable_nodes: Vec<NodeId>,
    pub caret_visible: bool,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            focused_node: None,
            focusable_nodes: Vec::new(),
            caret_visible: true,
        }
    }

    /// 注册或重建当前可获取焦点的控件链 (Button, Input, Checkbox 等)
    pub fn register_focusable(&mut self, node_id: NodeId) {
        if !self.focusable_nodes.contains(&node_id) {
            self.focusable_nodes.push(node_id);
        }
    }

    /// 自动从整树收集所有可聚焦的控件
    pub fn collect_from_tree(&mut self, tree: &ElementTree) {
        self.focusable_nodes.clear();
        for i in 0..tree.node_count() {
            let nid = NodeId(i as u32);
            if let Some(node) = tree.get_node(nid) {
                match node.tag {
                    ElementTag::Button
                    | ElementTag::Input
                    | ElementTag::Checkbox
                    | ElementTag::Select
                    | ElementTag::Slider => {
                        self.focusable_nodes.push(nid);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Tab 键向后轮转焦点
    pub fn focus_next(&mut self, tree: &mut ElementTree) -> Option<NodeId> {
        if self.focusable_nodes.is_empty() {
            return None;
        }

        let next_idx = match self.focused_node {
            Some(cur) => {
                let cur_idx = self
                    .focusable_nodes
                    .iter()
                    .position(|&n| n == cur)
                    .unwrap_or(0);
                (cur_idx + 1) % self.focusable_nodes.len()
            }
            None => 0,
        };

        let next_node = self.focusable_nodes[next_idx];
        self.set_focus(next_node, tree);
        Some(next_node)
    }

    /// Shift+Tab 键向前轮转焦点
    pub fn focus_prev(&mut self, tree: &mut ElementTree) -> Option<NodeId> {
        if self.focusable_nodes.is_empty() {
            return None;
        }

        let prev_idx = match self.focused_node {
            Some(cur) => {
                let cur_idx = self
                    .focusable_nodes
                    .iter()
                    .position(|&n| n == cur)
                    .unwrap_or(0);
                if cur_idx == 0 {
                    self.focusable_nodes.len() - 1
                } else {
                    cur_idx - 1
                }
            }
            None => self.focusable_nodes.len() - 1,
        };

        let prev_node = self.focusable_nodes[prev_idx];
        self.set_focus(prev_node, tree);
        Some(prev_node)
    }

    /// 显式聚焦指定节点
    pub fn set_focus(&mut self, node_id: NodeId, tree: &mut ElementTree) {
        if let Some(old) = self.focused_node {
            if let Some(old_node) = tree.get_node_mut(old) {
                old_node.state_mask.remove(ElementStateMask::FOCUSED);
            }
        }

        self.focused_node = Some(node_id);
        if let Some(new_node) = tree.get_node_mut(node_id) {
            new_node.state_mask.insert(ElementStateMask::FOCUSED);
        }
        self.caret_visible = true;
    }

    /// 光标 500ms 周期翻转
    pub fn tick_caret(&mut self) -> bool {
        self.caret_visible = !self.caret_visible;
        self.caret_visible
    }
}
