/// Borrowed wrapper around Leptonica's [`Box`](https://tpgit.github.io/Leptonica/struct_box.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedBoxWrapper<'a>(&'a *mut leptonica_sys::Box);

impl<'a> AsRef<leptonica_sys::Box> for BorrowedBoxWrapper<'a> {
    fn as_ref(&self) -> &leptonica_sys::Box {
        unsafe { &**self.0 }
    }
}

impl<'a> BorrowedBoxWrapper<'a> {
    /// Create a new BorrowedBoxWrapper from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Box struct.
    /// The box must not be mutated whilst the BorrowedBoxWrapper exists.
    pub unsafe fn new(b: &'a *mut leptonica_sys::Box) -> Self {
        Self(b)
    }
}
