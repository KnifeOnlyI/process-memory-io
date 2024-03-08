use windows::Win32::Foundation::{CloseHandle, HANDLE};

/// Close the specified handle.
///
/// # Arguments
/// handle - The handle to close.
///
/// # Returns
/// If the function succeeds, the return value is Ok.
pub fn close(handle: HANDLE) -> windows::core::Result<()> {
    return unsafe { CloseHandle(handle) };
}
