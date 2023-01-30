mod borrowed_box;
mod borrowed_box_wrapper;
mod borrowed_boxa;
mod borrowed_pix;
mod borrowed_pixa;
mod r#box;
mod boxa;
mod cloned_pix;
mod pix;
mod pixa;
mod str;

use self::leptonica_sys::{getImagelibVersions, getLeptonicaVersion};
pub use leptonica_sys;

pub use crate::str::Str;
pub use borrowed_box_wrapper::BorrowedBoxWrapper;
pub use borrowed_boxa::BorrowedBoxa;
pub use borrowed_pix::BorrowedPix;
pub use borrowed_pixa::BorrowedPixa;
pub use boxa::Boxa;
pub use cloned_pix::ClonedPix;
pub use pix::{Pix, PixReadError, PixReadMemError};
pub use pixa::Pixa;
pub use r#box::{Box, BoxCreateValidError};

/// Wrapper for [`getLeptonicaVersion`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/utils1.c#L970-L982)
///
/// Returns the version identifier as a LeptonicaString.
pub fn get_version() -> Str {
    unsafe { Str::new_from_pointer(getLeptonicaVersion()) }
}

/// Wrapper for [`getImagelibVersions`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/libversions.c#L82-L102)
///
/// Returns the image lib version identifiers as a LeptonicaString.
pub fn get_imagelib_versions() -> Str {
    unsafe { Str::new_from_pointer(getImagelibVersions()) }
}
