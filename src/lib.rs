mod borrowed_box;
mod borrowed_pix;
mod r#box;
mod boxa;
mod pix;
mod pixa;

use self::leptonica_sys::{free, getImagelibVersions, getLeptonicaVersion};
pub use leptonica_sys;
use libc::c_char;
use std::ffi::{c_void, CStr};

pub use borrowed_box::BorrowedBox;
pub use borrowed_pix::BorrowedPix;
pub use boxa::Boxa;
pub use pix::{Pix, PixReadError, PixReadMemError};
pub use pixa::Pixa;
pub use r#box::{Box, BoxCreateValidError};

/// Wrapper container for leptonica's version and image lib versions strings.
#[derive(Debug)]
pub struct LeptonicaVersion {
    version: *mut c_char,
    image_lib_versions: *mut c_char,
}

impl Drop for LeptonicaVersion {
    fn drop(&mut self) {
        unsafe {
            free(self.version as *mut c_void);
            free(self.image_lib_versions as *mut c_void);
        }
    }
}

/// Returns a struct containing leptonica's version and image lib versions strings,
/// and will automatically free the memory for these C-strings when out of scope.
impl LeptonicaVersion {
    pub fn new() -> Self {
        LeptonicaVersion {
            version: unsafe { getLeptonicaVersion() },
            image_lib_versions: unsafe { getImagelibVersions() },
        }
    }

    /// Wrapper for [`getLeptonicaVersion`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/utils1.c#L970-L982)
    ///
    /// Returns the version identifier as a static string.
    pub fn get_version(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(self.version)
                .to_str()
                .expect("failed to call getLeptonicaVersion")
        }
    }

    /// Wrapper for [`getImagelibVersions`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/libversions.c#L82-L102)
    ///
    /// Returns the image lib version identifiers as a static string.
    pub fn get_imagelib_versions(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(self.image_lib_versions)
                .to_str()
                .expect("failed to call getImagelibVersions")
        }
    }
}
