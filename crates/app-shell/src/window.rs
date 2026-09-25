//! 操作系统原生视窗封装与软缓冲 Surface 呈现器 (Window & Softbuffer Surface)

use std::num::NonZeroU32;
use std::rc::Rc;
use tiny_skia::Pixmap;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowBuilder};

pub struct NativeWindow {
    pub window: Rc<Window>,
    pub surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    pub width: u32,
    pub height: u32,
}

impl NativeWindow {
    /// 新建原生窗口与软缓冲区表面
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        title: &str,
        default_width: u32,
        default_height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Rc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(LogicalSize::new(
                    default_width as f64,
                    default_height as f64,
                ))
                .build(event_loop)?,
        );

        let context = softbuffer::Context::new(window.clone())?;
        let surface = softbuffer::Surface::new(&context, window.clone())?;

        Ok(Self {
            window,
            surface,
            width: default_width,
            height: default_height,
        })
    }

    /// 窗口尺寸变更时重新配置表面
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.width = width;
        self.height = height;
        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            let _ = self.surface.resize(w, h);
        }
    }

    /// 将渲染完毕的 tiny-skia Pixmap 复制到 softbuffer 帧缓冲区并呈现到屏幕
    pub fn present_pixmap(&mut self, pixmap: &Pixmap) -> Result<(), Box<dyn std::error::Error>> {
        if let (Some(w), Some(h)) = (NonZeroU32::new(self.width), NonZeroU32::new(self.height)) {
            self.surface.resize(w, h)?;
            let mut buffer = self.surface.buffer_mut()?;
            let pixels = pixmap.data();

            for (i, chunk) in pixels.chunks_exact(4).enumerate() {
                if i >= buffer.len() {
                    break;
                }
                // softbuffer 像素为 0x00RRGGBB 格式
                let r = chunk[0] as u32;
                let g = chunk[1] as u32;
                let b = chunk[2] as u32;
                buffer[i] = (r << 16) | (g << 8) | b;
            }

            buffer.present()?;
        }
        Ok(())
    }
}
