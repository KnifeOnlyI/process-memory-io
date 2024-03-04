use wapi::dll_injector::inject_dll;
use wapi::process::get_process_by_name;
use wapi::windows_api::constants::PROCESS_VM_READ;

use crate::windows_api::constants::{
    PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_WRITE,
};

mod dll_injector;
mod handle;
mod memory;
mod process;
mod system;
mod windows_api;

/// This is a simple example of how to use the lib crate to inject a dll into a process.
/// The process name and dll path are passed as command line arguments.
///
/// # Example
/// ```bash
/// wapi.exe <process_name> <dll_path>
/// ```
///
/// # Arguments
/// * process_name - The name of the process to inject the dll into.
/// * dll_path - The path to the dll to inject.
fn main() {
    // Get args from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: wapi.exe <process_name> <dll_path>");
        return;
    }

    let dll_path = &args[2];
    let process_name = &args[1];

    let process = get_process_by_name(
        process_name,
        None,
        PROCESS_QUERY_INFORMATION | PROCESS_VM_WRITE | PROCESS_VM_READ | PROCESS_VM_OPERATION,
    )
    .expect("Failed to open process");

    inject_dll(&process, dll_path).expect("Failed to inject dll");
}
