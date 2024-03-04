//! https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi

use std::ffi::c_void;

#[link(name = "kernel32")]
extern "C" {
    /// https://learn.microsoft.com/en-US/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
    pub fn OpenProcess(dw_desired_access: u32, b_inherit_handle: bool, dw_process_id: u32)
        -> usize;

    /// https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createremotethread
    pub fn CreateRemoteThread(
        h_process: usize,
        lp_thread_attributes: *const c_void,
        dw_stack_size: usize,
        lp_start_address: *const c_void,
        lp_parameter: *const c_void,
        dw_creation_flags: u32,
        lp_thread_id: *const c_void,
    ) -> usize;
}
