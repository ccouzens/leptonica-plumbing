use std::convert::TryInto;

use leptonica_sys::{boxaCreate, boxaDestroy, boxaGetBox, boxaGetCount, l_int32, L_CLONE, L_COPY};

use crate::{borrowed_boxa::BorrowedBoxa, BorrowedBoxWrapper, Box};

/// Wrapper around Leptonica's [`Boxa`](https://tpgit.github.io/Leptonica/struct_boxa.html) structure
#[derive(Debug, PartialEq)]
pub struct Boxa(*mut leptonica_sys::Boxa);

impl Drop for Boxa {
    fn drop(&mut self) {
        unsafe {
            boxaDestroy(&mut self.0);
        }
    }
}

impl AsRef<leptonica_sys::Boxa> for Boxa {
    fn as_ref(&self) -> &leptonica_sys::Boxa {
        unsafe { &*self.0 }
    }
}

impl Boxa {
    /// Create a new Boxa from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Boxa struct.
    /// The Boxa struct must not be mutated whilst the wrapper exists.
    pub unsafe fn new_from_pointer(p: *mut leptonica_sys::Boxa) -> Self {
        Self(p)
    }

    /// Wrapper for [`boxaCreate`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ae59916b7506831be9bf2119dea063253)
    ///
    /// Input: n (initial number of ptrs) Return: boxa, or null on error
    pub fn create(n: l_int32) -> Option<Boxa> {
        let ptr = unsafe { boxaCreate(n) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    pub fn as_borrowed_boxa(&self) -> impl BorrowedBoxa + '_ {
        self
    }
}

impl BorrowedBoxa for &Boxa {
    fn get_count(&self) -> l_int32 {
        unsafe { boxaGetCount(self.0) }
    }

    fn get_box_copied(&self, index: l_int32) -> Option<Box> {
        unsafe {
            boxaGetBox(self.0, index, L_COPY.try_into().unwrap())
                .as_mut()
                .map(|raw| Box::new(raw))
        }
    }

    fn get_box_cloned(&self, index: l_int32) -> Option<BorrowedBoxWrapper> {
        unsafe {
            boxaGetBox(self.0, index, L_CLONE.try_into().unwrap())
                .as_mut()
                .map(|raw| BorrowedBoxWrapper::new(raw))
        }
    }
}

#[test]
fn create_valid_test() {
    let boxa = Boxa::create(4).unwrap();
    assert_eq!(boxa.as_borrowed_boxa().get_count(), 0);
}
