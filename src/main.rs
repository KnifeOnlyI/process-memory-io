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

    let player_stats_ptr = memory::MultiLevelPointer {
        base_address: 0x0DBB88C0,
        offsets: vec![0xA0, 0x40, 0x50, 0x1F0],
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
}
