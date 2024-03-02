//! https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi

#[link(name = "kernel32")]
extern "C" {
    /// https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
    pub fn GetLastError() -> u32;
}
