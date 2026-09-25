//! # Charset Compat
//!
//! 跨平台多字符集兼容引擎：BOM 探测、GBK/GB18030 容错转码、Windows 宽字符互操作与 CJK 全半角排版计算。

pub mod detector;
pub mod grapheme;
pub mod transcoder;
pub mod win32;

pub use detector::{CharsetDetector, EncodingKind};
pub use grapheme::UnicodeMetrics;
pub use transcoder::Transcoder;
pub use win32::Win32String;

/// 一键将任意字符集字节流规范化转码为安全有效的 Rust UTF-8 字符串
pub fn normalize_to_utf8(bytes: &[u8]) -> String {
    Transcoder::to_utf8(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_bom_strip() {
        // 带 UTF-8 BOM 的 "Hello"
        let data = [0xEF, 0xBB, 0xBF, b'H', b'e', b'l', b'l', b'o'];
        let result = normalize_to_utf8(&data);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_utf16_le_decoding() {
        // UTF-16LE with BOM: "Hi"
        let data = [0xFF, 0xFE, b'H', 0x00, b'i', 0x00];
        let result = normalize_to_utf8(&data);
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_cjk_width() {
        assert_eq!(UnicodeMetrics::display_width("Hello"), 5);
        assert_eq!(UnicodeMetrics::display_width("你好"), 4);
        assert_eq!(UnicodeMetrics::display_width("A中文B"), 6);
    }
}
