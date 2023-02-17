use crate::memory::{LeptonicaDestroy, RefCountedExclusive};

use leptonica_sys::{boxCreateValid, boxDestroy, boxGetGeometry, l_int32, l_ok};
use thiserror::Error;

/// Wrapper around Leptonica's [`Box`](https://tpgit.github.io/Leptonica/struct_box.html) structure
#[derive(Debug, PartialEq)]
pub struct Box(*mut leptonica_sys::Box);

/// Error returned by Box::create_valid
#[derive(Debug, Error)]
#[error("Box::create_valid returned null")]
pub struct BoxCreateValidError();

impl AsRef<leptonica_sys::Box> for Box {
    fn as_ref(&self) -> &leptonica_sys::Box {
        unsafe { &*self.0 }
    }
}

impl AsMut<leptonica_sys::Box> for Box {
    fn as_mut(&mut self) -> &mut leptonica_sys::Box {
        unsafe { &mut *self.0 }
    }
}

impl Box {
    /// Create an owned Box from a box pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Box struct.
    /// The data pointed at may not be mutated while held by
    /// this struct except by this struct.
    /// On drop, the destroy method will be called (decrements
    /// the ref counter).
    pub unsafe fn new_from_pointer(b: *mut leptonica_sys::Box) -> Self {
        Self(b)
    }

    /// Wrapper for [`boxCreateValid`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#a435610d86a8562dc60bfd75fe0a15420)
    ///
    /// Input: x, y, w, h Return: box
    pub fn create_valid(
        x: l_int32,
        y: l_int32,
        w: l_int32,
        h: l_int32,
    ) -> Result<RefCountedExclusive<Self>, BoxCreateValidError> {
        let ptr = unsafe { boxCreateValid(x, y, w, h) };
        if ptr.is_null() {
            Err(BoxCreateValidError())
        } else {
            Ok(unsafe { RefCountedExclusive::new(Self(ptr)) })
        }
    }

    /// Wrapper for [`boxGetGeometry`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#aaf754e00c062c3f0f726bea73a17e646)
    pub fn get_geometry(
        &self,
        px: Option<&mut l_int32>,
        py: Option<&mut l_int32>,
        pw: Option<&mut l_int32>,
        ph: Option<&mut l_int32>,
    ) -> l_ok {
        unsafe {
            boxGetGeometry(
                self.0,
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

impl LeptonicaDestroy for Box {
    unsafe fn destroy(&mut self) {
        boxDestroy(&mut self.0);
    }
}

#[test]
fn create_valid_test() {
    let r#box = Box::create_valid(1, 2, 3, 4).unwrap();
    let mut pw = 0;
    r#box.get_geometry(None, None, Some(&mut pw), None);
    assert_eq!(pw, 3);
}

#[test]
fn create_invalid_test() {
    assert!(Box::create_valid(1, 2, 3, -4).is_err())
}
