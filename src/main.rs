mod windows_api;
pub mod process;

fn main() {
    let processes = process::enumerate_processes(None).unwrap();

    for pid in processes {
        println!("PID: {}", pid);
    }
}
