use crate::memory::MultiLevelPointer;
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
/// (This function use re2.exe as target process and the dll path is hardcoded)
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
    get_exec_hash(&process);
    inject_dll(&process, dll_path);
    read_write_multi_level_pointers(&process);
}

fn get_exec_path(process: &Process) {
    let exec_path = process::get_full_path(process).expect("Failed to get exec path");

    println!("Successfully read exec path: {}", exec_path);
}

fn get_exec_hash(process: &Process) {
    let exec_hash = process::get_hash(process).expect("Failed to get exec hash");

    println!("Successfully read exec hash: {:?}", exec_hash);
}

fn inject_dll(process: &Process, dll_path: &str) {
    dll_injector::inject_dll(&process, dll_path).expect("Failed to inject dll");

    println!("Successfully injected DLL into target process");
}

fn read_write_multi_level_pointers(process: &Process) {
    let player_condition_ptr = MultiLevelPointer {
        base_address: 0x091AD2C0,
        offsets: vec![0x50, 0x10, 0x20],
    };

    let player_hit_point_ptr = MultiLevelPointer::from(&player_condition_ptr, vec![0x230]);

    let player_max_hp_ptr = MultiLevelPointer::from(&player_hit_point_ptr, vec![0x54]);
    let player_hp_ptr = MultiLevelPointer::from(&player_hit_point_ptr, vec![0x58]);

    let max_hp = player_max_hp_ptr.read::<i32>(process, 0x0).unwrap();
    let mut hp = player_hp_ptr.read::<i32>(process, 0x0).unwrap();

    println!("HP: {} / {}", hp, max_hp);

    player_hp_ptr.write::<i32>(process, 0x0, max_hp).unwrap();

    hp = player_hp_ptr.read::<i32>(process, 0x0).unwrap();

    println!("NEW HP: {} / {}", hp, max_hp);

    println!("Successfully read and write in memory with multi-level pointers");
}
