extern crate leptonica_sys;
extern crate thiserror;
use self::leptonica_sys::{l_int32, pixGetHeight, pixGetWidth};

use crate::BorrowedPix;

use self::leptonica_sys::{pixDestroy, pixRead, pixReadMem};
use self::thiserror::Error;
use std::convert::{AsRef, TryInto};
use std::{ffi::CStr, num::TryFromIntError};

/// Wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
#[derive(Debug)]
pub struct Pix(*mut leptonica_sys::Pix);

/// Error returned by Pix::read_mem
#[derive(Debug, Error, PartialEq)]
pub enum PixReadMemError {
    #[error("Pix::read_mem returned null")]
    NullPtr,
    #[error("Failed to convert image size")]
    ImageSizeConversion(#[from] TryFromIntError),
}

/// Error returned by Pix::read
#[derive(Debug, Error)]
#[error("Pix::read returned null")]
pub struct PixReadError();

impl Drop for Pix {
    fn drop(&mut self) {
        unsafe {
            pixDestroy(&mut self.0);
        }
    }
}

impl AsRef<*mut leptonica_sys::Pix> for Pix {
    fn as_ref(&self) -> &*mut leptonica_sys::Pix {
        &self.0
    }
}

impl AsRef<leptonica_sys::Pix> for Pix {
    fn as_ref(&self) -> &leptonica_sys::Pix {
        unsafe { &*self.0 }
    }
}

impl Pix {
    /// Create a new instance from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid `Pix` struct.
    ///
    /// The structure must not be mutated or freed outside of the Rust code.
    ///
    /// It must be safe for Rust to free the pointer. If this is not the case consider using [super::BorrowedPixWrapper::new].
    pub unsafe fn new_from_pointer(ptr: *mut leptonica_sys::Pix) -> Self {
        Self(ptr)
    }

    /// Wrapper for [`pixRead`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a84634846cbb5e01df667d6e9241dfc53)
    ///
    /// Read an image from a filename
    pub fn read(filename: &CStr) -> Result<Self, PixReadError> {
        let ptr = unsafe { pixRead(filename.as_ptr()) };
        if ptr.is_null() {
            Err(PixReadError())
        } else {
            Ok(Self(ptr))
        }
    }

    /// Wrapper for [`pixReadMem`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a027a927dc3438192e3bdae8c219d7f6a)
    ///
    /// Read an image from memory
    pub fn read_mem(img: &[u8]) -> Result<Self, PixReadMemError> {
        let ptr = unsafe { pixReadMem(img.as_ptr(), img.len().try_into()?) };
        if ptr.is_null() {
            Err(PixReadMemError::NullPtr)
        } else {
            Ok(Self(ptr))
        }
    }

    pub fn as_borrowed_pix(&self) -> impl BorrowedPix + '_ {
        self
    }
}

impl BorrowedPix for &Pix {
    fn get_height(&self) -> l_int32 {
        unsafe { pixGetHeight(self.0) }
    }

    fn get_width(&self) -> l_int32 {
        unsafe { pixGetWidth(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_error_test() {
        let path = std::ffi::CString::new("fail").unwrap();
        assert!(Pix::read(&path).is_err());
    }

    #[test]
    fn read_mem_error_test() {
        assert_eq!(Pix::read_mem(&[]).err(), Some(PixReadMemError::NullPtr));
    }

    #[test]
    fn read_test() {
        let path = std::ffi::CString::new("image.png").unwrap();
        let pix = Pix::read(&path).unwrap();
        assert_eq!(pix.as_borrowed_pix().get_width(), 200);
    }

    #[test]
    fn read_memory_test() {
        let pix = Pix::read_mem(include_bytes!("../image.png")).unwrap();
        assert_eq!(pix.as_borrowed_pix().get_height(), 23);
    }
}
