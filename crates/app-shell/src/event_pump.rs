//! 原生窗口事件循环泵与分发器 (Event Loop Pump)

use crate::pipeline::UiPipeline;
use crate::window::NativeWindow;
use tiny_skia::Pixmap;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct EventPump {
    pub pipeline: UiPipeline,
}

impl EventPump {
    pub fn new(pipeline: UiPipeline) -> Self {
        Self { pipeline }
    }

    /// 启动应用主事件循环
    pub fn run(
        mut self,
        event_loop: EventLoop<()>,
        mut native_window: NativeWindow,
    ) -> Result<(), Box<dyn std::error::Error>> {
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event, window_id }
                    if window_id == native_window.window.id() =>
                {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::Resized(size) => {
                            native_window.resize(size.width, size.height);
                            self.pipeline
                                .layout_and_paint(size.width as f32, size.height as f32);
                            native_window.window.request_redraw();
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            // 鼠标移动检测
                            let _ = position;
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            if button == MouseButton::Left && state == ElementState::Pressed {
                                native_window.window.request_redraw();
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let width = native_window.width;
                            let height = native_window.height;
                            if let Some(mut pixmap) = Pixmap::new(width, height) {
                                self.pipeline.render_to_pixmap(&mut pixmap.as_mut());
                                let _ = native_window.present_pixmap(&pixmap);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })?;
        Ok(())
    }
}
