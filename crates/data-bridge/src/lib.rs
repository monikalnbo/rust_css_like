//! # Data Bridge
//!
//! 跨主流语言 (Python/Node.js/C++/Go/C#) 互操作 FFI 接口与后端数据库 (SQLite/SQL) 连接枢纽。

pub mod database;
pub mod ffi_c;
pub mod reactive;
pub mod sqlite_local;
pub mod storage;

pub use database::{DatabaseClient, DbRow, DbValue};
pub use ffi_c::{rust_css_engine_init, rust_css_engine_load_dsl, rust_css_engine_version};
pub use reactive::Signal;
pub use sqlite_local::EmbeddedDatabase;
pub use storage::{LocalStorage, SessionStore, StorageEntry};
