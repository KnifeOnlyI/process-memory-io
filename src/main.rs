use crate::process::Process;
use crate::windows_api::constants::{
    PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

mod dll_injector;
mod handle;
mod memory;
mod process;
mod system;
mod windows_api;

/// This main function is to test directly the library functions without build its.
fn main() {
    // Get args from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: wapi.exe <process_name> <dll_path>");
        return;
    }

    let dll_path = &args[2];
    let process_name = &args[1];

    let process = process::get_process_by_name(
        process_name,
        None,
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
    )
    .expect("Failed to get game process");

    get_exec_path(&process);
    inject_dll(&process, dll_path);
    read_write_multi_level_pointers(&process);
}

fn get_exec_path(process: &Process) {
    let exec_path = process::get_full_path(process).expect("Failed to get exec path");

    println!("Successfully read exec path: {}", exec_path);
}

fn inject_dll(process: &Process, dll_path: &str) {
    dll_injector::inject_dll(&process, dll_path).expect("Failed to inject dll");

    println!("Successfully injected DLL into target process");
}

fn read_write_multi_level_pointers(process: &Process) {
    let player_stats_ptr = memory::MultiLevelPointer {
        base_address: 0x0DBB8A38,
        offsets: vec![0x10, 0x40, 0x148, 0x10],
        struct_offset: None,
    };

    let max_hp_ptr = memory::MultiLevelPointer {
        base_address: player_stats_ptr.base_address,
        offsets: player_stats_ptr.offsets.clone(),
        struct_offset: Some(0),
    };

    let hp_ptr = memory::MultiLevelPointer {
        base_address: player_stats_ptr.base_address,
        offsets: player_stats_ptr.offsets.clone(),
        struct_offset: Some(0x4),
    };

    let max_hp = memory::read_multi_level_pointer::<i32>(&process, &max_hp_ptr).unwrap();
    let hp = memory::read_multi_level_pointer::<i32>(&process, &hp_ptr).unwrap();

    println!("HP: {} / {}", hp, max_hp);

    memory::write_multi_level_pointer::<i32>(&process, &hp_ptr, max_hp).unwrap();

    let hp = memory::read_multi_level_pointer::<i32>(&process, &hp_ptr).unwrap();

    println!("NEW HP: {} / {}", hp, max_hp);

    println!("Successfully read and write in memory with multi-level pointers");
}
