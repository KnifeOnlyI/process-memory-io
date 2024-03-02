#[link(name = "psapi")]
extern "C" {
    /// https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
    pub fn EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> bool;
}