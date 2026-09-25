//! 虚拟节点基础结构体 (ElementNode)

use crate::state::ElementStateMask;
use crate::tag::ElementTag;
use std::collections::HashMap;

/// 节点全局唯一标识符
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct NodeId(pub u32);

/// 轻量级 UI 虚拟节点
#[derive(Clone, Debug)]
pub struct ElementNode {
    pub id: NodeId,
    pub tag: ElementTag,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub state_mask: ElementStateMask,
    pub inline_styles: HashMap<String, String>,
    pub text_content: Option<String>,
}

impl ElementNode {
    pub fn new(id: NodeId, tag: ElementTag) -> Self {
        Self {
            id,
            tag,
            parent: None,
            children: Vec::new(),
            state_mask: ElementStateMask::NORMAL,
            inline_styles: HashMap::new(),
            text_content: None,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text_content = Some(text.into());
    }

    pub fn add_child(&mut self, child_id: NodeId) {
        self.children.push(child_id);
    }
}
