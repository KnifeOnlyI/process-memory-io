mod handle;
mod process;
mod windows_api;

fn main() {
    let process = process::get_process_by_name("notepad.exe", None).unwrap();

    println!("NAME = \"{}\" / PID = {} / HANDLE = {}", process.name, process.pid, process.handle);

    if handle::close(process.handle).is_err() {
        println!("Failed to close process handle {}", process.name);
    }
}
