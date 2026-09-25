//! 布局引擎整树递归排版调度器 (Tree Solver powered by Taffy)

use crate::bridge::LayoutBridge;
use crate::solver::LayoutRect;
use element_core::{ElementTree, NodeId};
use std::collections::HashMap;
use style_system::ComputedStyle;
use taffy::geometry::Size;
use taffy::style::AvailableSpace;
use taffy::TaffyTree;

/// 全树布局求解上下文环境
pub struct LayoutEngineContext {
    pub taffy: TaffyTree<()>,
    pub node_map: HashMap<NodeId, taffy::tree::NodeId>,
    pub reverse_map: HashMap<taffy::tree::NodeId, NodeId>,
}

impl Default for LayoutEngineContext {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngineContext {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            node_map: HashMap::new(),
            reverse_map: HashMap::new(),
        }
    }

    /// 清空上下文
    pub fn clear(&mut self) {
        self.taffy.clear();
        self.node_map.clear();
        self.reverse_map.clear();
    }

    /// 1. 递归将 ElementTree 构建进 TaffyTree 结构
    pub fn build_taffy_tree(
        &mut self,
        tree: &ElementTree,
        styles: &HashMap<NodeId, ComputedStyle>,
    ) -> Result<taffy::tree::NodeId, String> {
        self.clear();
        let root_id = tree
            .root
            .ok_or_else(|| "虚拟 DOM 树未指定根节点".to_string())?;
        self.build_recursive(root_id, tree, styles)
    }

    fn build_recursive(
        &mut self,
        node_id: NodeId,
        tree: &ElementTree,
        styles: &HashMap<NodeId, ComputedStyle>,
    ) -> Result<taffy::tree::NodeId, String> {
        let node = tree
            .get_node(node_id)
            .ok_or_else(|| format!("未找到节点 {:?}", node_id))?;
        let default_style = ComputedStyle::default();
        let style = styles.get(&node_id).unwrap_or(&default_style);
        let taffy_style = LayoutBridge::to_taffy_style(style);

        let mut child_taffy_ids = Vec::with_capacity(node.children.len());
        for &child_id in &node.children {
            let child_taffy_id = self.build_recursive(child_id, tree, styles)?;
            child_taffy_ids.push(child_taffy_id);
        }

        let taffy_id = self
            .taffy
            .new_with_children(taffy_style, &child_taffy_ids)
            .map_err(|e| format!("Taffy 创建节点失败: {:?}", e))?;

        self.node_map.insert(node_id, taffy_id);
        self.reverse_map.insert(taffy_id, node_id);

        Ok(taffy_id)
    }

    /// 2. 求解全局排版
    pub fn compute_layout(
        &mut self,
        root_taffy_id: taffy::tree::NodeId,
        available_width: f32,
        available_height: f32,
    ) -> Result<(), String> {
        let available_space = Size {
            width: AvailableSpace::Definite(available_width),
            height: AvailableSpace::Definite(available_height),
        };

        self.taffy
            .compute_layout(root_taffy_id, available_space)
            .map_err(|e| format!("Taffy 排版解算失败: {:?}", e))
    }

    /// 3. 将计算结果递归换算为物理全局绝对坐标 LayoutRect
    pub fn collect_layout_rects(&self, tree: &ElementTree) -> HashMap<NodeId, LayoutRect> {
        let mut results = HashMap::new();
        if let Some(root_id) = tree.root {
            self.collect_recursive(root_id, 0.0, 0.0, tree, &mut results);
        }
        results
    }

    fn collect_recursive(
        &self,
        node_id: NodeId,
        parent_abs_x: f32,
        parent_abs_y: f32,
        tree: &ElementTree,
        out: &mut HashMap<NodeId, LayoutRect>,
    ) {
        if let Some(&taffy_id) = self.node_map.get(&node_id) {
            if let Ok(layout) = self.taffy.layout(taffy_id) {
                let abs_x = parent_abs_x + layout.location.x;
                let abs_y = parent_abs_y + layout.location.y;
                let rect = LayoutRect::new(abs_x, abs_y, layout.size.width, layout.size.height);
                out.insert(node_id, rect);

                if let Some(node) = tree.get_node(node_id) {
                    for &child_id in &node.children {
                        self.collect_recursive(child_id, abs_x, abs_y, tree, out);
                    }
                }
            }
        }
    }
}
