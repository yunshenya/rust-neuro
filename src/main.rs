use std::ffi::CString;
use std::ptr;

use winapi::shared::minwindef::{FALSE, LPCVOID, LPVOID};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};

fn main() {
    let windows_name = CString::new("植物大战僵尸").unwrap();
    unsafe {
        let handle = FindWindowA(ptr::null(), windows_name.as_ptr());
        if handle.is_null() {
            println!("窗口没有找到");
        }
        let mut pid:u32 = 0;
        GetWindowThreadProcessId(handle, &mut pid as *mut u32);
        let hprocess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
        let base_adr = 0x006A9EC0 as LPCVOID;
        let offsets = [0x768, 0x5560];
        let value: u32 = 0;
        let tmp: usize = 0;
        let tmp2: usize = 0;

        ReadProcessMemory(hprocess, base_adr, tmp as LPVOID, size_of_val(&tmp), ptr::null_mut());
        ReadProcessMemory(hprocess, (tmp + offsets[0]) as LPCVOID, tmp2 as LPVOID, size_of_val(&tmp2), ptr::null_mut());
        ReadProcessMemory(hprocess, (tmp2  + offsets[1]) as LPCVOID, value  as LPVOID, size_of_val(&value), ptr::null_mut());
        println!("阳光的值：{}", value);

        let lock_value: u32 = 9999;
        WriteProcessMemory(hprocess, (tmp2 + offsets[1]) as LPVOID, lock_value as LPCVOID, size_of_val(&lock_value), ptr::null_mut());
    }
}