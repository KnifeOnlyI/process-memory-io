use crate::windows_api::errhandlingapi::GetLastError;
use crate::windows_api::handleapi::CloseHandle;

/// Close the specified handle.
///
/// # Arguments
/// handle - The handle to close.
///
/// # Returns
/// If the function succeeds, the return value is Ok.
pub fn close(handle: usize) -> Result<(), u32> {
    let r_close_handle = unsafe { CloseHandle(handle) };

    return if r_close_handle == 0 {
        Err(unsafe { GetLastError() })
    } else {
        Ok(())
    };
}
