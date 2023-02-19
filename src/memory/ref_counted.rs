use super::leptonica_destroy::LeptonicaDestroy;
use std::ops::Deref;

/// A wrapper for ref counted leptonica pointers.
pub struct RefCounted<T: LeptonicaDestroy> {
    inner: T,
}

impl<T: LeptonicaDestroy> RefCounted<T> {
    /// Creates a new ref counted wrapper
    ///
    /// # Safety
    ///
    /// It must be safe for this wrapper to destroy (decrement the ref count).
    /// The ref count must have already been incremented before being passed to `new`.
    /// The pointer must not be mutated whilst this wrapper exists.
    pub unsafe fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: LeptonicaDestroy> Drop for RefCounted<T> {
    fn drop(&mut self) {
        unsafe { self.inner.destroy() }
    }
}

impl<T: LeptonicaDestroy> Deref for RefCounted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
