use std::mem::size_of;

use crate::handle;
use crate::windows_api::constants::{
    DWORD_SIZE, LIST_MODULES_ALL, MAX_PATH, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};
use crate::windows_api::errhandlingapi::GetLastError;
use crate::windows_api::processthreadsapi::OpenProcess;
use crate::windows_api::psapi::{
    EnumProcessModules, EnumProcessModulesEx, EnumProcesses, GetModuleBaseNameW,
};
use crate::windows_api::wow64apiset::IsWow64Process;

/// The default maximum number of processes that can be enumerated.
static DEFAULT_MAX_NB_PROCESSES: u32 = 1024;

/// Represent a process running on the system.
pub struct Process {
    /// A handle to the process.
    pub(crate) handle: isize,

    /// The process identifier.
    pub(crate) pid: u32,

    /// The name of the process.
    pub(crate) name: String,
}

impl Drop for Process {
    fn drop(&mut self) {
        println!("Drop process");

        if handle::close(self.handle).is_err() {
            println!(
                "Failed to close process handle (probably already closed) {}",
                self.pid
            );
        }
    }
}

/// Enumerates PID of running processes on the system.
///
/// # Arguments
/// nb - The maximum number of PID that can be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is a list of process identifiers.
pub fn enumerate_pid(nb: u32) -> Result<Vec<u32>, u32> {
    let mut lpid_process = Vec::with_capacity(nb as usize);
    let mut lpcb_needed = 0;

    let success = unsafe { EnumProcesses(lpid_process.as_mut_ptr(), nb, &mut lpcb_needed) };

    return if !success {
        Err(unsafe { GetLastError() })
    } else {
        unsafe { lpid_process.set_len((lpcb_needed / DWORD_SIZE) as usize) };

        return Ok(lpid_process);
    };
}

/// Determines if the specified process is running under WOW64 (is 64 bits program).
///
/// # Arguments
/// process_handle - A handle to the process.
///
/// # Returns
/// If the function succeeds, the return value is true if the process is running under WOW64.
pub fn is_64bit_process(process_handle: isize) -> Result<bool, u32> {
    let mut is_wow64 = false;

    let success = unsafe { IsWow64Process(process_handle, &mut is_wow64) };

    return if !success {
        Err(unsafe { GetLastError() })
    } else {
        Ok(is_wow64)
    };
}

/// Open the specified process and return a handle to it.
///
/// # Arguments
/// pid - The process identifier.
/// access - The access to the process.
///
/// # Returns
/// If the function succeeds, the return value is a handle to the process.
pub fn open(pid: u32, access: u32) -> Result<isize, u32> {
    let handle = unsafe { OpenProcess(access, false, pid) };

    return if handle == 0 {
        Err(unsafe { GetLastError() })
    } else {
        Ok(handle)
    };
}

/// Enumerates the modules associated with the specified process (32 bits / 64 bits).
///
/// # Arguments
/// process_handle - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
pub fn enum_modules(process_handle: isize) -> Result<isize, u32> {
    let r_is_64bits = is_64bit_process(process_handle);

    if r_is_64bits.is_err() {
        return Err(r_is_64bits.unwrap_err());
    }

    return if r_is_64bits.unwrap() {
        enum_modules_64bits(process_handle)
    } else {
        enum_modules_32bits(process_handle)
    };
}

pub fn get_module_base_name(process_handle: isize, module_handle: isize) -> Result<String, u32> {
    let mut lp_base_name = [0; MAX_PATH];
    let n_size = lp_base_name.len() as u32;

    let module_base_name_length = unsafe {
        GetModuleBaseNameW(
            process_handle,
            module_handle,
            lp_base_name.as_mut_ptr(),
            n_size,
        )
    };

    return if module_base_name_length == 0 {
        Err(unsafe { GetLastError() })
    } else {
        Ok(String::from_utf16_lossy(
            &lp_base_name[0..module_base_name_length as usize],
        ))
    };
}

/// Find and return a process with the specified name (case-insensitive).
///
/// # Arguments
/// name - The name of the process to find.
/// max_search_size - The maximum number of PID that can be enumerated.
/// access - The access to the process.
///
/// # Returns
/// If the function succeeds, the return value is a process with the specified name.
pub fn get_process_by_name(
    name: &str,
    max_search_size: Option<u32>,
    access: u32,
) -> Result<Process, u32> {
    let r_all_pid = enumerate_pid(max_search_size.unwrap_or(DEFAULT_MAX_NB_PROCESSES));

    if r_all_pid.is_err() {
        return Err(r_all_pid.unwrap_err());
    }

    let mut processes = Vec::new();

    for pid in r_all_pid.unwrap() {
        let r_handle = open(pid, PROCESS_QUERY_INFORMATION | PROCESS_VM_READ);

        if r_handle.is_err() {
            continue;
        }

        let handle = r_handle.unwrap();

        let r_modules = enum_modules(handle);

        if r_modules.is_err() {
            println!("Cannot enumerate modules for process {}", pid);

            if handle::close(handle).is_err() {
                println!("Failed to close process handle {}", pid);
            }

            continue;
        }

        let h_module = r_modules.unwrap();

        let r_module_base_name = get_module_base_name(handle, h_module);

        if r_module_base_name.is_err() {
            println!("Cannot get module base name for process {}", pid);

            if handle::close(handle).is_err() {
                println!("Failed to close process handle {}", pid);
            }

            continue;
        }

        let process_name = r_module_base_name.unwrap();

        if process_name.to_lowercase() == name.to_lowercase() {
            if handle::close(handle).is_err() {
                println!("Failed to close corresponding process before re-open it with desired access {}", pid);
            }

            let r_handle = open(pid, access);

            if r_handle.is_err() {
                println!("Failed to re-open process with desired access {}", pid);
                return Err(unsafe { GetLastError() });
            }

            processes.push(Process {
                handle: r_handle.unwrap(),
                pid,
                name: process_name.clone(),
            });
        } else if handle::close(handle).is_err() {
            println!(
                "Failed to close process handle that not corresponding {}",
                pid
            );
        }
    }

    return if processes.is_empty() {
        Err(0)
    } else {
        Ok(processes.pop().unwrap())
    };
}

/// Enumerates the modules associated with the specified process (32 bits).
///
/// # Arguments
/// process_handle - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
fn enum_modules_32bits(process_handle: isize) -> Result<isize, u32> {
    let mut lph_module = isize::default();
    let mut lpcb_needed = 0;

    let success = unsafe {
        EnumProcessModules(
            process_handle,
            &mut lph_module,
            size_of::<isize>() as u32,
            &mut lpcb_needed,
        )
    };

    return if !success {
        Err(unsafe { GetLastError() })
    } else {
        Ok(lph_module)
    };
}

/// Enumerates the modules associated with the specified process (64 bits).
///
/// # Arguments
/// handle - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
fn enum_modules_64bits(process_handle: isize) -> Result<isize, u32> {
    let mut lph_module = isize::default();
    let mut lpcb_needed = 0;

    let success = unsafe {
        EnumProcessModulesEx(
            process_handle,
            &mut lph_module,
            size_of::<isize>() as u32,
            &mut lpcb_needed,
            LIST_MODULES_ALL,
        )
    };

    return if !success {
        Err(unsafe { GetLastError() })
    } else {
        Ok(lph_module)
    };
}
