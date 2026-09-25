//! .binui 离线打包与解包器 (BinUiPacker)
//!
//! 将明文 DSL 源码进行符号混淆、动态流加密，并封装为带防篡改校验头的 .binui 二进制包。

use crate::bytecode::BinaryPackage;
use crate::crypto::StreamCrypto;
use crate::obfuscate::Obfuscator;

/// 二进制打包选项
#[derive(Clone, Debug)]
pub struct PackOptions {
    pub key: Option<[u8; 32]>,
    pub obfuscate_identifiers: bool,
}

impl Default for PackOptions {
    fn default() -> Self {
        Self {
            key: None,
            obfuscate_identifiers: false,
        }
    }
}

/// 生产级 .binui 二进制打包解包中枢
pub struct BinUiPacker;

impl BinUiPacker {
    /// 将明文 UI DSL 打包为 .binui 字节流
    pub fn pack(source: &str, options: &PackOptions) -> Vec<u8> {
        let mut processed = source.to_string();

        // 1. 若开启标识符混淆，对源码中的变量进行初级脱敏
        if options.obfuscate_identifiers {
            let mut words: Vec<String> = Vec::new();
            for line in processed.lines() {
                let mut line_words = Vec::new();
                for w in line.split_whitespace() {
                    if w.starts_with('$') && w.len() > 1 {
                        line_words.push(format!("${}", Obfuscator::mangle(&w[1..])));
                    } else {
                        line_words.push(w.to_string());
                    }
                }
                words.push(line_words.join(" "));
            }
            processed = words.join("\n");
        }

        let mut payload = processed.into_bytes();

        // 2. 若配置了加密密钥，执行流加密
        if let Some(key) = options.key {
            let crypto = StreamCrypto::new(key);
            payload = crypto.transform(&payload);
        }

        // 3. 构建 BinaryPackage 并序列化
        let package = BinaryPackage::new(payload);
        package.serialize()
    }

    /// 解包 .binui 字节流
    pub fn unpack(bytes: &[u8], key: Option<[u8; 32]>) -> Result<String, String> {
        let package = BinaryPackage::deserialize(bytes)?;
        let payload = if let Some(k) = key {
            let crypto = StreamCrypto::new(k);
            crypto.transform(&package.payload)
        } else {
            package.payload
        };

        String::from_utf8(payload).map_err(|e| format!("UTF-8 解码错误: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_and_unpack_roundtrip() {
        let source = "win \"Title\" { btn \"Submit\" }";
        let key = [0x5au8; 32];
        let options = PackOptions {
            key: Some(key),
            obfuscate_identifiers: false,
        };

        let packed = BinUiPacker::pack(source, &options);
        assert!(!packed.is_empty());
        assert_eq!(&packed[0..4], &crate::bytecode::BYTECODE_MAGIC);

        let unpacked = BinUiPacker::unpack(&packed, Some(key)).unwrap();
        assert_eq!(unpacked, source);
    }
}
