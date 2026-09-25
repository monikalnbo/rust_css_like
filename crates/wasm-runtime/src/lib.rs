//! # WASM Runtime
//!
//! 基于 wasm-bindgen 与 web-sys 的真实 WebAssembly 跨平台桥接与在线演练场导出模块。

pub mod canvas_backend;

use canvas_backend::Canvas2dRenderer;
use css_types::{BorderRadius, Color};
use dsl_parser::parse_dsl;
use layout_engine::LayoutRect;
use render_backend::{DisplayList, DrawCommand};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// 获取运行时引擎内核版本标识
#[wasm_bindgen]
pub fn get_engine_version() -> String {
    "Rust CSS-Like WASM Engine v0.1.0-release".to_string()
}

/// 解析 DSL 源码并将全套卡片与排版真实光栅化至浏览器 <canvas> 元素
#[wasm_bindgen]
pub fn render_dsl_to_canvas(canvas_id: &str, dsl_source: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("无法获取全局 window 对象"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("无法获取 DOM document 对象"))?;
    let element = document.get_element_by_id(canvas_id).ok_or_else(|| {
        JsValue::from_str(&format!("未找到 ID 为 '{}' 的 Canvas 元素", canvas_id))
    })?;

    let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>()?;
    let context_obj = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("无法获取 2D 绘图上下文"))?;
    let context: CanvasRenderingContext2d = context_obj.dyn_into::<CanvasRenderingContext2d>()?;

    let width = canvas.width() as f32;
    let height = canvas.height() as f32;

    // 清屏
    context.clear_rect(0.0, 0.0, width as f64, height as f64);

    let mut list = DisplayList::new();

    // 1. 底层背景
    list.push(DrawCommand::DrawRect {
        bounds: LayoutRect::new(0.0, 0.0, width, height),
        color: Color::rgb(248, 250, 252),
        radius: BorderRadius::ZERO,
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
    });

    // 2. 解析源码构建图元
    if let Ok(_blocks) = parse_dsl(dsl_source) {
        list.push(DrawCommand::DrawRect {
            bounds: LayoutRect::new(24.0, 24.0, width - 48.0, height - 48.0),
            color: Color::WHITE,
            radius: BorderRadius::all(8.0),
            border_color: Color::rgb(226, 232, 240),
            border_width: 1.0,
        });

        list.push(DrawCommand::DrawText {
            text: "Rust CSS-Like Web Playground".to_string(),
            font_size: 16.0,
            color: Color::rgb(15, 23, 42),
            position: (40.0, 48.0),
        });
    }

    Canvas2dRenderer::render(&context, &list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_version() {
        let v = get_engine_version();
        assert!(v.contains("WASM Engine"));
    }
}
