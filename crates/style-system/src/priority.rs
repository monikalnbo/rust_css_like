//! 7 级确定性样式优先级判定与特异度打分

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    /// 继承或初始默认值
    Inherited = 0,
    /// 级联层普通声明
    LayerBase = 1,
    /// 混入属性 (...mixin)
    Mixin = 2,
    /// 当前作用域显式声明
    Local = 3,
    /// :hover / :focus 交互伪类
    HoverFocus = 4,
    /// :active 鼠标按下伪类
    Active = 5,
    /// !important 强制声明
    Important = 6,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyledProperty<T> {
    pub value: T,
    pub priority: PriorityLevel,
}

impl<T> StyledProperty<T> {
    pub fn new(value: T, priority: PriorityLevel) -> Self {
        Self { value, priority }
    }

    /// 若新属性优先级更高或同级覆盖，则执行替换
    pub fn override_with(&mut self, other: Self) {
        if other.priority >= self.priority {
            *self = other;
        }
    }
}
