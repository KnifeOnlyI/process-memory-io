use std::ffi::c_void;

use crate::process::Process;
use crate::windows_api::errhandlingapi::GetLastError;
use crate::windows_api::memoryapi::{ReadProcessMemory, VirtualAllocEx, WriteProcessMemory};

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

/// Read the value at the specified multi-level pointer from the process memory.
///
/// # Arguments
/// process - The process to read from.
/// mlp - The multi-level pointer to read from.
///
/// # Returns
/// If the function succeeds, the return value is the value read from the specified process.
pub fn read_multi_level_pointer<T>(process: &Process, mlp: &MultiLevelPointer) -> Result<T, u32> {
    let mut ptr = read::<usize>(
        &process,
        (process.module_handle + mlp.base_address) as *const c_void,
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
) -> Result<usize, u32> {
    let mut ptr = read::<usize>(
        &process,
        (process.module_handle + mlp.base_address) as *const c_void,
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
    fl_allocation_type: u32,
    fl_protect: u32,
) -> Result<*const c_void, u32> {
    let lp_base_address = unsafe {
        VirtualAllocEx(
            process.handle,
            std::ptr::null(),
            size,
            fl_allocation_type,
            fl_protect,
        )
    };

    return if lp_base_address.is_null() {
        Err(unsafe { GetLastError() })
    } else {
        Ok(lp_base_address)
    };
}
