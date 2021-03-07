use std::marker::PhantomData;

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
    pub unsafe fn new(p: *mut leptonica_sys::Pix) -> Self {
        Self {
            raw: p,
            phantom: PhantomData,
        }
    }
}
