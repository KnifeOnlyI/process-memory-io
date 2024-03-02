use std::ffi::c_void;

use crate::windows_api::constants::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_VM_WRITE};

mod handle;
mod memory;
mod process;
mod windows_api;

fn main() {
    let process = process::get_process_by_name(
        "re4.exe",
        None,
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    )
    .expect("Failed to get game process");

    let ammo_ptr = 0x2402ABC4usize as *const c_void;

    let ammo_initial_qty =
        memory::read::<i32>(&process, ammo_ptr).expect("Failed to read process memory");

    println!("Ammo Initial Quantity: {}", ammo_initial_qty);

    let ammo_new_qty = 9999;

    memory::write(&process, ammo_ptr, ammo_new_qty).expect("Failed to write process memory");

    let ammo_new_qty =
        memory::read::<i32>(&process, ammo_ptr).expect("Failed to read process memory");

    println!("Ammo New Quantity: {}", ammo_new_qty);
}
