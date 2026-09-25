//! # DSL Parser
//!
//! 基于 `{}` 约束符的作用域解析器与实时增量分析引擎。
//! 全面支持：@import 模块化、组件模版 (component)、控制流 (for/if/else)、动作流箭头 (->) 与字符集兼容。

pub mod ast;
pub mod bracket;
pub mod component_expander;
pub mod decl_parser;
pub mod lexer;
pub mod module_resolver;
pub mod parser;
pub mod scan_helpers;
pub mod span;
pub mod token;

pub use ast::{PropertyDecl, ScopeBlock, ScopeKind, Value};
pub use bracket::{BracketEvent, BracketTracker};
pub use component_expander::ComponentExpander;
pub use lexer::Lexer;
pub use module_resolver::ModuleResolver;
pub use parser::Parser;
pub use span::Span;
pub use token::{Token, TokenKind};

/// 解析 UTF-8 文本源码
pub fn parse_dsl(source: &str) -> Result<Vec<ScopeBlock>, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// 解析任意编码的原始二进制字节流（自动探测 BOM、转码 GBK/UTF-16 并标准化）
pub fn parse_bytes(raw_bytes: &[u8]) -> Result<Vec<ScopeBlock>, String> {
    let normalized = charset_compat::normalize_to_utf8(raw_bytes);
    parse_dsl(&normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_advanced_syntax() {
        let code = r#"
            @import "components/header.ui";
            let count = 0;

            component Card(title) {
                box {
                    txt title;
                }
            }

            window {
                for item in tasks {
                    btn "点击" -> count += 1;
                }
            }
        "#;
        let blocks = parse_dsl(code).unwrap();
        assert_eq!(blocks.len(), 4);
        assert_eq!(
            blocks[0].kind,
            ScopeKind::Import("components/header.ui".to_string())
        );
        assert!(matches!(blocks[1].kind, ScopeKind::StateLet { .. }));
        assert!(matches!(blocks[2].kind, ScopeKind::ComponentDef { .. }));
        assert_eq!(blocks[3].kind, ScopeKind::Window);
        assert_eq!(blocks[3].children.len(), 1);
        assert!(matches!(
            blocks[3].children[0].kind,
            ScopeKind::ForLoop { .. }
        ));
    }

    #[test]
    fn test_component_expander() {
        let code = r#"
            component Card(title) {
                box {
                    txt: $title;
                }
            }

            window {
                Card {
                    title: "Hello Card";
                }
            }
        "#;
        let blocks = parse_dsl(code).unwrap();
        let mut expander = ComponentExpander::new();
        expander.register_templates(&blocks);
        let expanded = expander.expand_tree(blocks);

        // Card 调用被成功展开为真实的 box 虚拟节点
        assert_eq!(expanded.len(), 1); // window
        assert_eq!(expanded[0].children.len(), 1); // box
        assert_eq!(
            expanded[0].children[0].kind,
            ScopeKind::Element("box".to_string())
        );
    }

    #[test]
    fn test_parse_counter_ui_official() {
        let code = include_str!("../../../examples/counter.ui");
        let blocks = parse_dsl(code).expect("Failed to parse counter.ui");

        // 验证两个顶层块：let count = 0 与 win "极简计数器应用"
        assert_eq!(blocks.len(), 2);

        // 1. 验证 state let
        match &blocks[0].kind {
            ScopeKind::StateLet { name, init_expr } => {
                assert_eq!(name, "count");
                assert_eq!(init_expr, "0");
            }
            other => panic!("Expected StateLet, got {:?}", other),
        }

        // 2. 验证 win
        assert_eq!(blocks[1].kind, ScopeKind::Window);
        let win = &blocks[1];
        let title_prop = win.properties.iter().find(|p| p.name == "title").unwrap();
        assert_eq!(title_prop.value, Value::String("极简计数器应用".to_string()));
        let w_prop = win.properties.iter().find(|p| p.name == "width").unwrap();
        assert_eq!(w_prop.value, Value::Number(400.0));
        let h_prop = win.properties.iter().find(|p| p.name == "height").unwrap();
        assert_eq!(h_prop.value, Value::Number(300.0));
        let bg_prop = win.properties.iter().find(|p| p.name == "bg").unwrap();
        assert_eq!(bg_prop.value, Value::HexColor("#0f172a".to_string()));

        // 3. 验证 col 容器
        assert_eq!(win.children.len(), 1);
        let col = &win.children[0];
        assert_eq!(col.kind, ScopeKind::Element("col".to_string()));
        let pad_prop = col.properties.iter().find(|p| p.name == "pad").unwrap();
        assert_eq!(pad_prop.value, Value::Number(32.0));
        let gap_prop = col.properties.iter().find(|p| p.name == "gap").unwrap();
        assert_eq!(gap_prop.value, Value::Number(20.0));
        let align_prop = col.properties.iter().find(|p| p.name == "align").unwrap();
        assert_eq!(align_prop.value, Value::Ident("center".to_string()));
        let justify_prop = col.properties.iter().find(|p| p.name == "justify").unwrap();
        assert_eq!(justify_prop.value, Value::Ident("center".to_string()));
        let flex_prop = col.properties.iter().find(|p| p.name == "flex").unwrap();
        assert_eq!(flex_prop.value, Value::Number(1.0));

        // 4. 验证 col 的子节点: 2 个 txt 与 1 个 row
        assert_eq!(col.children.len(), 3);
        let txt1 = &col.children[0];
        assert_eq!(txt1.kind, ScopeKind::Element("txt".to_string()));
        assert_eq!(
            txt1.properties.iter().find(|p| p.name == "text").unwrap().value,
            Value::String("极简前端语言演示".to_string())
        );
        assert_eq!(
            txt1.properties.iter().find(|p| p.name == "color").unwrap().value,
            Value::HexColor("#94a3b8".to_string())
        );
        assert_eq!(
            txt1.properties.iter().find(|p| p.name == "font_size").unwrap().value,
            Value::String("14px".to_string())
        );

        let txt2 = &col.children[1];
        assert_eq!(txt2.kind, ScopeKind::Element("txt".to_string()));
        assert_eq!(
            txt2.properties.iter().find(|p| p.name == "text").unwrap().value,
            Value::String("当前计数: $count".to_string())
        );
        assert_eq!(
            txt2.properties.iter().find(|p| p.name == "font_size").unwrap().value,
            Value::String("28px".to_string())
        );
        assert_eq!(
            txt2.properties.iter().find(|p| p.name == "font_weight").unwrap().value,
            Value::Ident("bold".to_string())
        );

        // 5. 验证 row 与其子按钮
        let row = &col.children[2];
        assert_eq!(row.kind, ScopeKind::Element("row".to_string()));
        assert_eq!(
            row.properties.iter().find(|p| p.name == "gap").unwrap().value,
            Value::Number(12.0)
        );
        assert_eq!(row.children.len(), 3);

        let btn_dec = &row.children[0];
        assert_eq!(btn_dec.kind, ScopeKind::Element("btn".to_string()));
        assert_eq!(
            btn_dec.properties.iter().find(|p| p.name == "text").unwrap().value,
            Value::String("减少 (-1)".to_string())
        );
        assert_eq!(
            btn_dec.properties.iter().find(|p| p.name == "on_click").unwrap().value,
            Value::ActionCode("count -= 1".to_string())
        );

        let btn_reset = &row.children[1];
        assert_eq!(
            btn_reset.properties.iter().find(|p| p.name == "on_click").unwrap().value,
            Value::ActionCode("count = 0".to_string())
        );

        let btn_inc = &row.children[2];
        assert_eq!(
            btn_inc.properties.iter().find(|p| p.name == "on_click").unwrap().value,
            Value::ActionCode("count += 1".to_string())
        );
    }

    #[test]
    fn test_parse_app_ui_official() {
        let code = include_str!("../../../examples/app.ui");
        let blocks = parse_dsl(code).expect("Failed to parse app.ui");

        // 验证 6 个 let 顶层状态与 1 个 win
        assert_eq!(blocks.len(), 7);
        assert!(matches!(blocks[0].kind, ScopeKind::StateLet { .. }));
        assert!(matches!(blocks[4].kind, ScopeKind::StateLet { .. }));
        assert_eq!(blocks[6].kind, ScopeKind::Window);

        let win = &blocks[6];
        assert_eq!(win.children.len(), 2); // row (navbar) and col (main content)
    }

    #[test]
    fn test_component_call_args() {
        let code = r#"
            component StatCard(title, value) {
                box pad=16 bg=#1e293b {
                    txt "$title";
                    txt "$value";
                }
            }

            win {
                StatCard(title="Active Users", value="12,480")
            }
        "#;
        let blocks = parse_dsl(code).expect("Failed to parse component call syntax");
        assert_eq!(blocks.len(), 2); // component def and win

        let mut expander = ComponentExpander::new();
        expander.register_templates(&blocks);
        let expanded = expander.expand_tree(blocks);

        assert_eq!(expanded.len(), 1); // win
        assert_eq!(expanded[0].children.len(), 1); // expanded box
        let box_node = &expanded[0].children[0];
        assert_eq!(box_node.kind, ScopeKind::Element("box".to_string()));
        assert_eq!(box_node.children.len(), 2);
        assert_eq!(
            box_node.children[0].properties.iter().find(|p| p.name == "text").unwrap().value,
            Value::String("Active Users".to_string())
        );
        assert_eq!(
            box_node.children[1].properties.iter().find(|p| p.name == "text").unwrap().value,
            Value::String("12,480".to_string())
        );
    }
}
