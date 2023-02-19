use std::{marker::PhantomData, ops::Deref};

/// A non-ref counted wrapper that's valid for the lifetime of a parent struct
pub struct BorrowedFrom<'a, T: 'a> {
    inner: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> BorrowedFrom<'a, T> {
    /// Creates a new borrowed-from wrapper
    ///
    /// # Safety
    ///
    /// The pointer must not be mutated whilst this wrapper exists.
    pub unsafe fn new(inner: T) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a> Deref for BorrowedFrom<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
