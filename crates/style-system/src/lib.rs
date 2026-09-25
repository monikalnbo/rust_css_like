//! # Style System
//!
//! 对标 CSS3 的层叠计算、复杂选择器匹配、calc() 动态求解与作用域变量系统。

pub mod calc;
pub mod computed;
pub mod matcher;
pub mod priority;
pub mod selector;
pub mod variables;

pub use calc::CalcSolver;
pub use computed::ComputedStyle;
pub use matcher::SelectorMatcher;
pub use priority::{PriorityLevel, StyledProperty};
pub use selector::{Combinator, Selector, SimpleSelector};
pub use variables::VariableTable;

#[cfg(test)]
mod tests {
    use super::*;
    use element_core::{ElementStateMask, ElementTag, ElementTree};
    use std::sync::Arc;

    #[test]
    fn test_selector_matching_and_specificity() {
        let mut tree = ElementTree::new();
        let card = tree.create_node(ElementTag::Box);
        let btn = tree.create_node(ElementTag::Button);
        tree.append_child(card, btn);

        // 设置 card 类名
        tree.get_node_mut(card)
            .unwrap()
            .inline_styles
            .insert("class".to_string(), "card main-card".to_string());

        // 设置 btn 类名与 Hover 交互状态
        let btn_node = tree.get_node_mut(btn).unwrap();
        btn_node
            .inline_styles
            .insert("class".to_string(), "btn btn-primary".to_string());
        btn_node.state_mask.insert(ElementStateMask::HOVERED);

        // 解析选择器 `.card > .btn:hover`
        let sel = Selector::parse(".card > .btn:hover").unwrap();

        // 校验特异度打分：.card (10) + .btn (10) + :hover (10) = 30
        assert_eq!(sel.specificity(), 30);

        // 校验选择器精确匹配当前按钮节点
        assert!(sel.matches(btn, &tree));
        // 校验不匹配未 hover 的节点
        btn_node.state_mask.remove(ElementStateMask::HOVERED);
        assert!(!sel.matches(btn, &tree));
    }

    #[test]
    fn test_calc_solver_percentages_and_pixels() {
        // calc(50% + 10px) 在 400px 下应为 210px
        let res1 = CalcSolver::solve("calc(50% + 10px)", 400.0).unwrap();
        assert_eq!(res1, 210.0);

        // calc(100% - 32px) 在 500px 下应为 468px
        let res2 = CalcSolver::solve("calc(100% - 32px)", 500.0).unwrap();
        assert_eq!(res2, 468.0);
    }

    #[test]
    fn test_scoped_variables_inheritance() {
        let mut parent_table = VariableTable::new();
        parent_table.set("primary", "#3b82f6");
        parent_table.set("font_size", "14px");

        let parent_arc = Arc::new(parent_table);
        let mut child_table = VariableTable::child(parent_arc.clone());

        // 子节点覆盖 primary，但继承 font_size
        child_table.set("primary", "#10b981");

        assert_eq!(
            child_table.resolve("--primary"),
            Some("#10b981".to_string())
        );
        assert_eq!(child_table.resolve("--font_size"), Some("14px".to_string()));

        // 父级保持纯净，未被子级污染
        assert_eq!(parent_arc.resolve("--primary"), Some("#3b82f6".to_string()));
    }
}
