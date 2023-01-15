extern crate leptonica_sys;
use self::leptonica_sys::{l_int32, pixGetHeight, pixGetWidth};
use std::marker::PhantomData;

pub trait BorrowedPixMethods {
    /// Wrapper for [`pixGetHeight`](https://tpgit.github.io/Leptonica/pix1_8c.html#ae40704b3acbd343639e9aed696da531f)
    fn get_height(&self) -> l_int32;
    /// Wrapper for [`pixGetWidth`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#aa71e0b02548a56e723c76996ab145257)
    fn get_width(&self) -> l_int32;
}

impl BorrowedPixMethods for leptonica_sys::Pix {
    fn get_height(&self) -> l_int32 {
        unsafe { pixGetHeight(self) }
    }

    fn get_width(&self) -> l_int32 {
        unsafe { pixGetWidth(self) }
    }
}

/// Borrowed wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedPix<'a> {
    raw: *mut leptonica_sys::Pix,
    phantom: PhantomData<&'a *mut leptonica_sys::Pix>,
}

impl<'a> AsRef<leptonica_sys::Pix> for BorrowedPix<'a> {
    fn as_ref(&self) -> &leptonica_sys::Pix {
        unsafe { &*self.raw }
    }
}

impl<'a> BorrowedPix<'a> {
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
}
