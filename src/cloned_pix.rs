use leptonica_sys::{l_int32, pixDestroy, pixGetHeight, pixGetWidth};
use std::marker::PhantomData;

use crate::BorrowedPix;

/// Borrowed ref counted wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure.
/// Leptonica uses the term cloned to mean ref-counted.
#[derive(Debug, PartialEq)]
pub struct ClonedPix<'a> {
    raw: *mut leptonica_sys::Pix,
    phantom: PhantomData<&'a *mut leptonica_sys::Pix>,
}

impl Drop for ClonedPix<'_> {
    fn drop(&mut self) {
        unsafe {
            pixDestroy(&mut self.raw);
        }
    }
}

impl<'a> AsRef<leptonica_sys::Pix> for ClonedPix<'a> {
    fn as_ref(&self) -> &leptonica_sys::Pix {
        unsafe { &*self.raw }
    }
}

impl<'a> ClonedPix<'a> {
    /// Create a new ClonedPix from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pix struct.
    /// The pix must not be mutated whilst the BorrowedPix exists and will be destroyed on Drop.
    pub unsafe fn new_from_pointer(p: *mut leptonica_sys::Pix) -> Self {
        Self {
            raw: p,
            phantom: PhantomData,
        }
    }

    pub fn as_borrowed_pix(&self) -> impl BorrowedPix + '_ {
        self
    }
}

impl<'a> BorrowedPix for &ClonedPix<'a> {
    fn get_height(&self) -> l_int32 {
        unsafe { pixGetHeight(self.raw) }
    }

    fn get_width(&self) -> l_int32 {
        unsafe { pixGetWidth(self.raw) }
    }
}
