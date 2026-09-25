//! 文本排版核心属性枚举

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    #[default]
    Normal, // 400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
}
