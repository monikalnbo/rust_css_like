//! 对标 HTML 基础元素原语的 Tag 枚举与表单控件族

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ElementTag {
    /// 顶级原生窗口表面
    Window,
    /// 万能矩形容器 (对标 div)
    Box,
    /// 横向流式容器 (对标 flex-direction: row)
    Row,
    /// 纵向流式容器 (对标 flex-direction: column)
    Col,
    /// 块级文本排版单元 (对标 p / h1-h6)
    Text,
    /// 行内文本切片单元 (对标 inline span，用于富文本混排)
    Span,
    /// 可交互动作容器 (对标 button)
    Button,
    /// 原生单行/多行输入框 (对标 input / textarea)
    Input,
    /// 复选框控件 (对标 input[type="checkbox"])
    Checkbox,
    /// 单选框控件 (对标 input[type="radio"])
    /// 下拉选择菜单 (对标 select)
    Select,
    /// 下拉选项单元 (对标 option)
    Option,
    /// 数值范围滑块 (对标 input[type="range"] / slider)
    Slider,
    /// 栅格与矢量图片展示 (对标 img / svg)
    Image,
    /// 高性能滚动视口 (对标 overflow: scroll)
    ScrollView,
    /// 自定义组件扩展
    Custom(String),
}

impl Default for ElementTag {
    fn default() -> Self {
        Self::Box
    }
}
