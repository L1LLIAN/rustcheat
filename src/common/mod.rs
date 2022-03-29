use std::ffi::CString;

use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use crate::str::WinStr;

pub fn get_proc_addr(module: &str, symbol: &str) -> Option<usize> {
    let module = module.as_wstr();
    let symbol = CString::new(symbol).unwrap();

    unsafe {
        let handle = GetModuleHandleW(module);
        match GetProcAddress(handle, symbol.as_ptr()) as usize {
            0 => None,
            n => Some(n),
        }
    }
}