/// Trait to define how leptonica clones this memory.
///
/// Usually this would be to call a `clone` method that increments a reference count.
pub trait LeptonicaClone {
    /// Call to leptonica's internal structure-clone method.
    ///
    /// # Safety
    ///
    /// This must eventually be accompanied by a destroy call to decrement the reference count and possibly free the memory.
    ///
    /// The memory will be shared, so must not be mutated.
    unsafe fn clone(&mut self) -> Self;
}
