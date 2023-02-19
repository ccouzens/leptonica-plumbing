use self::leptonica_sys::free;
pub use leptonica_sys;
use libc::c_char;
use std::{ffi::CStr, str::Utf8Error};

/// Wrapper for heap allocated leptonica strings
#[derive(Debug)]
pub struct Str {
    value: *mut c_char,
}

impl Str {
    /// Create a new Str from a heap allocated c-string pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid heap allocated c-string.
    /// The data pointed at may not be mutated while held by
    /// this struct except by this struct.
    /// On drop, the pointer will be freed.
    pub unsafe fn new_from_pointer(pointer: *mut c_char) -> Self {
        Self { value: pointer }
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        AsRef::<CStr>::as_ref(self).to_str()
    }
}

impl AsRef<CStr> for Str {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.value) }
    }
}

impl Drop for Str {
    fn drop(&mut self) {
        unsafe { free(self.value.cast()) }
    }
}
