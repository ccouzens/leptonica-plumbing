use leptonica_sys::{l_int32, l_ok};

pub trait BorrowedBox {
    /// Wrapper for [`boxGetGeometry`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#aaf754e00c062c3f0f726bea73a17e646)
    fn get_geometry(
        &self,
        px: Option<&mut l_int32>,
        py: Option<&mut l_int32>,
        pw: Option<&mut l_int32>,
        ph: Option<&mut l_int32>,
    ) -> l_ok;
}
