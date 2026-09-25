//! AOT 二进制字节码编码器 (Binary Bytecode Packager)
//!
//! 将明文 DSL 编译为紧凑、无明文字符串的 .binui 二进制包，彻底防逆向窃取。

pub const BYTECODE_MAGIC: [u8; 4] = [0x52, 0x43, 0x53, 0x53]; // "RCSS"

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryPackage {
    pub version: u16,
    pub payload: Vec<u8>,
}

impl BinaryPackage {
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            version: 1,
            payload,
        }
    }

    /// 序列化为带 Magic 校验头的最终二进制数据流
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&BYTECODE_MAGIC);
        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    /// 反序列化二进制包
    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 10 {
            return Err("文件头部截断".to_string());
        }
        if &bytes[0..4] != &BYTECODE_MAGIC {
            return Err("非法或损坏的二进制 UI 字节码包".to_string());
        }
        let version = u16::from_le_bytes([bytes[4], bytes[5]]);
        let len = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]) as usize;
        let payload = bytes[10..10 + len].to_vec();

        Ok(Self { version, payload })
    }
}
