//! 绘制指令队列与显示列表构建器 (Display List)

use crate::command::DrawCommand;

/// 扁平化的单帧 GPU 绘制指令列表
#[derive(Clone, Debug, Default)]
pub struct DisplayList {
    commands: Vec<DrawCommand>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}
