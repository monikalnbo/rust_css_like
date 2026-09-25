//! 组件模版宏注册与实参展开引擎 (Component Template Expander)

use crate::ast::{ScopeBlock, ScopeKind, Value};
use std::collections::HashMap;

pub struct ComponentExpander {
    templates: HashMap<String, (Vec<String>, Vec<ScopeBlock>)>,
}

impl Default for ComponentExpander {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentExpander {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// 提取并注册所有 component 模版定义
    pub fn register_templates(&mut self, blocks: &[ScopeBlock]) {
        for block in blocks {
            if let ScopeKind::ComponentDef { name, params } = &block.kind {
                self.templates
                    .insert(name.clone(), (params.clone(), block.children.clone()));
            }
        }
    }

    /// 递归遍历 AST，将自定义组件调用展开为真实虚拟 DOM 节点树
    pub fn expand_tree(&self, blocks: Vec<ScopeBlock>) -> Vec<ScopeBlock> {
        let mut expanded = Vec::with_capacity(blocks.len());

        for mut block in blocks {
            // 如果当前块是组件定义本身，保留或跳过
            if matches!(block.kind, ScopeKind::ComponentDef { .. }) {
                continue;
            }

            if let ScopeKind::Element(tag_name) = &block.kind {
                if let Some((params, body)) = self.templates.get(tag_name) {
                    // 找到组件模板，执行实参形参映射替换
                    let mut arg_map = HashMap::new();
                    for prop in &block.properties {
                        arg_map.insert(prop.name.clone(), prop.value.clone());
                    }

                    for template_child in body {
                        let mut instantiated = template_child.clone();
                        self.substitute_params(&mut instantiated, params, &arg_map);
                        expanded.push(instantiated);
                    }
                    continue;
                }
            }

            // 递归展开子节点
            block.children = self.expand_tree(block.children);
            expanded.push(block);
        }

        expanded
    }

    fn substitute_params(
        &self,
        block: &mut ScopeBlock,
        _params: &[String],
        arg_map: &HashMap<String, Value>,
    ) {
        for prop in &mut block.properties {
            if let Value::Variable(v) = &prop.value {
                if let Some(concrete_val) = arg_map.get(v) {
                    prop.value = concrete_val.clone();
                }
            }
        }

        for child in &mut block.children {
            self.substitute_params(child, _params, arg_map);
        }
    }
}
