pub static DWORD_SIZE: u32 = 4;

pub static PROCESS_VM_READ: u32 = 0x0010;
pub static PROCESS_VM_WRITE: u32 = 0x0020;
pub static PROCESS_VM_OPERATION: u32 = 0x0008;
pub static PROCESS_QUERY_INFORMATION: u32 = 0x0400;

pub static LIST_MODULES_ALL: u32 = 0x03;

pub static MEM_COMMIT: u32 = 0x1000;
pub static PAGE_EXECUTE_READWRITE: u32 = 0x40;
