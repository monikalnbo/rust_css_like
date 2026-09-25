//! UI 渲染管线驱动 (DSL -> AST -> Elements -> Styles -> Layout -> Render)

use css_types::{BorderRadius, Color, Dimension, Display, FlexDirection, Size};
use dsl_parser::{parse_dsl, ScopeBlock, ScopeKind, Value};
use element_core::{ElementTag, ElementTree, NodeId};
use layout_engine::{LayoutEngineContext, LayoutRect};
use render_backend::{DisplayList, DrawCommand, SoftwareRenderer};
use std::collections::HashMap;
use style_system::ComputedStyle;
use tiny_skia::PixmapMut;

/// UI 核心驱动管线
pub struct UiPipeline {
    pub tree: ElementTree,
    pub root_node: Option<NodeId>,
    pub display_list: DisplayList,
    pub layout_context: LayoutEngineContext,
}

impl Default for UiPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl UiPipeline {
    pub fn new() -> Self {
        Self {
            tree: ElementTree::new(),
            root_node: None,
            display_list: DisplayList::new(),
            layout_context: LayoutEngineContext::new(),
        }
    }

    /// 从 DSL 源码构建渲染树并刷新
    pub fn load_dsl(&mut self, source: &str, width: f32, height: f32) -> Result<(), String> {
        let blocks = parse_dsl(source)?;
        self.tree = ElementTree::new();
        self.display_list.clear();

        let root = self.tree.create_node(ElementTag::Window);
        self.root_node = Some(root);

        for block in &blocks {
            self.convert_block(root, block);
        }

        self.layout_and_paint(width, height);
        Ok(())
    }

    fn convert_block(&mut self, parent: NodeId, block: &ScopeBlock) {
        let tag = match &block.kind {
            ScopeKind::Element(name) => match name.as_str() {
                "window" => ElementTag::Window,
                "box" | "col" | "row" | "card" => ElementTag::Box,
                "btn" | "button" => ElementTag::Button,
                "txt" | "text" => ElementTag::Text,
                "inp" | "input" => ElementTag::Input,
                _ => ElementTag::Custom(name.clone()),
            },
            _ => return,
        };

        let child = self.tree.create_node(tag);
        if let Some(node) = self.tree.get_node_mut(child) {
            for prop in &block.properties {
                let val_str = match &prop.value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::HexColor(c) => c.clone(),
                    Value::Variable(v) => v.clone(),
                    Value::Ident(i) => i.clone(),
                    Value::ActionCode(a) => a.clone(),
                    Value::Expression(e) => e.clone(),
                };
                node.inline_styles.insert(prop.name.clone(), val_str);
            }
        }

        self.tree.append_child(parent, child);
        for sub_block in &block.children {
            self.convert_block(child, sub_block);
        }
    }

    /// 执行整树排版并构建 DisplayList
    pub fn layout_and_paint(&mut self, width: f32, height: f32) {
        self.display_list.clear();
        let Some(root_id) = self.tree.root else {
            return;
        };

        // 1. 为虚拟 DOM 树各节点准备基础样式
        let mut styles = HashMap::new();
        self.prepare_styles(root_id, &mut styles);

        // 2. 调度 Taffy 进行整树排版求解
        if let Ok(taffy_root) = self.layout_context.build_taffy_tree(&self.tree, &styles) {
            let _ = self
                .layout_context
                .compute_layout(taffy_root, width, height);
        }

        // 3. 回填物理几何坐标并生成绘制图元
        let rects = self.layout_context.collect_layout_rects(&self.tree);
        self.emit_draw_commands(root_id, &rects, &styles);
    }

    fn prepare_styles(&self, node_id: NodeId, out: &mut HashMap<NodeId, ComputedStyle>) {
        let Some(node) = self.tree.get_node(node_id) else {
            return;
        };
        let mut style = ComputedStyle::default();

        match &node.tag {
            ElementTag::Window => {
                style.display = Display::Flex;
                style.flex_direction = FlexDirection::Column;
                style.background = Color::rgb(248, 250, 252);
            }
            ElementTag::Box => {
                style.display = Display::Flex;
                style.flex_direction = FlexDirection::Column;
                style.background = Color::WHITE;
                style.border_radius = BorderRadius::all(8.0);
            }
            ElementTag::Button => {
                style.size = Size {
                    width: Dimension::Px(120.0),
                    height: Dimension::Px(36.0),
                };
                style.background = Color::rgb(37, 99, 235);
                style.border_radius = BorderRadius::all(6.0);
                style.text_color = Color::WHITE;
            }
            _ => {}
        }

        // 应用 inline styles 中的自定义属性
        for (k, v) in &node.inline_styles {
            match k.as_str() {
                "bg" | "background" => {
                    if let Some(c) = Color::from_hex(v) {
                        style.background = c;
                    }
                }
                "color" => {
                    if let Some(c) = Color::from_hex(v) {
                        style.text_color = c;
                    }
                }
                "rad" | "radius" | "border-radius" => {
                    if let Ok(r) = v.parse::<f32>() {
                        style.border_radius = BorderRadius::all(r);
                    }
                }
                "w" | "width" => {
                    if let Ok(w) = v.parse::<f32>() {
                        style.size.width = Dimension::Px(w);
                    }
                }
                "h" | "height" => {
                    if let Ok(h) = v.parse::<f32>() {
                        style.size.height = Dimension::Px(h);
                    }
                }
                _ => {}
            }
        }

        out.insert(node_id, style);
        for &child in &node.children {
            self.prepare_styles(child, out);
        }
    }

    fn emit_draw_commands(
        &mut self,
        node_id: NodeId,
        rects: &HashMap<NodeId, LayoutRect>,
        styles: &HashMap<NodeId, ComputedStyle>,
    ) {
        let Some(node) = self.tree.get_node(node_id) else {
            return;
        };
        let rect = rects.get(&node_id).copied().unwrap_or(LayoutRect::ZERO);
        let default_style = ComputedStyle::default();
        let style = styles.get(&node_id).unwrap_or(&default_style);

        match &node.tag {
            ElementTag::Window | ElementTag::Box | ElementTag::Button => {
                self.display_list.push(DrawCommand::DrawRect {
                    bounds: rect,
                    color: style.background,
                    radius: style.border_radius,
                    border_color: style.border_color,
                    border_width: style.border_width,
                });
            }
            ElementTag::Text => {
                let text = node.text_content.as_deref().unwrap_or("Label");
                self.display_list.push(DrawCommand::DrawText {
                    text: text.to_string(),
                    font_size: style.font_size,
                    color: style.text_color,
                    position: (rect.x, rect.y),
                });
            }
            _ => {}
        }

        let children = node.children.clone();
        for child in children {
            self.emit_draw_commands(child, rects, styles);
        }
    }

    /// 光栅化呈现到 Pixmap
    pub fn render_to_pixmap(&self, pixmap: &mut PixmapMut) {
        SoftwareRenderer::render_to_pixmap(&self.display_list, pixmap);
    }
}
