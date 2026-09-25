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
}
