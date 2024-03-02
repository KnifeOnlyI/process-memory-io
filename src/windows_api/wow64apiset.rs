//! https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset

#[link(name = "kernel32")]
extern "C" {
    /// https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process
    pub fn IsWow64Process(h_process: isize, wow64process: *mut bool) -> bool;
}
