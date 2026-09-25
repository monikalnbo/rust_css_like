//! 图像资产与矢量 SVG 渲染管线 (Image & SVG Pipeline)
//!
//! 对标 HTML <img> 与 <svg> 规范，管理位图像素缓存池与矢量路径解析。

use crate::command::DrawCommand;
use css_types::Color;
use layout_engine::LayoutRect;
use std::collections::HashMap;

/// 解码后的未压缩 RGBA 原始位图
#[derive(Clone, Debug)]
pub struct RawImage {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub rgba_pixels: Vec<u8>,
}

/// 矢量 SVG 路径分段图元
#[derive(Clone, Debug, PartialEq)]
pub enum SvgPathSegment {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    QuadTo(f32, f32, f32, f32),
    CubicTo(f32, f32, f32, f32, f32, f32),
    Close,
}

/// 图像与矢量资源中枢管线
#[derive(Default)]
pub struct ImagePipeline {
    images: HashMap<u32, RawImage>,
    next_image_id: u32,
}

impl ImagePipeline {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            next_image_id: 1,
        }
    }

    /// 注册 RGBA 图像到资源缓存池
    pub fn register_rgba(&mut self, width: u32, height: u32, pixels: Vec<u8>) -> u32 {
        let id = self.next_image_id;
        self.next_image_id += 1;
        self.images.insert(
            id,
            RawImage {
                id,
                width,
                height,
                rgba_pixels: pixels,
            },
        );
        id
    }

    pub fn get_image(&self, id: u32) -> Option<&RawImage> {
        self.images.get(&id)
    }

    /// 创建绘制图像指令
    pub fn create_image_cmd(&self, image_id: u32, bounds: LayoutRect, opacity: f32) -> DrawCommand {
        DrawCommand::DrawImage {
            image_id,
            bounds,
            opacity: opacity.clamp(0.0, 1.0),
        }
    }

    /// 解析 SVG Path 字符串为绘制指令
    pub fn create_svg_cmd(
        &self,
        path_data: impl Into<String>,
        bounds: LayoutRect,
        fill: Option<Color>,
        stroke: Option<Color>,
        stroke_width: f32,
    ) -> DrawCommand {
        DrawCommand::DrawSvgPath {
            path_data: path_data.into(),
            bounds,
            fill_color: fill,
            stroke_color: stroke,
            stroke_width,
        }
    }
}

/// 简易 SVG 路径命令解析器
pub struct SvgPathParser;

impl SvgPathParser {
    pub fn parse(d: &str) -> Vec<SvgPathSegment> {
        let mut segments = Vec::new();
        let tokens: Vec<&str> = d.split_whitespace().collect();
        let mut i = 0;

        while i < tokens.len() {
            match tokens[i] {
                "M" | "m" if i + 2 < tokens.len() => {
                    let x = tokens[i + 1].parse().unwrap_or(0.0);
                    let y = tokens[i + 2].parse().unwrap_or(0.0);
                    segments.push(SvgPathSegment::MoveTo(x, y));
                    i += 3;
                }
                "L" | "l" if i + 2 < tokens.len() => {
                    let x = tokens[i + 1].parse().unwrap_or(0.0);
                    let y = tokens[i + 2].parse().unwrap_or(0.0);
                    segments.push(SvgPathSegment::LineTo(x, y));
                    i += 3;
                }
                "Q" | "q" if i + 4 < tokens.len() => {
                    let x1 = tokens[i + 1].parse().unwrap_or(0.0);
                    let y1 = tokens[i + 2].parse().unwrap_or(0.0);
                    let x = tokens[i + 3].parse().unwrap_or(0.0);
                    let y = tokens[i + 4].parse().unwrap_or(0.0);
                    segments.push(SvgPathSegment::QuadTo(x1, y1, x, y));
                    i += 5;
                }
                "Z" | "z" => {
                    segments.push(SvgPathSegment::Close);
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        segments
    }
}
