//! 富文本切片与分段定义 (Rich Text Spans & Segments)

use css_types::Color;

/// 富文本局部样式切片
#[derive(Clone, Debug, PartialEq)]
pub struct TextSpan {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub is_bold: bool,
    pub is_italic: bool,
}

impl TextSpan {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        Self {
            text: text.into(),
            font_size,
            color,
            is_bold: false,
            is_italic: false,
        }
    }

    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.is_italic = true;
        self
    }
}

/// 由多个切片组成的富文本段落
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RichText {
    pub spans: Vec<TextSpan>,
}

impl RichText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, span: TextSpan) {
        self.spans.push(span);
    }

    pub fn to_plain_string(&self) -> String {
        self.spans.iter().map(|s| s.text.as_str()).collect()
    }
}
