use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub trait CStr {
    fn as_wide_cstr(&self) -> *const u16;
}

impl<T> CStr for T where T: AsRef<OsStr> {
    fn as_wide_cstr(&self) -> *const u16 {
        self.as_ref().encode_wide().chain(Some(0)).collect::<Vec<u16>>().as_ptr()
    }
}