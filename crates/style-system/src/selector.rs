//! 现代 CSS 选择器语法与特异度打分模型 (CSS Selector Model)

use crate::matcher::SelectorMatcher;
use element_core::{ElementTree, NodeId};

/// 单个原子选择器
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleSelector {
    Tag(String),
    Class(String),
    Id(String),
    PseudoHover,
    PseudoFocus,
    PseudoActive,
}

/// 选择器组合关系
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Combinator {
    Child,      // `>` 直接子代
    Descendant, // 空格 后代
}

/// 复合选择器链 (如 `.card > .btn:hover`)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Selector {
    pub segments: Vec<(Option<Combinator>, Vec<SimpleSelector>)>,
}

impl Selector {
    pub fn new() -> Self {
        Self::default()
    }

    /// 解析简易 CSS 选择器字符串
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut segments = Vec::new();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut pending_combinator: Option<Combinator> = None;

        for part in parts {
            if part == ">" {
                pending_combinator = Some(Combinator::Child);
                continue;
            }

            let mut simples = Vec::new();
            let mut cur = String::new();
            let mut mode = 't'; // 't' tag, '.' class, '#' id, ':' pseudo

            for ch in part.chars() {
                if ch == '.' || ch == '#' || ch == ':' {
                    if !cur.is_empty() {
                        Self::push_simple(&mut simples, mode, &cur);
                        cur.clear();
                    }
                    mode = ch;
                } else {
                    cur.push(ch);
                }
            }
            if !cur.is_empty() {
                Self::push_simple(&mut simples, mode, &cur);
            }

            let comb = pending_combinator.take();
            segments.push((comb, simples));
        }

        Ok(Selector { segments })
    }

    fn push_simple(simples: &mut Vec<SimpleSelector>, mode: char, val: &str) {
        match mode {
            '.' => simples.push(SimpleSelector::Class(val.to_string())),
            '#' => simples.push(SimpleSelector::Id(val.to_string())),
            ':' => match val {
                "hover" => simples.push(SimpleSelector::PseudoHover),
                "focus" => simples.push(SimpleSelector::PseudoFocus),
                "active" => simples.push(SimpleSelector::PseudoActive),
                _ => {}
            },
            _ => simples.push(SimpleSelector::Tag(val.to_string())),
        }
    }

    /// 计算选择器特异度得分 (ID: 100, Class: 10, Tag: 1)
    pub fn specificity(&self) -> u32 {
        let mut score = 0;
        for (_, simples) in &self.segments {
            for s in simples {
                match s {
                    SimpleSelector::Id(_) => score += 100,
                    SimpleSelector::Class(_) => score += 10,
                    SimpleSelector::PseudoHover
                    | SimpleSelector::PseudoFocus
                    | SimpleSelector::PseudoActive => score += 10,
                    SimpleSelector::Tag(_) => score += 1,
                }
            }
        }
        score
    }

    /// 判断当前节点是否精确匹配该选择器链
    #[inline]
    pub fn matches(&self, node_id: NodeId, tree: &ElementTree) -> bool {
        SelectorMatcher::matches(self, node_id, tree)
    }
}
