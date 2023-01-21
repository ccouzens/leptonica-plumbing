extern crate leptonica_sys;
use self::leptonica_sys::{l_int32, pixGetHeight, pixGetWidth};

pub trait BorrowedPix {
    /// Wrapper for [`pixGetHeight`](https://tpgit.github.io/Leptonica/pix1_8c.html#ae40704b3acbd343639e9aed696da531f)
    fn get_height(&self) -> l_int32;
    /// Wrapper for [`pixGetWidth`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#aa71e0b02548a56e723c76996ab145257)
    fn get_width(&self) -> l_int32;
}

impl BorrowedPix for leptonica_sys::Pix {
    fn get_height(&self) -> l_int32 {
        unsafe { pixGetHeight(self) }
    }

    fn get_width(&self) -> l_int32 {
        unsafe { pixGetWidth(self) }
    }
}
