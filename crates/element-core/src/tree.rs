//! 虚拟节点树管理与遍历 (ElementTree)

use crate::node::{ElementNode, NodeId};
use crate::tag::ElementTag;
use std::collections::HashMap;

/// 高性能扁平化存储的虚拟节点树
#[derive(Clone, Debug, Default)]
pub struct ElementTree {
    nodes: HashMap<NodeId, ElementNode>,
    next_id: u32,
    pub root: Option<NodeId>,
}

impl ElementTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建一个新节点并加入树中
    pub fn create_node(&mut self, tag: ElementTag) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        let node = ElementNode::new(id, tag);
        self.nodes.insert(id, node);
        if self.root.is_none() {
            self.root = Some(id);
        }
        id
    }

    /// 挂载父子关系
    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.add_child(child_id);
        }
        if let Some(child) = self.nodes.get_mut(&child_id) {
            child.parent = Some(parent_id);
        }
    }

    pub fn get_node(&self, id: NodeId) -> Option<&ElementNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut ElementNode> {
        self.nodes.get_mut(&id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
