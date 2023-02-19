/// Trait to define how leptonica frees this memory.
///
/// Usually this would be to call a `destroy` method that decrements a reference count.
/// Sometimes a reference count isn't used, so this isn't called (eg `BorrowedFrom`).
pub trait LeptonicaDestroy {
    /// Call to leptonica's internal structure-destroy method.
    ///
    /// # Safety
    ///
    /// The reference to the pointer must not be used after destroy is called.
    unsafe fn destroy(&mut self);
}
