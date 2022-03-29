use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};

use detour::static_detour;
use winapi::ctypes::__int32;
use winapi::shared::windef::{HDC__, HWND};
use winapi::um::winuser::{MB_OK, MessageBoxW};

use crate::common::get_proc_addr;
use crate::str::CStr;

static_detour! {
    static wglSwapBuffersHook: unsafe extern "system" fn(*const HDC__) -> __int32;
}

type WglSwapBuffersT = unsafe extern "system" fn(*const HDC__) -> __int32;

static CALLED: AtomicBool = AtomicBool::new(false);

fn wgl_swap_buffers(hdc: *const HDC__) -> __int32 {
    if !CALLED.load(Ordering::Relaxed) {
        CALLED.store(true, Ordering::Relaxed);
        unsafe {
            MessageBoxW(0 as HWND, "uwu".as_wide_cstr(), "uwu".as_wide_cstr(), MB_OK);
        }
    }

    unsafe {
        wglSwapBuffersHook.call(hdc)
    }
}

pub fn init() {
    unsafe {
        let address = get_proc_addr("opengl32.dll", "wglSwapBuffers").unwrap();
        let target: WglSwapBuffersT = mem::transmute(address);

        wglSwapBuffersHook
            .initialize(target, wgl_swap_buffers)
            .unwrap()
            .enable()
            .unwrap();
    }
}