use leptonica_sys::{l_int32, pixDestroy, pixGetHeight, pixGetWidth};
use std::marker::PhantomData;

use crate::BorrowedPix;

/// Borrowed wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedPixWrapper<'a> {
    pub(crate) raw: *mut leptonica_sys::Pix,
    pub(crate) phantom: PhantomData<&'a *mut leptonica_sys::Pix>,
}

impl Drop for BorrowedPixWrapper<'_> {
    fn drop(&mut self) {
        unsafe {
            pixDestroy(&mut self.raw);
        }
    }
}

impl<'a> AsRef<leptonica_sys::Pix> for BorrowedPixWrapper<'a> {
    fn as_ref(&self) -> &leptonica_sys::Pix {
        unsafe { &*self.raw }
    }
}

impl<'a> BorrowedPixWrapper<'a> {
    /// Create a new BorrowedPix from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pix struct.
    /// The pix must not be mutated whilst the BorrowedPix exists.
    pub unsafe fn new(p: *mut leptonica_sys::Pix) -> Self {
        Self {
            raw: p,
            phantom: PhantomData,
        }
    }

    pub fn as_borrowed_pix(&self) -> impl BorrowedPix + '_ {
        self
    }
}

impl<'a> BorrowedPix for &BorrowedPixWrapper<'a> {
    fn get_height(&self) -> l_int32 {
        unsafe { pixGetHeight(self.raw) }
    }

    fn get_width(&self) -> l_int32 {
        unsafe { pixGetWidth(self.raw) }
    }
}
