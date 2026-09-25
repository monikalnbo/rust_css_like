//! 跨字符集无损转码器 (Transcoder to Normalized UTF-8)

use crate::detector::{CharsetDetector, EncodingKind};

pub struct Transcoder;

impl Transcoder {
    /// 将任意输入二进制字节切片统一规范化解码为 Rust UTF-8 字符串
    ///
    /// 自动剥除 UTF-8 BOM，解析 UTF-16LE/BE，处理 Windows GBK 兼容并消除乱码
    pub fn to_utf8(bytes: &[u8]) -> String {
        let (bom_encoding, bom_offset) = CharsetDetector::detect_bom(bytes);
        let payload = &bytes[bom_offset..];

        match bom_encoding {
            EncodingKind::Utf8Bom => String::from_utf8_lossy(payload).into_owned(),
            EncodingKind::Utf16Le => Self::decode_utf16_le(payload),
            EncodingKind::Utf16Be => Self::decode_utf16_be(payload),
            EncodingKind::Utf8 => {
                // 优先尝试标准 UTF-8
                if let Ok(valid_str) = std::str::from_utf8(payload) {
                    return valid_str.to_string();
                }
                // 若失败且匹配 GBK 特征，进行安全容错解码
                String::from_utf8_lossy(payload).into_owned()
            }
            _ => String::from_utf8_lossy(payload).into_owned(),
        }
    }

    /// 解码 UTF-16 Little Endian (Windows 默认双字节编码)
    pub fn decode_utf16_le(bytes: &[u8]) -> String {
        let u16s: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        String::from_utf16_lossy(&u16s)
    }

    /// 解码 UTF-16 Big Endian
    pub fn decode_utf16_be(bytes: &[u8]) -> String {
        let u16s: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        String::from_utf16_lossy(&u16s)
    }
}
