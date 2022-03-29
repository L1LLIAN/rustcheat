use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub trait WinStr {
    fn as_wstr(&self) -> *const u16;
}

impl<T> WinStr for T where T: AsRef<OsStr> {
    fn as_wstr(&self) -> *const u16 {
        self.as_ref().encode_wide().chain(Some(0)).collect::<Vec<u16>>().as_ptr()
    }
}