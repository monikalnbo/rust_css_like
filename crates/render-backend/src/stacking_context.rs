//! 局部层叠上下文树与 Z-Index 树形隔离合成器 (Stacking Context Tree)
//!
//! 对标 CSS 规范：子元素的 z-index 严格受制于父级层叠上下文，
//! 避免子元素 z-index 越狱穿透父同胞节点。

use crate::command::DrawCommand;
use crate::display_list::DisplayList;
use layout_engine::LayoutRect;

/// 独立的局部层叠上下文节点
#[derive(Clone, Debug)]
pub struct StackingContextNode {
    pub id: usize,
    pub z_index: i32,
    pub opacity: f32,
    pub clip_rect: Option<LayoutRect>,
    pub background_commands: Vec<DrawCommand>,
    pub content_commands: Vec<DrawCommand>,
    pub children: Vec<StackingContextNode>,
}

impl StackingContextNode {
    pub fn new(id: usize, z_index: i32) -> Self {
        Self {
            id,
            z_index,
            opacity: 1.0,
            clip_rect: None,
            background_commands: Vec::new(),
            content_commands: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn with_clip(mut self, clip: LayoutRect) -> Self {
        self.clip_rect = Some(clip);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn add_background_cmd(&mut self, cmd: DrawCommand) {
        self.background_commands.push(cmd);
    }

    pub fn add_content_cmd(&mut self, cmd: DrawCommand) {
        self.content_commands.push(cmd);
    }

    pub fn add_child_context(&mut self, child: StackingContextNode) {
        self.children.push(child);
    }

    /// 递归自底向上展平合成为最终的物理渲染 DisplayList
    pub fn flatten_to(&self, out: &mut DisplayList) {
        // 1. 若当前层叠上下文包含裁剪边界，压入裁剪
        if let Some(clip) = self.clip_rect {
            out.push(DrawCommand::PushClip { clip_rect: clip });
        }

        // 2. 绘制本层背景与边框
        for cmd in &self.background_commands {
            out.push(cmd.clone());
        }

        // 3. 收集子层叠上下文，拆分为负 z-index 与 非负 z-index
        let mut negative_children: Vec<&StackingContextNode> = Vec::new();
        let mut non_negative_children: Vec<&StackingContextNode> = Vec::new();

        for child in &self.children {
            if child.z_index < 0 {
                negative_children.push(child);
            } else {
                non_negative_children.push(child);
            }
        }

        // 负 z-index 严格升序展平
        negative_children.sort_by_key(|c| c.z_index);
        for child in negative_children {
            child.flatten_to(out);
        }

        // 4. 绘制本层普通流内容与文本
        for cmd in &self.content_commands {
            out.push(cmd.clone());
        }

        // 5. 非负 z-index 严格按 z-index 升序展平
        non_negative_children.sort_by_key(|c| c.z_index);
        for child in non_negative_children {
            child.flatten_to(out);
        }

        // 6. 弹出裁剪
        if self.clip_rect.is_some() {
            out.push(DrawCommand::PopClip);
        }
    }
}

/// 全局层叠上下文管理者
#[derive(Clone, Debug)]
pub struct StackingContextTree {
    pub root: StackingContextNode,
}

impl Default for StackingContextTree {
    fn default() -> Self {
        Self::new()
    }
}

impl StackingContextTree {
    pub fn new() -> Self {
        Self {
            root: StackingContextNode::new(0, 0),
        }
    }

    /// 整体合成为一条平面的线性 DisplayList
    pub fn compose(&self) -> DisplayList {
        let mut list = DisplayList::new();
        self.root.flatten_to(&mut list);
        list
    }
}
