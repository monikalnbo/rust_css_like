//! CSS 选择器在 ElementTree 上的命中判定器 (Selector DOM Matcher)

use crate::selector::{Combinator, Selector, SimpleSelector};
use element_core::{ElementStateMask, ElementTag, ElementTree, NodeId};

pub struct SelectorMatcher;

impl SelectorMatcher {
    /// 判断指定节点是否精确匹配该选择器链
    pub fn matches(selector: &Selector, node_id: NodeId, tree: &ElementTree) -> bool {
        if selector.segments.is_empty() {
            return false;
        }

        let mut seg_idx = selector.segments.len() - 1;
        let mut cur_node_id = Some(node_id);

        while let Some(nid) = cur_node_id {
            let (comb, simples) = &selector.segments[seg_idx];
            let matched = Self::match_simples(nid, simples, tree);

            if seg_idx == selector.segments.len() - 1 {
                if !matched {
                    return false;
                }
            } else if matched {
                // 祖先或父级匹配成功
            } else if *comb == Some(Combinator::Child) {
                // 直接子代要求严格父级匹配
                return false;
            } else {
                // 后代选择器继续沿 parent 向上探查
                cur_node_id = tree.get_node(nid).and_then(|n| n.parent);
                continue;
            }

            if seg_idx == 0 {
                return true;
            }

            seg_idx -= 1;
            let next_comb = selector.segments[seg_idx + 1].0;
            cur_node_id = tree.get_node(nid).and_then(|n| n.parent);

            if cur_node_id.is_none() {
                return false;
            }
            if next_comb == Some(Combinator::Child) {
                // 仅上溯一级
            }
        }

        false
    }

    fn match_simples(node_id: NodeId, simples: &[SimpleSelector], tree: &ElementTree) -> bool {
        let Some(node) = tree.get_node(node_id) else {
            return false;
        };

        for s in simples {
            match s {
                SimpleSelector::Tag(expected_tag) => {
                    let actual_tag = match &node.tag {
                        ElementTag::Window => "window",
                        ElementTag::Box => "box",
                        ElementTag::Button => "btn",
                        ElementTag::Text => "txt",
                        ElementTag::Input => "inp",
                        ElementTag::Custom(c) => c.as_str(),
                        _ => "",
                    };
                    if actual_tag != expected_tag && expected_tag != "*" {
                        return false;
                    }
                }
                SimpleSelector::Class(cls) => {
                    let class_attr = node
                        .inline_styles
                        .get("class")
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    let has_class = class_attr.split_whitespace().any(|c| c == cls);
                    if !has_class {
                        return false;
                    }
                }
                SimpleSelector::Id(id_val) => {
                    let id_attr = node
                        .inline_styles
                        .get("id")
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    if id_attr != id_val {
                        return false;
                    }
                }
                SimpleSelector::PseudoHover => {
                    if !node.state_mask.contains(ElementStateMask::HOVERED) {
                        return false;
                    }
                }
                SimpleSelector::PseudoFocus => {
                    if !node.state_mask.contains(ElementStateMask::FOCUSED) {
                        return false;
                    }
                }
                SimpleSelector::PseudoActive => {
                    if !node.state_mask.contains(ElementStateMask::PRESSED) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
