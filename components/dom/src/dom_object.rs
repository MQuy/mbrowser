use std::{ops::Deref, ptr::NonNull};

pub struct Dom<T> {
    ptr: NonNull<T>,
}

impl<T> Clone for Dom<T> {
    fn clone(&self) -> Self {
        Dom {
            ptr: self.ptr.clone(),
        }
    }
}

impl<T> Deref for Dom<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // We can only have &Dom<T> from a rooted thing, so it's safe to deref
        // it to &T.
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T> Dom<T> {
    /// Create a Dom<T> from a &T
    pub fn from_ref(obj: &T) -> Dom<T> {
        Dom {
            ptr: NonNull::from(obj),
        }
    }
}
