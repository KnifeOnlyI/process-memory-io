//! https://docs.microsoft.com/en-us/windows/win32/api/psapi

#[link(name = "psapi")]
extern "C" {
    /// https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocesses
    pub fn EnumProcesses(lpid_process: *mut u32, cb: u32, lpcb_needed: *mut u32) -> bool;

    /// https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules
    pub fn EnumProcessModules(
        h_process: usize,
        lph_module: *mut usize,
        cb: u32,
        lpcb_needed: *mut u32,
    ) -> bool;

    /// https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex
    pub fn EnumProcessModulesEx(
        h_process: usize,
        lph_module: *mut usize,
        cb: u32,
        lpcb_needed: *mut u32,
        dw_filter_flag: u32,
    ) -> bool;

    /// https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew
    pub fn GetModuleBaseNameW(
        h_process: usize,
        h_module: usize,
        lp_base_name: *mut u16,
        n_size: u32,
    ) -> u32;
}
