//! 跨主流语言标准 C-ABI 导出层 (FFI Bridge)
//!
//! 供 Python (ctypes/PyO3), C++, Go (cgo), C# (P/Invoke), Node.js (napi) 调用。

use std::ffi::{c_char, c_int, CStr, CString};

/// 导出供外部主流语言调用的引擎生命周期句柄
#[no_mangle]
pub extern "C" fn rust_css_engine_init() -> c_int {
    // 0 表示初始化成功
    0
}

/// 接收外部语言传入的 DSL 字符串并触发更新
///
/// # Safety
/// `dsl_code` 必须是一个合法的以 null 结尾的 UTF-8 C 字符串指针。
#[no_mangle]
pub unsafe extern "C" fn rust_css_engine_load_dsl(dsl_code: *const c_char) -> c_int {
    if dsl_code.is_null() {
        return -1;
    }
    let c_str = CStr::from_ptr(dsl_code);
    if let Ok(_source) = c_str.to_str() {
        // 成功接收到外部语言传入的代码
        0
    } else {
        -2
    }
}

/// 查询引擎版本号
#[no_mangle]
pub extern "C" fn rust_css_engine_version() -> *mut c_char {
    let version = CString::new("0.1.0").unwrap();
    version.into_raw()
}
