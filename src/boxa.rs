use std::convert::TryInto;

use leptonica_sys::{boxaCreate, boxaDestroy, boxaGetBox, boxaGetCount, l_int32, L_CLONE, L_COPY};

use crate::{
    memory::{LeptonicaDestroy, RefCounted, RefCountedExclusive},
    Box,
};

/// Wrapper around Leptonica's [`Boxa`](https://tpgit.github.io/Leptonica/struct_boxa.html) structure
#[derive(Debug, PartialEq)]
pub struct Boxa(*mut leptonica_sys::Boxa);

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
    pub fn create(n: l_int32) -> Option<RefCountedExclusive<Boxa>> {
        let ptr = unsafe { boxaCreate(n) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { RefCountedExclusive::new(Self(ptr)) })
        }
    }

    /// Wrapper for [`boxaGetCount`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#a82555cab9ef5578c4728ef5109264723)
    pub fn get_count(&self) -> l_int32 {
        unsafe { boxaGetCount(self.0) }
    }

    /// Wrapper for [`boxaGetBox`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ac7c6fcfadf130bfa738ce6aab51318e5) with copied `accessflag`: `L_COPY`
    pub fn get_box_copied(&self, index: l_int32) -> Option<RefCountedExclusive<Box>> {
        unsafe {
            boxaGetBox(self.0, index, L_COPY.try_into().unwrap())
                .as_mut()
                .map(|raw| RefCountedExclusive::new(Box::new_from_pointer(raw)))
        }
    }

    /// Wrapper for [`boxaGetBox`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ac7c6fcfadf130bfa738ce6aab51318e5) with copied `accessflag`: `L_CLONE`
    pub fn get_box_cloned(&self, index: l_int32) -> Option<RefCounted<Box>> {
        unsafe {
            boxaGetBox(self.0, index, L_CLONE.try_into().unwrap())
                .as_mut()
                .map(|raw| RefCounted::new(Box::new_from_pointer(raw)))
        }
    }
}

impl LeptonicaDestroy for Boxa {
    fn destroy(&mut self) {
        unsafe {
            boxaDestroy(&mut self.0);
        }
    }
}

#[test]
fn create_valid_test() {
    let boxa = Boxa::create(4).unwrap();
    assert_eq!(boxa.get_count(), 0);
}
