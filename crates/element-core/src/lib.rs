//! # Element Core
//!
//! 对标 HTML 基础元素原语的虚拟节点树、表单控件状态机与双向绑定中枢。

pub mod form;
pub mod node;
pub mod pseudo;
pub mod state;
pub mod tag;
pub mod tree;

pub use form::{
    CheckboxState, FormControlKind, FormRegistry, RadioState, SelectState, SliderState,
};
pub use node::{ElementNode, NodeId};
pub use pseudo::{PseudoElementDef, PseudoInjector, PseudoKind};
pub use state::ElementStateMask;
pub use tag::ElementTag;
pub use tree::ElementTree;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_construction() {
        let mut tree = ElementTree::new();
        let win = tree.create_node(ElementTag::Window);
        let btn = tree.create_node(ElementTag::Button);
        tree.append_child(win, btn);

        assert_eq!(tree.node_count(), 2);
        assert_eq!(tree.get_node(btn).unwrap().parent, Some(win));
        assert_eq!(tree.get_node(win).unwrap().children, vec![btn]);
    }

    #[test]
    fn test_checkbox_toggle_and_two_way_binding() {
        let mut registry = FormRegistry::new();
        let cb_node = NodeId(101);

        registry.register(
            cb_node,
            FormControlKind::Checkbox(CheckboxState::new(false, Some("is_agreed".to_string()))),
        );

        // 第一次点击：切换为 true，并返回绑定的变量名
        let (checked, bind_var) = registry.toggle_checkbox(cb_node).unwrap();
        assert!(checked);
        assert_eq!(bind_var, Some("is_agreed".to_string()));

        // 第二次点击：切换为 false
        let (checked_again, _) = registry.toggle_checkbox(cb_node).unwrap();
        assert!(!checked_again);
    }

    #[test]
    fn test_radio_group_mutual_exclusion() {
        let mut registry = FormRegistry::new();
        let r1 = NodeId(201);
        let r2 = NodeId(202);

        registry.register(
            r1,
            FormControlKind::Radio(RadioState::new("payment", "alipay", true)),
        );
        registry.register(
            r2,
            FormControlKind::Radio(RadioState::new("payment", "wechat", false)),
        );

        // 选中微信支付，支付宝应自动互斥为 false
        let selected_val = registry.select_radio(r2).unwrap();
        assert_eq!(selected_val, "wechat");

        if let Some(FormControlKind::Radio(r)) = registry.get(r1) {
            assert!(!r.selected);
        }
        if let Some(FormControlKind::Radio(r)) = registry.get(r2) {
            assert!(r.selected);
        }
    }
}
