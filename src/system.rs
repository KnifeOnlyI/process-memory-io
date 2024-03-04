use std::ffi::c_void;

use crate::windows_api::errhandlingapi::GetLastError;
use crate::windows_api::libloaderapi::{GetModuleHandleA, GetProcAddress};

/// Load a library into the address space of the calling process.
///
/// # Arguments
/// library_name - A pointer to a null-terminated string that specifies the name of the library.
///
/// # Returns
/// If the function succeeds, the return value is a handle to the library.
pub fn load_library(library_name: &str) -> Result<*const c_void, u32> {
    let library_handle = unsafe { GetModuleHandleA(library_name.as_ptr() as *const c_void) };

    return if library_handle.is_null() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(library_handle)
    };
}

/// Retrieves the address of an exported function or variable from the specified library (a DLL).
///
/// # Arguments
/// library_handle - A handle to the library that contains the function or variable.
/// proc_name - The function or variable name, or the function's ordinal value.
///
/// # Returns
/// If the function succeeds, the return value is the address of the exported function or variable.
pub fn get_proc_address(
    library_handle: *const c_void,
    proc_name: &str,
) -> Result<*const c_void, u32> {
    let func_address =
        unsafe { GetProcAddress(library_handle, proc_name.as_ptr() as *const c_void) };

    return if func_address.is_null() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(func_address)
    };
}
