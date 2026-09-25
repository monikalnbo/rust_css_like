//! 前端自定义组件扩展槽位与插件协议 (Extension Slots & Plugin Protocol)

use crate::value::ScriptValue;
use std::collections::HashMap;
use std::sync::Arc;

/// 开发者自定义扩展组件 Trait (供开发者插入自研 Canvas、3D 视口、图表等原生能力)
pub trait CustomComponentPlugin: Send + Sync {
    /// 插件组件名称（例如 "VideoPlayer", "EChart", "GameView"）
    fn name(&self) -> &str;
    /// 触发组件的自定义脚本动作响应
    fn invoke(&self, method: &str, args: &[ScriptValue]) -> Result<ScriptValue, String>;
}

/// 全局组件与插件扩展注册中心
#[derive(Default)]
pub struct ExtensionRegistry {
    plugins: HashMap<String, Arc<dyn CustomComponentPlugin>>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册一个自定义扩展插件
    pub fn register(&mut self, plugin: Arc<dyn CustomComponentPlugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    /// 查询扩展插件
    pub fn get(&self, name: &str) -> Option<Arc<dyn CustomComponentPlugin>> {
        self.plugins.get(name).cloned()
    }

    /// 调用插件扩展方法
    pub fn call(
        &self,
        plugin_name: &str,
        method: &str,
        args: &[ScriptValue],
    ) -> Result<ScriptValue, String> {
        let plugin = self
            .get(plugin_name)
            .ok_or_else(|| format!("未找到扩展插件: {}", plugin_name))?;
        plugin.invoke(method, args)
    }
}
