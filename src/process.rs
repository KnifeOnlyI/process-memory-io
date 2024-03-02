use std::fmt::Error;

use crate::windows_api::psapi::EnumProcesses;
use crate::windows_api::types::DWORD_SIZE;

/// The default maximum number of processes that can be enumerated.
static DEFAULT_MAX_NB_PROCESSES: u32 = 1024;

/// Enumerates all processes running on the system.
///
/// # Arguments
/// cb - The maximum number of processes that can be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is a list of process identifiers.
pub fn enumerate_processes(cb: Option<u32>) -> Result<Vec<u32>, Error> {
    let cb = cb.unwrap_or(DEFAULT_MAX_NB_PROCESSES);

    let mut lpidprocess = Vec::with_capacity(cb as usize);
    let mut lpcbneeded = 0;

    let success = unsafe { EnumProcesses(lpidprocess.as_mut_ptr(), cb, &mut lpcbneeded) };

    if !success {
        return Err(Error);
    }

    unsafe { lpidprocess.set_len((lpcbneeded / DWORD_SIZE) as usize) };

    return Ok(lpidprocess);
}
