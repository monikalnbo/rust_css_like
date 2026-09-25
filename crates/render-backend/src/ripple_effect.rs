//! 交互式水波纹底层效果实现示例 (Interactive Ripple Effect)

use crate::command::DrawCommand;
use crate::display_list::DisplayList;
use crate::effect::{CustomEffect, InteractionEvent};
use layout_engine::LayoutRect;

/// 内建的水波纹自研动效
pub struct InteractiveRippleEffect {
    pub center: (f32, f32),
    pub radius: f32,
    pub intensity: f32,
}

impl Default for InteractiveRippleEffect {
    fn default() -> Self {
        Self {
            center: (0.0, 0.0),
            radius: 0.0,
            intensity: 1.0,
        }
    }
}

impl CustomEffect for InteractiveRippleEffect {
    fn name(&self) -> &str {
        "InteractiveRipple"
    }

    fn on_interaction(&mut self, event: &InteractionEvent, _bounds: LayoutRect) {
        match *event {
            InteractionEvent::CursorMoved { local_x, local_y } => {
                self.center = (local_x, local_y);
            }
            InteractionEvent::PointerDown {
                local_x, local_y, ..
            } => {
                self.center = (local_x, local_y);
                self.radius = 0.0;
                self.intensity = 1.0;
            }
            _ => {}
        }
    }

    fn update(&mut self, dt_secs: f32) {
        if self.intensity > 0.0 {
            self.radius += 200.0 * dt_secs;
            self.intensity = (self.intensity - 1.2 * dt_secs).max(0.0);
        }
    }

    fn render(&self, bounds: LayoutRect, list: &mut DisplayList) {
        if self.intensity <= 0.0 {
            return;
        }

        list.push(DrawCommand::CustomEffect {
            effect_name: self.name().to_string(),
            bounds,
            uniforms: vec![
                ("u_center_x".to_string(), self.center.0),
                ("u_center_y".to_string(), self.center.1),
                ("u_radius".to_string(), self.radius),
                ("u_intensity".to_string(), self.intensity),
            ],
        });
    }
}
