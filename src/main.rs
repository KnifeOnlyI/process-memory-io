use wapi::process::{get_full_path, get_process_by_name};
use wapi::windows_api::constants::{
    PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

mod dll_injector;
mod handle;
mod memory;
mod process;
mod system;
mod windows_api;

/// This is a simple example of how to use the lib crate find a process by name and get the exec path.
/// The process name and is passed as command line arguments.
///
/// # Example
/// ```bash
/// wapi.exe <process_name>
/// ```
///
/// # Arguments
/// * process_name - The name of the process to find and get the exec path.
fn main() {
    // Get args from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: wapi.exe <process_name>");
        return;
    }

    let process_name = &args[1];

    let process = get_process_by_name(
        process_name,
        None,
        PROCESS_QUERY_INFORMATION | PROCESS_VM_WRITE | PROCESS_VM_READ | PROCESS_VM_OPERATION,
    )
    .expect("Failed to open process");

    println!("Process: {}", process.name);

    let exec_path = get_full_path(&process).expect("Failed to get exec path");

    println!("Exec path: {}", exec_path);
}
