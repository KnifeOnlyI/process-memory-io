use windows::core::imp::{GetProcAddress, FARPROC, HMODULE};
use windows::core::{Error, PCSTR};
use windows::Win32::Foundation;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

/// Load a library into the address space of the calling process.
///
/// # Arguments
/// library_name - A pointer to a null-terminated string that specifies the name of the library.
///
/// # Returns
/// If the function succeeds, the return value is a handle to the library.
pub fn load_library(library_name: &str) -> Result<Foundation::HMODULE, Error> {
    return unsafe { GetModuleHandleA(PCSTR::from_raw(library_name.as_ptr())) };
}

/// Retrieves the address of an exported function or variable from the specified library (a DLL).
///
/// # Arguments
/// library_handle - A handle to the library that contains the function or variable.
/// proc_name - The function or variable name, or the function's ordinal value.
///
/// # Returns
/// If the function succeeds, the return value is the address of the exported function or variable.
pub fn get_proc_address(library_handle: HMODULE, proc_name: &str) -> Result<FARPROC, Error> {
    let func_address = unsafe { GetProcAddress(library_handle, proc_name.as_ptr()) };

    return if func_address.is_none() {
        Err(Error::from_win32())
    } else {
        Ok(func_address)
    };
}
