use std::marker::PhantomData;

use leptonica_sys::{boxDestroy, boxGetGeometry, l_int32, l_ok};

use crate::borrowed_box::BorrowedBox;

/// Borrowed wrapper around Leptonica's [`Box`](https://tpgit.github.io/Leptonica/struct_box.html) structure
#[derive(Debug)]
pub struct BorrowedBoxWrapper<'a> {
    raw: *mut leptonica_sys::Box,
    phantom: PhantomData<&'a *mut leptonica_sys::Box>,
}

impl Drop for BorrowedBoxWrapper<'_> {
    fn drop(&mut self) {
        unsafe {
            boxDestroy(&mut self.raw);
        }
    }
}

impl<'a> AsRef<leptonica_sys::Box> for BorrowedBoxWrapper<'a> {
    fn as_ref(&self) -> &leptonica_sys::Box {
        unsafe { &*self.raw }
    }
}

impl<'a> BorrowedBoxWrapper<'a> {
    /// Create a new BorrowedBoxWrapper from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Box struct.
    /// The box must not be mutated whilst the BorrowedBoxWrapper exists.
    pub unsafe fn new_from_pointer(b: *mut leptonica_sys::Box) -> Self {
        Self {
            raw: b,
            phantom: PhantomData,
        }
    }

    pub fn as_borrowed_box(&self) -> impl BorrowedBox + '_ {
        self
    }
}

impl<'a> BorrowedBox for &BorrowedBoxWrapper<'a> {
    fn get_geometry(
        &self,
        px: Option<&mut l_int32>,
        py: Option<&mut l_int32>,
        pw: Option<&mut l_int32>,
        ph: Option<&mut l_int32>,
    ) -> l_ok {
        unsafe {
            boxGetGeometry(
                self.raw,
                match px {
                    None => std::ptr::null_mut(),
                    Some(px) => px,
                },
                match py {
                    None => std::ptr::null_mut(),
                    Some(py) => py,
                },
                match pw {
                    None => std::ptr::null_mut(),
                    Some(pw) => pw,
                },
                match ph {
                    None => std::ptr::null_mut(),
                    Some(ph) => ph,
                },
            )
        }
    }
}
