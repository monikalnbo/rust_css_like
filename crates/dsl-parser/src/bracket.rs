//! `{}` 约束符追踪与闭合平衡状态机
//!
//! 用于实时监听编辑流中的大括号配对。
//! 一旦检测到某一个完整平衡闭合的 `{ ... }` 作用域，立即触发增量编译与即时预览信号。

#[derive(Clone, Debug, Default)]
pub struct BracketTracker {
    depth: usize,
    open_positions: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BracketEvent {
    /// 开启一个新的约束作用域 `{`
    ScopeOpened { depth: usize, position: usize },
    /// 成功合并闭合一个完整的作用域 `}`（触发增量实时预览的核心触发点）
    ScopeClosed {
        depth: usize,
        start_pos: usize,
        end_pos: usize,
    },
    /// 括号未平衡或多余闭合
    MismatchedClose { position: usize },
}

impl BracketTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// 当前作用域嵌套深度
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// 是否处于完全平衡的顶层状态
    pub fn is_balanced(&self) -> bool {
        self.depth == 0
    }

    /// 消费一个字符，若遇到 `{` 或 `}` 返回相应的平衡事件
    pub fn feed_char(&mut self, ch: char, position: usize) -> Option<BracketEvent> {
        match ch {
            '{' => {
                self.depth += 1;
                self.open_positions.push(position);
                Some(BracketEvent::ScopeOpened {
                    depth: self.depth,
                    position,
                })
            }
            '}' => {
                if self.depth > 0 {
                    self.depth -= 1;
                    let start_pos = self.open_positions.pop().unwrap_or(0);
                    Some(BracketEvent::ScopeClosed {
                        depth: self.depth + 1,
                        start_pos,
                        end_pos: position,
                    })
                } else {
                    Some(BracketEvent::MismatchedClose { position })
                }
            }
            _ => None,
        }
    }

    /// 重置状态机
    pub fn reset(&mut self) {
        self.depth = 0;
        self.open_positions.clear();
    }
}
