use std::ffi::CString;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};

use detour::static_detour;
use winapi::ctypes::__int32;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::shared::windef::{HDC__, HWND};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{MB_OK, MessageBoxW};

use crate::str::WinStr;

mod str;

static_detour! {
    static wglSwapBuffersHook: unsafe extern "system" fn(*const HDC__) -> __int32;
}

type WglSwapBuffersT = unsafe extern "system" fn(*const HDC__) -> __int32;

static CALLED: AtomicBool = AtomicBool::new(false);

fn wgl_swap_buffers(hdc: *const HDC__) -> __int32 {
    if !CALLED.load(Ordering::Relaxed) {
        CALLED.store(true, Ordering::Relaxed);
        unsafe {
            MessageBoxW(0 as HWND, "uwu".as_wstr(), "uwu".as_wstr(), MB_OK);
        }
    }

    unsafe {
        wglSwapBuffersHook.call(hdc)
    }
}

#[no_mangle]
pub extern "system" fn DllMain(_module: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            let address = get_module_symbol_address("opengl32.dll", "wglSwapBuffers").unwrap();
            let target: WglSwapBuffersT = mem::transmute(address);

            wglSwapBuffersHook
                .initialize(target, wgl_swap_buffers)
                .unwrap()
                .enable()
                .unwrap();
        }
    }

    TRUE
}

fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
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