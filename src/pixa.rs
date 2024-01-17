use crate::{
    memory::{LeptonicaDestroy, RefCounted, RefCountedExclusive},
    Pix,
};
use leptonica_sys::{
    pixaDestroy, pixaGetCount, pixaGetPix, pixaReadMultipageTiff, L_CLONE, L_COPY,
};
use std::{convert::TryInto, ffi::CStr};

/// Wrapper around Leptonica's [`Pixa`](https://tpgit.github.io/Leptonica/struct_pixa.html) structure
#[derive(Debug, PartialEq)]
pub struct Pixa(*mut leptonica_sys::Pixa);

impl AsRef<leptonica_sys::Pixa> for Pixa {
    fn as_ref(&self) -> &leptonica_sys::Pixa {
        unsafe { &*self.0 }
    }
}

impl AsMut<leptonica_sys::Pixa> for Pixa {
    fn as_mut(&mut self) -> &mut leptonica_sys::Pixa {
        unsafe { &mut *self.0 }
    }
}

impl Pixa {
    /// Create a new Pixa from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pixa struct.
    /// The Pixa struct must not be mutated whilst the wrapper exists.
    pub unsafe fn new_from_pointer(p: *mut leptonica_sys::Pixa) -> Self {
        Self(p)
    }

    /// Wrapper for [`pixaReadMultipageTiff`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a4a52e686cf67f0e5bfda661fc3a3fb7b)
    pub fn read_multipage_tiff(filename: &CStr) -> Option<RefCountedExclusive<Self>> {
        let ptr = unsafe { pixaReadMultipageTiff(filename.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { RefCountedExclusive::new(Self(ptr)) })
        }
    }

    /// Wrapper for [`pixaGetCount`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a098d32e94acc16c5d6e91a930a4b45ff)
    pub fn get_count(&self) -> leptonica_sys::l_int32 {
        unsafe { pixaGetCount(self.0) }
    }

    /// Wrapper for [`pixaGetPix`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a3f62a77cf11114981267a6cf9918fc45) with copied `accessflag`: `L_COPY`
    pub fn get_pix_copied(
        &self,
        index: leptonica_sys::l_int32,
    ) -> Option<RefCountedExclusive<Pix>> {
        unsafe {
            pixaGetPix(self.0, index, L_COPY.try_into().unwrap())
                .as_mut()
                .map(|raw| RefCountedExclusive::new(Pix::new_from_pointer(raw)))
        }
    }

    pub fn get_pix_cloned(&self, index: leptonica_sys::l_int32) -> Option<RefCounted<Pix>> {
        unsafe {
            pixaGetPix(self.0, index, L_CLONE.try_into().unwrap())
                .as_mut()
                .map(|raw| RefCounted::new(Pix::new_from_pointer(raw)))
        }
    }
}

impl LeptonicaDestroy for Pixa {
    unsafe fn destroy(&mut self) {
        pixaDestroy(&mut self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_multipage_tiff_test() {
        let pixa =
            Pixa::read_multipage_tiff(CStr::from_bytes_with_nul(b"multipage.tiff\0").unwrap())
                .unwrap();
        assert_eq!(pixa.get_count(), 2);
        assert_eq!(pixa.get_pix_copied(0).unwrap().get_width(), 165);
        assert_eq!(pixa.get_pix_cloned(0).unwrap().get_height(), 67);
        assert_eq!(pixa.get_pix_copied(1).unwrap().get_width(), 165);
        assert_eq!(pixa.get_pix_cloned(1).unwrap().get_height(), 67);
        assert!(pixa.get_pix_copied(2).is_none());
        assert!(pixa.get_pix_cloned(2).is_none());
    }
}
