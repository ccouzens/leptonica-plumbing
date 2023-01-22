use leptonica_sys::l_int32;

use crate::{BorrowedPixWrapper, Pix};

pub trait BorrowedPixa {
    /// Wrapper for [`pixaGetCount`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a098d32e94acc16c5d6e91a930a4b45ff)
    fn get_count(&self) -> l_int32;
    /// Wrapper for [`pixaGetPix`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a3f62a77cf11114981267a6cf9918fc45) with copied `accessflag`: `L_COPY`
    fn get_pix_copied(&self, index: l_int32) -> Option<Pix>;
    /// Wrapper for [`pixaGetPix`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a3f62a77cf11114981267a6cf9918fc45) with copied `accessflag`: `L_CLONE`
    fn get_pix_cloned(&self, index: l_int32) -> Option<BorrowedPixWrapper>;
}
