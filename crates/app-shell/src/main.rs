//! Rust CSS-Like 桌面宿主主入口 (App Shell Main Entry)

pub mod event_pump;
pub mod pipeline;
pub mod window;

use event_pump::EventPump;
use pipeline::UiPipeline;
use std::fs;
use std::path::Path;
use window::NativeWindow;
use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(">>> 启动 Rust CSS-Like 原生渲染运行时外壳 (app-shell)...");

    let event_loop = EventLoop::new()?;
    let native_window = NativeWindow::new(&event_loop, "Rust CSS-Like Native Runtime", 960, 640)?;

    let mut pipeline = UiPipeline::new();

    // 读取 examples/app.ui 源码，若无则使用内建默认骨架
    let dsl_path = Path::new("examples/app.ui");
    let dsl_source = if dsl_path.exists() {
        fs::read_to_string(dsl_path).unwrap_or_else(|_| default_dsl().to_string())
    } else {
        default_dsl().to_string()
    };

    if let Err(e) = pipeline.load_dsl(&dsl_source, 960.0, 640.0) {
        eprintln!("[警告] DSL 加载警告，使用安全回退布局: {}", e);
        pipeline.layout_and_paint(960.0, 640.0);
    }

    let event_pump = EventPump::new(pipeline);
    event_pump.run(event_loop, native_window)?;

    Ok(())
}

fn default_dsl() -> &'static str {
    r#"
    window {
        card {
            txt "Hello Rust CSS-Like Native GUI";
            btn {
                txt "Click Me";
            }
        }
    }
    "#
}
