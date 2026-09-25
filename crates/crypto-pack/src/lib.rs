//! # Crypto Pack
//!
//! AOT 字节码二进制打包、符号哈希混淆脱敏与内存动态加密引擎。

pub mod binui_packer;
pub mod bytecode;
pub mod crypto;
pub mod obfuscate;

pub use binui_packer::{BinUiPacker, PackOptions};
pub use bytecode::{BinaryPackage, BYTECODE_MAGIC};
pub use crypto::StreamCrypto;
pub use obfuscate::{hash_symbol, Obfuscator};
