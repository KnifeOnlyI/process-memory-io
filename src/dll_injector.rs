use std::ffi::c_void;

use crate::memory::{allocate_memory, write_process_memory};
use crate::process::{create_remote_thread, Process};
use crate::system::{get_proc_address, load_library};
use crate::windows_api::constants::{MEM_COMMIT, PAGE_EXECUTE_READWRITE};

/// Inject a DLL into the specified process.
///
/// # Arguments
/// process - The process to inject the DLL into.
/// dll_path - The path to the DLL to inject.
///
/// # Returns
/// If the function succeeds, the return value is Ok(()).
pub fn inject_dll(process: &Process, dll_path: &str) -> Result<(), u32> {
    let dll_path = std::ffi::CString::new(dll_path).unwrap();
    let dll_path_nb_bytes = dll_path.to_bytes().len() + 1;

    let r_remote_memory = allocate_memory(
        process,
        dll_path_nb_bytes,
        MEM_COMMIT,
        PAGE_EXECUTE_READWRITE,
    );

    if r_remote_memory.is_err() {
        return Err(r_remote_memory.unwrap_err());
    }

    let remote_memory = r_remote_memory.unwrap();

    let dll_path_buffer = dll_path.as_ptr() as *const c_void;

    let r_write_memory =
        write_process_memory(process, remote_memory, dll_path_buffer, dll_path_nb_bytes);

    if r_write_memory.is_err() {
        return Err(r_write_memory.unwrap_err());
    }

    let r_kernel32 = load_library("kernel32.dll");

    if r_kernel32.is_err() {
        return Err(r_kernel32.unwrap_err());
    }

    let r_load_library_a_addr = get_proc_address(r_kernel32.unwrap(), "LoadLibraryA");

    if r_load_library_a_addr.is_err() {
        return Err(r_load_library_a_addr.unwrap_err());
    }

    let r_thread = create_remote_thread(process, r_load_library_a_addr.unwrap(), remote_memory);

    if r_thread.is_err() {
        return Err(r_thread.unwrap_err());
    }

    Ok(())
}
