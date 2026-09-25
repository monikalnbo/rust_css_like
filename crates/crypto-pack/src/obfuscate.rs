//! 符号混淆与标识符哈希脱敏器 (Symbol Obfuscator)

/// 快速 64 位非加密哈希算法 (FNV-1a)，用于将类名、变量名、组件标识彻底脱敏
pub fn hash_symbol(ident: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in ident.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// 混淆结构体：映射源码变量名为不可读的混淆符号
pub struct Obfuscator;

impl Obfuscator {
    /// 将人类可读的字符串（如 "btn_submit", "user_password"）转换为混淆标识
    pub fn mangle(name: &str) -> String {
        let hash = hash_symbol(name);
        format!("_0x{:x}", hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangling() {
        let m1 = Obfuscator::mangle("user_password");
        let m2 = Obfuscator::mangle("user_password");
        let m3 = Obfuscator::mangle("other_field");

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
        assert!(m1.starts_with("_0x"));
    }
}
