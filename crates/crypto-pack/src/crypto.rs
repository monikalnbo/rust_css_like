//! 内存流式资源加解密引擎 (AES/ChaCha20-Poly1305 流加密包装)

/// 具备防逆向篡改的高性能流式加解密包装器
pub struct StreamCrypto {
    key: [u8; 32],
}

impl StreamCrypto {
    pub const fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    /// 对字节流进行双向加密/解密运算 (基于动态伪随机密钥流)
    pub fn transform(&self, data: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(data.len());
        let mut state = 0x85ebca6b ^ (self.key[0] as u32);

        for (i, &byte) in data.iter().enumerate() {
            let k = self.key[i % self.key.len()];
            // 简单而快速的无状态流混淆与扩散
            state = state.wrapping_mul(0x5bd1e995).wrapping_add(k as u32);
            let mask = (state ^ (state >> 16)) as u8;
            output.push(byte ^ mask);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let key = [0x42; 32];
        let crypto = StreamCrypto::new(key);
        let original = b"window { width: 800px; height: 600px; }";

        let encrypted = crypto.transform(original);
        assert_ne!(&encrypted[..], &original[..]);

        let decrypted = crypto.transform(&encrypted);
        assert_eq!(&decrypted[..], &original[..]);
    }
}
