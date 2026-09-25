//! # Layout Engine
//!
//! 基于 Taffy 布局算法的绝对物理几何解算与整树递归排版模块。

pub mod bridge;
pub mod solver;
pub mod text_measure;
pub mod tree_solver;

pub use bridge::LayoutBridge;
pub use solver::LayoutRect;
pub use text_measure::{IntrinsicSize, TextMeasurer};
pub use tree_solver::LayoutEngineContext;

#[cfg(test)]
mod tests {
    use super::*;
    use css_types::{Dimension, Display, FlexDirection, Size};
    use element_core::{ElementTag, ElementTree};
    use std::collections::HashMap;
    use style_system::ComputedStyle;

    #[test]
    fn test_hit_test() {
        let rect = LayoutRect::new(10.0, 10.0, 100.0, 50.0);
        assert!(rect.contains(20.0, 20.0));
        assert!(!rect.contains(5.0, 20.0));
    }

    #[test]
    fn test_nested_tree_layout_and_resize() {
        let mut tree = ElementTree::new();
        let win = tree.create_node(ElementTag::Window);
        let col = tree.create_node(ElementTag::Box);
        let row1 = tree.create_node(ElementTag::Box);
        let row2 = tree.create_node(ElementTag::Box);

        tree.append_child(win, col);
        tree.append_child(col, row1);
        tree.append_child(col, row2);

        let mut styles = HashMap::new();

        // 窗口全屏自适应
        let mut win_style = ComputedStyle::default();
        win_style.size = Size::new(Dimension::Percent(100.0), Dimension::Percent(100.0));
        styles.insert(win, win_style);

        // col 列容器
        let mut col_style = ComputedStyle::default();
        col_style.display = Display::Flex;
        col_style.flex_direction = FlexDirection::Column;
        col_style.size = Size::new(Dimension::Percent(100.0), Dimension::Percent(100.0));
        styles.insert(col, col_style);

        // row1 与 row2 各固定高度 50px，宽度 100%
        let mut row_style = ComputedStyle::default();
        row_style.size = Size::new(Dimension::Percent(100.0), Dimension::Px(50.0));
        styles.insert(row1, row_style.clone());
        styles.insert(row2, row_style);

        let mut engine = LayoutEngineContext::new();

        // 第一次求解：800 x 600
        let root_taffy = engine.build_taffy_tree(&tree, &styles).unwrap();
        engine.compute_layout(root_taffy, 800.0, 600.0).unwrap();
        let rects_800 = engine.collect_layout_rects(&tree);

        assert_eq!(rects_800[&win].width, 800.0);
        assert_eq!(rects_800[&row1].width, 800.0);
        assert_eq!(rects_800[&row1].height, 50.0);
        assert_eq!(rects_800[&row2].y, 50.0);

        // 第二次拉伸重排：1200 x 800
        engine.compute_layout(root_taffy, 1200.0, 800.0).unwrap();
        let rects_1200 = engine.collect_layout_rects(&tree);

        assert_eq!(rects_1200[&win].width, 1200.0);
        assert_eq!(rects_1200[&row1].width, 1200.0);
        assert_eq!(rects_1200[&row2].width, 1200.0);
    }
}
