//! https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi

use std::ffi::c_void;

#[link(name = "kernel32")]
extern "C" {
    /// https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlea
    pub fn GetModuleHandleA(lp_module_name: *const c_void) -> *const c_void;

    /// https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress
    pub fn GetProcAddress(h_module: *const c_void, lp_proc_name: *const c_void) -> *const c_void;
}
