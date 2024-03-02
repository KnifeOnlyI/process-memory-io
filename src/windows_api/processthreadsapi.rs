//! https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi

#[link(name = "kernel32")]
extern "C" {
    /// https://learn.microsoft.com/en-US/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
    pub fn OpenProcess(dw_desired_access: u32, b_inherit_handle: bool, dw_process_id: u32)
        -> isize;
}
