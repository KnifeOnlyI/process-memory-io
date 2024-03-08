use std::ffi::c_void;

use windows::core::Error;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Memory::{
    VirtualAllocEx, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
};

use crate::process::Process;

/// Represent a multi-level pointer.
///
/// # Fields
/// base_address - The base address of the pointer.
/// offsets - The offsets to apply to the base address.
/// struct_offset - The offset to apply to the final address (usefull to read a specific property of a struct).
pub struct MultiLevelPointer {
    pub base_address: usize,
    pub offsets: Vec<usize>,
    pub struct_offset: Option<usize>,
}

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
) -> Result<usize, Error> {
    let mut lp_number_of_bytes_read = 0;

    let result = unsafe {
        ReadProcessMemory(
            process.handle,
            ptr,
            buffer,
            size,
            Some(&mut lp_number_of_bytes_read),
        )
    };

    return if result.is_err() {
        Err(result.unwrap_err())
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
pub fn read<T>(process: &Process, ptr: *const c_void) -> Result<T, Error> {
    let mut buffer: T = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<T>();

    let result = read_process_memory(process, ptr, &mut buffer as *mut T as *mut c_void, size);

    return if result.is_err() {
        Err(result.unwrap_err())
    } else {
        Ok(buffer)
    };
}

/// Read the value at the specified multi-level pointer from the process memory.
///
/// # Arguments
/// process - The process to read from.
/// mlp - The multi-level pointer to read from.
///
/// # Returns
/// If the function succeeds, the return value is the value read from the specified process.
pub fn read_multi_level_pointer<T>(process: &Process, mlp: &MultiLevelPointer) -> Result<T, Error> {
    let mut ptr = read::<usize>(
        &process,
        (process.module_handle.0 + mlp.base_address as isize) as *const c_void,
    )
    .unwrap();

    for i in 0..mlp.offsets.len() - 1 {
        ptr = read::<usize>(&process, (ptr + mlp.offsets[i]) as *const c_void).unwrap();
    }

    ptr = ptr + (mlp.offsets[mlp.offsets.len() - 1] + mlp.struct_offset.unwrap_or(0));

    return read::<T>(&process, ptr as *const c_void);
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
) -> Result<usize, Error> {
    let mut lp_number_of_bytes_written = 0;

    let result = unsafe {
        WriteProcessMemory(
            process.handle,
            ptr,
            buffer,
            size,
            Some(&mut lp_number_of_bytes_written),
        )
    };

    return if result.is_err() {
        Err(result.unwrap_err())
    } else {
        Ok(lp_number_of_bytes_written)
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
pub fn write<T>(process: &Process, ptr: *const c_void, value: T) -> Result<usize, Error> {
    let size = std::mem::size_of::<T>();

    let result = write_process_memory(process, ptr, &value as *const T as *const c_void, size);

    return if result.is_err() {
        Err(result.unwrap_err())
    } else {
        Ok(result.unwrap())
    };
}

/// Write the specified value at the specified multi-level pointer from the process memory.
///
/// # Arguments
/// process - The process to write to.
/// mlp - The multi-level pointer to write to.
/// value - The value to write.
///
/// # Returns
/// If the function succeeds, the return value is the number of bytes written in the specified process.
pub fn write_multi_level_pointer<T>(
    process: &Process,
    mlp: &MultiLevelPointer,
    value: T,
) -> Result<usize, Error> {
    let mut ptr = read::<usize>(
        &process,
        (process.module_handle.0 + mlp.base_address as isize) as *const c_void,
    )
    .unwrap();

    for i in 0..mlp.offsets.len() - 1 {
        ptr = read::<usize>(&process, (ptr + mlp.offsets[i]) as *const c_void).unwrap();
    }

    ptr = ptr + (mlp.offsets[mlp.offsets.len() - 1] + mlp.struct_offset.unwrap_or(0));

    return write::<T>(&process, ptr as *const c_void, value);
}

/// Allocate memory in the specified process.
///
/// # Arguments
/// process - The process to allocate memory in.
/// size - The size of the memory to allocate.
/// fl_allocation_type - The type of memory allocation.
/// fl_protect - The memory protection for the region of pages to be allocated
///
/// # Returns
/// If the function succeeds, the return value is the base address of the allocated memory.
pub fn allocate_memory(
    process: &Process,
    size: usize,
    fl_allocation_type: VIRTUAL_ALLOCATION_TYPE,
    fl_protect: PAGE_PROTECTION_FLAGS,
) -> Result<*mut c_void, Error> {
    let lp_base_address =
        unsafe { VirtualAllocEx(process.handle, None, size, fl_allocation_type, fl_protect) };

    return if lp_base_address.is_null() {
        Err(Error::from_win32())
    } else {
        Ok(lp_base_address)
    };
}
