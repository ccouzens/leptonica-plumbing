use leptonica_sys::l_int32;

use crate::{BorrowedBoxWrapper, Box};

pub trait BorrowedBoxa {
    /// Wrapper for [`boxaGetCount`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#a82555cab9ef5578c4728ef5109264723)
    fn get_count(&self) -> l_int32;

    /// Wrapper for [`boxaGetBox`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ac7c6fcfadf130bfa738ce6aab51318e5) with copied `accessflag`: `L_COPY`
    fn get_box_copied(&self, index: l_int32) -> Option<Box>;
    /// Wrapper for [`boxaGetBox`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ac7c6fcfadf130bfa738ce6aab51318e5) with copied `accessflag`: `L_CLONE`
    fn get_box_cloned(&self, index: l_int32) -> Option<BorrowedBoxWrapper>;
}
