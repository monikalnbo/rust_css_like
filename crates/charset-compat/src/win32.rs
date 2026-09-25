//! Windows 平台原生宽字符 (LPCWSTR / UTF-16) 互操作通道

pub struct Win32String;

impl Win32String {
    /// 将 Rust UTF-8 字符串编码为 Windows Win32 API 接受的 null-terminated UTF-16 缓冲
    pub fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    /// 从以 null 结尾的 Windows 宽字符指针读取还原为 UTF-8 String
    ///
    /// # Safety
    /// `ptr` 必须是一个合法的以 0 结尾的 u16 指针。
    pub unsafe fn from_wide_ptr(ptr: *const u16) -> String {
        if ptr.is_null() {
            return String::new();
        }
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf16_lossy(slice)
    }
}
