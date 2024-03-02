use std::ffi::c_void;

use crate::process::Process;
use crate::windows_api::errhandlingapi::GetLastError;
use crate::windows_api::memoryapi::ReadProcessMemory;

/// Read value at the specified address (ptr) from the process memory.
///
/// # Arguments
/// process - The process to read from.
/// ptr - The address to read from.
/// buffer - The buffer to read into.
/// size - The size of the buffer.
///
/// # Returns
/// If the function succeeds, the return value is the number of bytes read from the specified process.
pub fn read_process_memory(
    process: &Process,
    ptr: *const c_void,
    buffer: *mut c_void,
    size: usize,
) -> Result<usize, u32> {
    let mut lp_number_of_bytes_read = 0;

    let success = unsafe {
        ReadProcessMemory(
            process.handle,
            ptr,
            buffer,
            size,
            &mut lp_number_of_bytes_read,
        )
    };

    return if !success {
        Err(unsafe { GetLastError() })
    } else {
        Ok(lp_number_of_bytes_read)
    };
}

/// Read the value at the specified address (ptr) from the process memory.
///
/// # Arguments
/// process - The process to read from.
/// ptr - The address to read from.
///
/// # Returns
/// If the function succeeds, the return value is the value read from the specified process.
pub fn read<T>(process: &Process, ptr: *const c_void) -> Result<T, u32> {
    let mut buffer: T = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<T>();

    let r_read_process_memory =
        read_process_memory(process, ptr, &mut buffer as *mut T as *mut c_void, size);

    return if r_read_process_memory.is_err() {
        Err(r_read_process_memory.unwrap_err())
    } else {
        Ok(buffer)
    };
}
