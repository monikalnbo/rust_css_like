//! 跨文件模块递归导入与循环引用检测器 (Module Import Resolver)

use crate::ast::{ScopeBlock, ScopeKind};
use crate::parse_dsl;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ModuleResolver {
    visited_files: HashSet<PathBuf>,
}

impl Default for ModuleResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleResolver {
    pub fn new() -> Self {
        Self {
            visited_files: HashSet::new(),
        }
    }

    /// 递归解析 @import 语句，将所有外部模块内联展开为扁平 AST 块列表
    pub fn resolve_file(&mut self, file_path: impl AsRef<Path>) -> Result<Vec<ScopeBlock>, String> {
        let canonical = file_path
            .as_ref()
            .canonicalize()
            .map_err(|e| format!("无法定位文件 {:?}: {}", file_path.as_ref(), e))?;

        if self.visited_files.contains(&canonical) {
            // 避免循环依赖引用，直接返回空
            return Ok(Vec::new());
        }
        self.visited_files.insert(canonical.clone());

        let source = fs::read_to_string(&canonical)
            .map_err(|e| format!("读取文件 {:?} 失败: {}", canonical, e))?;

        let blocks = parse_dsl(&source)?;
        let base_dir = canonical.parent().unwrap_or_else(|| Path::new("."));
        self.resolve_blocks(blocks, base_dir)
    }

    /// 递归扫描 AST 中的 ScopeKind::Import 并展开
    pub fn resolve_blocks(
        &mut self,
        blocks: Vec<ScopeBlock>,
        base_dir: &Path,
    ) -> Result<Vec<ScopeBlock>, String> {
        let mut resolved = Vec::new();

        for block in blocks {
            if let ScopeKind::Import(rel_path) = &block.kind {
                let target_path = base_dir.join(rel_path);
                if target_path.exists() {
                    let mut imported_blocks = self.resolve_file(target_path)?;
                    resolved.append(&mut imported_blocks);
                } else {
                    return Err(format!("找不到被导入的模块文件: {:?}", target_path));
                }
            } else {
                resolved.push(block);
            }
        }

        Ok(resolved)
    }
}
