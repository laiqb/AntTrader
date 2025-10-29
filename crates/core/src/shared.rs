use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

#[repr(transparent)]
#[derive(Debug)]

pub struct SharedCell<T>(Rc<RefCell<T>>);

impl<T> Clone for SharedCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}


impl<T> SharedCell<T> {
    /// Wraps a value inside `Rc<RefCell<..>>`.
    #[inline]
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }

    /// Creates a [`WeakCell`] pointing to the same allocation.
    #[inline]
    #[must_use]
    pub fn downgrade(&self) -> WeakCell<T> {
        WeakCell(Rc::downgrade(&self.0))
    }

    /// Immutable borrow of the inner value.
    #[inline]
    #[must_use]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    /// Mutable borrow of the inner value.
    #[inline]
    #[must_use]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

    /// Number of active strong references.
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }

    /// Number of active weak references.
    #[inline]
    #[must_use]
    pub fn weak_count(&self) -> usize {
        Rc::weak_count(&self.0)
    }
}

impl<T> From<Rc<RefCell<T>>> for SharedCell<T> {
    fn from(inner: Rc<RefCell<T>>) -> Self {
        Self(inner)
    }
}

impl<T> From<SharedCell<T>> for Rc<RefCell<T>> {
    fn from(shared: SharedCell<T>) -> Self {
        shared.0
    }
}

impl<T> std::ops::Deref for SharedCell<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Weak counterpart to [`SharedCell`].
#[repr(transparent)]
#[derive(Debug)]
pub struct WeakCell<T>(Weak<RefCell<T>>);

impl<T> Clone for WeakCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> WeakCell<T> {
    /// Attempts to upgrade the weak reference to a strong [`SharedCell`].
    #[inline]
    pub fn upgrade(&self) -> Option<SharedCell<T>> {
        self.0.upgrade().map(SharedCell)
    }

    /// Returns `true` if the pointed-to value has been dropped.
    #[inline]
    #[must_use]
    pub fn is_dropped(&self) -> bool {
        self.0.strong_count() == 0
    }
}

impl<T> From<Weak<RefCell<T>>> for WeakCell<T> {
    fn from(inner: Weak<RefCell<T>>) -> Self {
        Self(inner)
    }
}

impl<T> From<WeakCell<T>> for Weak<RefCell<T>> {
    fn from(cell: WeakCell<T>) -> Self {
        cell.0
    }
}
