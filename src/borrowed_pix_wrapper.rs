extern crate leptonica_sys;

use crate::BorrowedPix;

/// Borrowed wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedPixWrapper<'a>(&'a *mut leptonica_sys::Pix);

impl<'a> AsRef<leptonica_sys::Pix> for BorrowedPixWrapper<'a> {
    fn as_ref(&self) -> &leptonica_sys::Pix {
        unsafe { &**self.0 }
    }
}

impl<'a> BorrowedPixWrapper<'a> {
    /// Create a new BorrowedPix from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pix struct.
    /// The pix must not be mutated whilst the BorrowedPix exists.
    pub unsafe fn new(p: &'a *mut leptonica_sys::Pix) -> Self {
        Self(p)
    }

    pub fn as_borrowed_pix(&self) -> impl BorrowedPix + '_ {
        unsafe { **self.0 }
    }
}
