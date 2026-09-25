//! 排版段落与分行几何模型 (Paragraph & Line Layout)

/// 单行排版几何数据
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutLine {
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
}

impl LayoutLine {
    pub fn new(text: String, width: f32, height: f32, y_offset: f32) -> Self {
        Self {
            text,
            width,
            height,
            y_offset,
        }
    }
}

/// 整段文本断行计算完成后的整体度量
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParagraphLayout {
    pub lines: Vec<LayoutLine>,
    pub total_width: f32,
    pub total_height: f32,
}

impl ParagraphLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
