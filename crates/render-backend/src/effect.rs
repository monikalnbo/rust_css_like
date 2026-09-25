//! 自定义扩展效果协议与底层交互响应中枢 (Custom Effects & Low-Level Interaction)

use crate::display_list::DisplayList;

use layout_engine::LayoutRect;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 用户输入底层物理交互事件
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionEvent {
    /// 鼠标移动 (传递相对于当前组件左上角的局部坐标 local_x, local_y)
    CursorMoved { local_x: f32, local_y: f32 },
    /// 鼠标按下点击
    PointerDown {
        local_x: f32,
        local_y: f32,
        button: u8,
    },
    /// 鼠标抬起
    PointerUp { local_x: f32, local_y: f32 },
    /// 鼠标滚轮物理微动
    Wheel { delta_x: f32, delta_y: f32 },
}

/// 开发者自研效果必须实现的核心底层 Trait
pub trait CustomEffect: Send + Sync {
    /// 效果全局唯一标识符
    fn name(&self) -> &str;

    /// 接收底层真实硬件交互事件
    fn on_interaction(&mut self, event: &InteractionEvent, bounds: LayoutRect);

    /// 每帧时间推进更新
    fn update(&mut self, dt_secs: f32);

    /// 渲染阶段：向 DisplayList 注入自定义图元或 Shader Uniform 指令
    fn render(&self, bounds: LayoutRect, list: &mut DisplayList);
}

/// 全局特效注册与交互派发中心
#[derive(Default)]
pub struct EffectRegistry {
    effects: Mutex<HashMap<String, Arc<Mutex<dyn CustomEffect>>>>,
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册一个全新的自研底层效果
    pub fn register<E: CustomEffect + 'static>(&self, effect: E) {
        let name = effect.name().to_string();
        self.effects
            .lock()
            .unwrap()
            .insert(name, Arc::new(Mutex::new(effect)));
    }

    /// 向指定效果派发底层交互事件
    pub fn dispatch_interaction(
        &self,
        effect_name: &str,
        event: &InteractionEvent,
        bounds: LayoutRect,
    ) {
        if let Some(effect) = self.effects.lock().unwrap().get(effect_name) {
            effect.lock().unwrap().on_interaction(event, bounds);
        }
    }

    /// 渲染注册效果图元
    pub fn render_effect(&self, effect_name: &str, bounds: LayoutRect, list: &mut DisplayList) {
        if let Some(effect) = self.effects.lock().unwrap().get(effect_name) {
            effect.lock().unwrap().render(bounds, list);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::DrawCommand;

    #[test]
    fn test_custom_effect_interaction_and_render() {
        let registry = EffectRegistry::new();
        registry.register(InteractiveRippleEffect::default());

        let bounds = LayoutRect::new(0.0, 0.0, 100.0, 50.0);
        let mut list = DisplayList::new();

        registry.dispatch_interaction(
            "InteractiveRipple",
            &InteractionEvent::PointerDown {
                local_x: 50.0,
                local_y: 25.0,
                button: 0,
            },
            bounds,
        );

        registry.render_effect("InteractiveRipple", bounds, &mut list);
        assert_eq!(list.len(), 1);

        if let DrawCommand::CustomEffect {
            effect_name,
            uniforms,
            ..
        } = &list.commands()[0]
        {
            assert_eq!(effect_name, "InteractiveRipple");
            assert_eq!(uniforms[0].1, 50.0);
        }
    }
}
