//! 伪元素虚拟盒子机制 (::before / ::after)
//!
//! 对标 CSS 规范：在虚拟 DOM 构建与渲染期，自动向当前宿主节点的
//! 子节点列表头部或尾部注入合成虚拟节点 (Synthetic Virtual Node)。

use crate::node::NodeId;
use crate::tag::ElementTag;
use crate::tree::ElementTree;
use std::collections::HashMap;

/// 伪元素分类
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PseudoKind {
    Before,
    After,
}

/// 伪元素声明规格
#[derive(Clone, Debug)]
pub struct PseudoElementDef {
    pub kind: PseudoKind,
    pub content: String,
    pub styles: HashMap<String, String>,
}

impl PseudoElementDef {
    pub fn before(content: impl Into<String>) -> Self {
        Self {
            kind: PseudoKind::Before,
            content: content.into(),
            styles: HashMap::new(),
        }
    }

    pub fn after(content: impl Into<String>) -> Self {
        Self {
            kind: PseudoKind::After,
            content: content.into(),
            styles: HashMap::new(),
        }
    }

    pub fn with_style(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.styles.insert(key.into(), value.into());
        self
    }
}

/// 伪元素合成注入器
pub struct PseudoInjector;

impl PseudoInjector {
    /// 向指定宿主节点注入合成伪元素虚拟盒子
    pub fn inject(
        tree: &mut ElementTree,
        host_id: NodeId,
        pseudo: &PseudoElementDef,
    ) -> Option<NodeId> {
        // 1. 确保宿主节点存在
        if tree.get_node(host_id).is_none() {
            return None;
        }

        // 2. 根据伪元素内容决定创建文本还是盒子
        let tag = if pseudo.content.is_empty() {
            ElementTag::Box
        } else {
            ElementTag::Text
        };

        let synthetic_id = tree.create_node(tag);

        // 3. 配置合成节点的文本内容与样式
        if let Some(synthetic_node) = tree.get_node_mut(synthetic_id) {
            synthetic_node.parent = Some(host_id);
            if !pseudo.content.is_empty() {
                synthetic_node.set_text(pseudo.content.clone());
            }
            synthetic_node.inline_styles = pseudo.styles.clone();
        }

        // 4. 根据 Before/After 分别插入到宿主子节点列表头部或尾部
        if let Some(host_node) = tree.get_node_mut(host_id) {
            match pseudo.kind {
                PseudoKind::Before => {
                    host_node.children.insert(0, synthetic_id);
                }
                PseudoKind::After => {
                    host_node.children.push(synthetic_id);
                }
            }
        }

        Some(synthetic_id)
    }

    /// 批量为宿主节点应用前后伪元素
    pub fn apply_decorations(
        tree: &mut ElementTree,
        host_id: NodeId,
        before: Option<&PseudoElementDef>,
        after: Option<&PseudoElementDef>,
    ) {
        if let Some(b) = before {
            Self::inject(tree, host_id, b);
        }
        if let Some(a) = after {
            Self::inject(tree, host_id, a);
        }
    }
}
