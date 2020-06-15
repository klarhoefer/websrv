
use std::os::raw::{c_char, c_long, c_ulong, c_void};
use std::str;

type HKEY = *mut c_void;
type PVOID = *mut c_void;
type LPCSTR = *const c_char;
type DWORD = c_ulong;
type LPDWORD = *mut DWORD;
type LSTATUS = c_long;

extern "system" {
    fn RegGetValueA(
        hkey: HKEY,
        lpSubKey: LPCSTR,
        lpValue: LPCSTR,
        dwFlags: DWORD,
        pdwType: LPDWORD,
        pvData: PVOID,
        pcbData: LPDWORD
    ) -> LSTATUS;
}

const HKEY_LOCAL_MACHINE: HKEY = 0x80000002u32 as isize as HKEY;
const ERROR_SUCCESS: LSTATUS = 0;
const NULL: usize = 0;

//HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe\
pub fn get_chrome_path() -> Option<String> {
    unsafe {
        let mut buffer = [0u8; 256];
        let mut count = buffer.len() as DWORD;
        let status = RegGetValueA(HKEY_LOCAL_MACHINE,
            b"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\chrome.exe\\\0" as *const _ as LPCSTR,
            b"\0" as *const _ as LPCSTR,
            0xffff,
            NULL as LPDWORD,
            buffer.as_mut_ptr() as PVOID,
            &mut count as LPDWORD
        );
        if status == ERROR_SUCCESS {
            let upper = count as usize - 1; // drop trailing zero
            if let Ok(path) = str::from_utf8(&buffer[..upper]) {
                return Some(path.into());
            }
        }
    }
    None
}
