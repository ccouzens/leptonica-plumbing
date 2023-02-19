mod r#box;
mod boxa;
pub mod memory;
mod pix;
mod pixa;
mod str;

use self::leptonica_sys::{getImagelibVersions, getLeptonicaVersion};
pub use leptonica_sys;

pub use crate::str::Str;
pub use boxa::Boxa;
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
pub fn get_imagelib_versions() -> Option<Str> {
    unsafe {
        let pointer = getImagelibVersions();
        if pointer.is_null() {
            None
        } else {
            Some(Str::new_from_pointer(pointer))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        assert_eq!(&get_version().to_str().unwrap()[0..10], "leptonica-");
    }

    #[test]
    fn test_get_imagelib_versions() {
        // No assertions as there's not much we can guarantee given different compile time options.
        // Instead used when testing with valgrind to check for leaks.
        get_imagelib_versions();
    }
}
