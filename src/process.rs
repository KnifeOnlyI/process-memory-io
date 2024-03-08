use std::ffi::c_void;
use std::mem::size_of;

use windows::core::imp::FARPROC;
use windows::core::Error;
use windows::Win32::Foundation::{BOOL, HANDLE, HMODULE, MAX_PATH};
use windows::Win32::System::ProcessStatus::{
    EnumProcessModules, EnumProcessModulesEx, EnumProcesses, GetModuleBaseNameW,
    GetModuleFileNameExA, ENUM_PROCESS_MODULES_EX_FLAGS,
};
use windows::Win32::System::Threading::{
    CreateRemoteThread, IsWow64Process, OpenProcess, PROCESS_ACCESS_RIGHTS,
    PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};

use crate::handle;
use crate::windows_api::constants::{DWORD_SIZE, LIST_MODULES_ALL};

/// The default maximum number of processes that can be enumerated.
static DEFAULT_MAX_NB_PROCESSES: u32 = 1024;

/// Represent a process running on the system.
pub struct Process {
    /// A handle to the process.
    pub handle: HANDLE,

    /// A handle to the module.
    pub module_handle: HMODULE,

    /// The process identifier.
    pub pid: u32,

    /// The name of the process.
    pub name: String,
}

/// Enumerates PID of running processes on the system.
///
/// # Arguments
/// nb - The maximum number of PID that can be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is a list of process identifiers.
pub fn enumerate_pid(nb: u32) -> Result<Vec<u32>, Error> {
    let mut lpid_process = Vec::with_capacity(nb as usize);
    let mut lpcb_needed = 0;

    let result = unsafe { EnumProcesses(lpid_process.as_mut_ptr(), nb, &mut lpcb_needed) };

    return if result.is_err() {
        Err(result.unwrap_err())
    } else {
        unsafe { lpid_process.set_len((lpcb_needed / DWORD_SIZE) as usize) };

        Ok(lpid_process)
    };
}

/// Determines if the specified process is running under WOW64 (is 64 bits program).
///
/// # Arguments
/// process_handle - A handle to the process.
///
/// # Returns
/// If the function succeeds, the return value is true if the process is running under WOW64.
pub fn is_64bit_process(process_handle: HANDLE) -> Result<BOOL, Error> {
    let mut is_wow64 = BOOL::from(false);

    let result = unsafe { IsWow64Process(process_handle, &mut is_wow64) };

    return if result.is_err() {
        Err(result.unwrap_err())
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
pub fn open(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> Result<HANDLE, Error> {
    let handle = unsafe { OpenProcess(access, false, pid) };

    return if handle.is_err() {
        Err(handle.unwrap_err())
    } else {
        Ok(handle.unwrap())
    };
}

/// Enumerates the modules associated with the specified process (32 bits / 64 bits).
///
/// # Arguments
/// process_handle - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
pub fn enum_modules(process_handle: HANDLE) -> Result<HMODULE, Error> {
    let result = is_64bit_process(process_handle);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    return if result.unwrap().as_bool() {
        enum_modules_64bits(process_handle)
    } else {
        enum_modules_32bits(process_handle)
    };
}

pub fn get_module_base_name(
    process_handle: HANDLE,
    module_handle: HMODULE,
) -> Result<String, Error> {
    let mut lp_base_name = [0; MAX_PATH as usize];

    let result = unsafe { GetModuleBaseNameW(process_handle, module_handle, &mut lp_base_name) };

    return if result == 0 {
        Err(Error::from_win32())
    } else {
        Ok(String::from_utf16_lossy(&lp_base_name[0..result as usize]))
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
) -> Result<Process, Error> {
    let r_all_pid = enumerate_pid(max_search_size.unwrap_or(DEFAULT_MAX_NB_PROCESSES));

    if r_all_pid.is_err() {
        return Err(Error::from_win32());
    }

    let mut process = Process {
        handle: HANDLE::default(),
        module_handle: HMODULE::default(),
        pid: 0,
        name: String::default(),
    };

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

            let r_handle = open(pid, PROCESS_ACCESS_RIGHTS(access));

            if r_handle.is_err() {
                println!("Failed to re-open process with desired access {}", pid);
                return Err(r_handle.unwrap_err());
            }

            process.handle = r_handle.unwrap();
            process.module_handle = h_module;
            process.pid = pid;
            process.name = process_name;
        } else if handle::close(handle).is_err() {
            println!(
                "Failed to close process handle that not corresponding {}",
                pid
            );
        }
    }

    return if process.handle.is_invalid() {
        Err(Error::from_win32())
    } else {
        Ok(process)
    };
}

/// Create a new thread that runs in the virtual address space of another process.
///
/// # Arguments
/// process - A handle to the process in which the thread is to be created.
/// lp_start_address - A pointer to the application-defined function of type LPTHREAD_START_ROUTINE to be executed by the thread.
/// lp_parameter - A pointer to a variable to be passed to the thread.
///
/// # Returns
/// If the function succeeds, the return value is the handle to the new thread.
pub fn create_remote_thread(
    process: &Process,
    lp_start_address: FARPROC,
    lp_parameter: *const c_void,
) -> Result<HANDLE, Error> {
    let thread_start_routine: Option<
        unsafe extern "system" fn(lpthreadparameter: *mut c_void) -> u32,
    > = lp_start_address.map(|f| unsafe { std::mem::transmute(f) });

    return unsafe {
        CreateRemoteThread(
            process.handle,
            None,
            0,
            thread_start_routine,
            Some(lp_parameter),
            0,
            None,
        )
    };
}

/// Retrieves the main module full path of the specified process.
///
/// # Arguments
/// process - A handle to the process that contains the module.
///
/// # Returns
/// If the function succeeds, the return value is the full path of the module.
pub fn get_full_path(process: &Process) -> Result<String, Error> {
    let mut buffer = [0u8; 1024];
    let size = unsafe { GetModuleFileNameExA(process.handle, process.module_handle, &mut buffer) };

    return if size == 0 {
        Err(Error::from_win32())
    } else {
        Ok(String::from_utf8_lossy(&buffer[0..size as usize]).to_string())
    };
}

/// Enumerates the modules associated with the specified process (32 bits).
///
/// # Arguments
/// process_handle - A handle to the process whose modules are to be enumerated.
///
/// # Returns
/// If the function succeeds, the return value is an array of module handles.
fn enum_modules_32bits(process_handle: HANDLE) -> Result<HMODULE, Error> {
    let mut lph_module = HMODULE::default();
    let mut lpcb_needed = 0;

    let result = unsafe {
        EnumProcessModules(
            process_handle,
            &mut lph_module,
            size_of::<usize>() as u32,
            &mut lpcb_needed,
        )
    };

    return if result.is_err() {
        Err(result.unwrap_err())
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
fn enum_modules_64bits(process_handle: HANDLE) -> Result<HMODULE, Error> {
    let mut lph_module = HMODULE::default();
    let mut lpcb_needed = 0;

    let result = unsafe {
        EnumProcessModulesEx(
            process_handle,
            &mut lph_module,
            size_of::<usize>() as u32,
            &mut lpcb_needed,
            ENUM_PROCESS_MODULES_EX_FLAGS(LIST_MODULES_ALL),
        )
    };

    return if result.is_err() {
        Err(result.unwrap_err())
    } else {
        Ok(lph_module)
    };
}
