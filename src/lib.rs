use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod str;
mod hooks;
mod common;

#[no_mangle]
pub extern "system" fn DllMain(_module: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        hooks::swapbuffers::init();
    }

    TRUE
}