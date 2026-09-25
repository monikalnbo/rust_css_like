//! 抽象语法树定义 (Scope AST with Control Flow & Components)

use crate::span::Span;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f32),
    HexColor(String),
    Variable(String),
    Ident(String),
    /// 行内动作脚本代码 (由 `->` 引导)
    ActionCode(String),
    /// 算术或条件表达式 (如 "a + 1", "is_dark ? #000 : #fff")
    Expression(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyDecl {
    pub name: String,
    pub value: Value,
    pub is_important: bool,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScopeKind {
    /// 顶级窗口或视口
    Window,
    /// 元素容器 (box, row, col, navbar, card, btn, txt 等)
    Element(String),
    /// 富文本行内切片 (span)
    Span,
    /// 跨文件模块导入指令: @import "path/file.ui"
    Import(String),
    /// 自定义组件模版定义: component Card(title, desc) { ... }
    ComponentDef { name: String, params: Vec<String> },
    /// 循环展开控制流: for item in items { ... }
    ForLoop { item_var: String, iterable: String },
    /// 条件渲染控制流: if condition { ... }
    IfBranch { condition: String },
    /// 条件分支否则分支: else { ... }
    ElseBranch,
    /// 响应式状态定义: let count = 0
    StateLet { name: String, init_expr: String },
    /// 交互伪类约束块 (:hover, :active, :focus)
    Pseudo(String),
    /// 混入与复用块 (...mixin)
    MixinSpread(String),
    /// 主题或变量块 (theme)
    Theme,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScopeBlock {
    pub kind: ScopeKind,
    pub properties: Vec<PropertyDecl>,
    pub children: Vec<ScopeBlock>,
    pub span: Span,
}

impl ScopeBlock {
    pub fn new(kind: ScopeKind, span: Span) -> Self {
        Self {
            kind,
            properties: Vec::new(),
            children: Vec::new(),
            span,
        }
    }
}
