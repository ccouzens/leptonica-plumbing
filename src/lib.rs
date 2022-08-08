mod borrowed_box;
mod borrowed_pix;
mod r#box;
mod boxa;
mod pix;
mod pixa;

use self::leptonica_sys::{getImagelibVersions, getLeptonicaVersion};
pub use leptonica_sys;
use std::ffi::CStr;

pub use borrowed_box::BorrowedBox;
pub use borrowed_pix::BorrowedPix;
pub use boxa::Boxa;
pub use pix::{Pix, PixReadError, PixReadMemError};
pub use pixa::Pixa;
pub use r#box::{Box, BoxCreateValidError};

/// Wrapper for [`getLeptonicaVersion`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/utils1.c#L970-L982)
///
/// Returns the version identifier as a static string.
pub fn version() -> &'static CStr {
    unsafe { CStr::from_ptr(getLeptonicaVersion()) }
}

/// Wrapper for [`getImagelibVersions`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/libversions.c#L82-L102)
///
/// Returns the image lib version identifiers as a static string.
pub fn image_lib_versions() -> &'static CStr {
    unsafe { CStr::from_ptr(getImagelibVersions()) }
}
