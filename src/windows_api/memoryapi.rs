//! https://learn.microsoft.com/en-us/windows/win32/api/memoryapi

use std::ffi::c_void;

#[link(name = "kernel32")]
extern "C" {
    /// https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory
    pub fn ReadProcessMemory(
        h_process: isize,
        lp_base_address: *const c_void,
        lp_buffer: *mut c_void,
        n_size: usize,
        lp_number_of_bytes_read: *mut usize,
    ) -> bool;

    /// https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory
    pub fn WriteProcessMemory(
        h_process: isize,
        lp_base_address: *const c_void,
        lp_buffer: *const c_void,
        n_size: usize,
        lp_number_of_bytes_written: *mut usize,
    ) -> bool;
}
