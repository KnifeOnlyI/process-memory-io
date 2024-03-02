//! https://learn.microsoft.com/en-us/windows/win32/api/handleapi

#[link(name = "kernel32")]
extern "C" {
    /// https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
    pub fn CloseHandle(handle: isize) -> u32;
}
