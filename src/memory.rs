use std::ffi::c_void;

use crate::process::Process;
use crate::windows_api::errhandlingapi::GetLastError;
use crate::windows_api::memoryapi::{ReadProcessMemory, WriteProcessMemory};

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

/// Write the specified buffer in the memory of the specified process at the specified address.
///
/// # Arguments
/// process - The process to write to.
/// ptr - The address to write to.
/// buffer - The buffer to write from.
/// size - The size of the buffer.
///
/// # Returns
/// If the function succeeds, the return value is the number of bytes written in the specified process.
pub fn write_process_memory(
    process: &Process,
    ptr: *const c_void,
    buffer: *const c_void,
    size: usize,
) -> Result<usize, u32> {
    let mut lp_number_of_bytes_written = 0;

    let success = unsafe {
        WriteProcessMemory(
            process.handle,
            ptr,
            buffer,
            size,
            &mut lp_number_of_bytes_written,
        )
    };

    return if !success {
        Err(unsafe { GetLastError() })
    } else {
        Ok(lp_number_of_bytes_written)
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

/// Write the specified value in the memory of the specified process at the specified address.
///
/// # Arguments
/// process - The process to write to.
/// ptr - The address to write to.
/// value - The value to write.
///
/// # Returns
/// If the function succeeds, the return value is the number of bytes written in the specified process.
pub fn write<T>(process: &Process, ptr: *const c_void, value: T) -> Result<usize, u32> {
    let size = std::mem::size_of::<T>();

    let r_write_process_memory =
        write_process_memory(process, ptr, &value as *const T as *const c_void, size);

    return if r_write_process_memory.is_err() {
        Err(r_write_process_memory.unwrap_err())
    } else {
        Ok(r_write_process_memory.unwrap())
    };
}
