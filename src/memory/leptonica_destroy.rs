/// Trait to define how leptonica frees this memory.
///
/// Usually this would be to call a `destroy` method that decrements a reference count.
/// Sometimes a reference count isn't used, so this isn't called (eg `BorrowedFrom`).
pub trait LeptonicaDestroy {
    fn destroy(&mut self);
}
