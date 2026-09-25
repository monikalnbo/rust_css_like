//! BOM 字节序标记检测与常见编码探测器 (BOM & Encoding Detector)

/// 常见文本编码类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EncodingKind {
    #[default]
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16Be,
    GbkGb18030,
    Windows1252,
}

pub struct CharsetDetector;

impl CharsetDetector {
    /// 依据文件头字节序列 (Magic BOM) 高速检测编码
    pub fn detect_bom(bytes: &[u8]) -> (EncodingKind, usize) {
        if bytes.len() >= 3 && &bytes[0..3] == [0xEF, 0xBB, 0xBF] {
            // UTF-8 with BOM
            return (EncodingKind::Utf8Bom, 3);
        }
        if bytes.len() >= 2 {
            if &bytes[0..2] == [0xFF, 0xFE] {
                // UTF-16 Little Endian (Windows 默认记事本/Win32宽字符常用)
                return (EncodingKind::Utf16Le, 2);
            }
            if &bytes[0..2] == [0xFE, 0xFF] {
                // UTF-16 Big Endian
                return (EncodingKind::Utf16Be, 2);
            }
        }
        (EncodingKind::Utf8, 0)
    }

    /// 启发式检测：是否可能是 GBK / GB2312 / GB18030 (中文 Windows 经典编码)
    pub fn is_likely_gbk(bytes: &[u8]) -> bool {
        let mut i = 0;
        let mut gbk_pairs = 0;

        while i < bytes.len() {
            let b = bytes[i];
            if b <= 0x7F {
                i += 1;
                continue;
            }
            // GBK 双字节高位首字节通常在 0x81..=0xFE 之间
            if (0x81..=0xFE).contains(&b) && i + 1 < bytes.len() {
                let next = bytes[i + 1];
                // 低位尾字节通常在 0x40..=0xFE 之间 (不含 0x7F)
                if (0x40..=0xFE).contains(&next) && next != 0x7F {
                    gbk_pairs += 1;
                    i += 2;
                    continue;
                }
            }
            return false;
        }

        gbk_pairs > 0
    }
}
