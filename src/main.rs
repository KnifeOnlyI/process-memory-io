use process_memory_io::memory::{
    read_multi_level_pointer, write_multi_level_pointer, MultiLevelPointer,
};
use process_memory_io::process::get_process_by_name;
use process_memory_io::windows_api::constants::{
    PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

/// This is a simple example of how to use the process_memory_io crate to read and write memory from a process.
/// The example reads the player's HP and PTAS from the game Resident Evil 4.
/// The example also writes the player's HP to the maximum value.
/// The example uses the MultiLevelPointer struct to read and write the player's HP and PTAS.
fn main() {
    let process_name = "re4.exe";
    let process = get_process_by_name(
        process_name,
        None,
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
    )
    .expect("Failed to open process");

    let player_stats_ptr = MultiLevelPointer {
        base_address: 0x0DBB88C0,
        offsets: vec![0xA0, 0x40, 0x50, 0x1F0],
        struct_offset: None,
    };

    let max_hp_ptr = MultiLevelPointer {
        base_address: player_stats_ptr.base_address,
        offsets: player_stats_ptr.offsets.clone(),
        struct_offset: Some(0),
    };

    let hp_ptr = MultiLevelPointer {
        base_address: player_stats_ptr.base_address,
        offsets: player_stats_ptr.offsets.clone(),
        struct_offset: Some(0x4),
    };

    let ptas_ptr = MultiLevelPointer {
        base_address: 0x0DBB8A90,
        offsets: vec![0x10],
        struct_offset: None,
    };

    let max_hp = read_multi_level_pointer::<i32>(&process, &max_hp_ptr).unwrap();
    let hp = read_multi_level_pointer::<i32>(&process, &hp_ptr).unwrap();
    let ptas = read_multi_level_pointer::<i32>(&process, &ptas_ptr).unwrap();

    println!("Process {}", process.name);
    println!("Player : \n  - PTAS: {}\n  - HP: {} / {}", ptas, hp, max_hp);

    // Write the player's HP to the maximum value and PTAS to 999999
    write_multi_level_pointer::<i32>(&process, &hp_ptr, max_hp).unwrap();
    write_multi_level_pointer::<i32>(&process, &ptas_ptr, 999999).unwrap();

    let hp = read_multi_level_pointer::<i32>(&process, &hp_ptr).unwrap();
    let ptas = read_multi_level_pointer::<i32>(&process, &ptas_ptr).unwrap();

    println!(
        "Player Updated : \n  - PTAS: {}\n  - HP: {} / {}",
        ptas, hp, max_hp
    );
}
