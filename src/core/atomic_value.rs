use std::{
  ops::{Deref, DerefMut},
  sync::atomic::{AtomicPtr, Ordering},
};

pub struct AtomicValue<T>(AtomicPtr<T>);

unsafe impl<T> Send for AtomicValue<T> {}
unsafe impl<T> Sync for AtomicValue<T> {}

impl<T> AtomicValue<T> {
  pub fn new(value: T) -> Self {
    let ptr = Box::into_raw(Box::new(value));
    Self(AtomicPtr::new(ptr))
  }

  pub const fn empty() -> Self {
    Self(AtomicPtr::new(std::ptr::null_mut()))
  }

  pub fn is_empty(&self) -> bool {
    self.0.load(Ordering::Relaxed).is_null()
  }

  pub fn store(&self, value: T, ordering: Ordering) {
    let new_ptr = Box::into_raw(Box::new(value));
    let prev_ptr = self.0.swap(new_ptr, ordering);
    if prev_ptr.is_null() {
      return;
    }
    unsafe {
      drop(Box::from_raw(prev_ptr));
    }
  }

  pub fn set(&self, value: T, ordering: Ordering) {
    self.store(value, ordering);
  }

  pub fn as_ref(&self, ordering: Ordering) -> &T {
    let ptr = self.0.load(ordering);
    unsafe { &*ptr }
  }

  pub fn as_mut(&self, ordering: Ordering) -> &mut T {
    let ptr = self.0.load(ordering);
    unsafe { &mut *ptr }
  }
}

impl<T> AtomicValue<T>
where
  T: Clone,
{
  pub fn load(&self, ordering: Ordering) -> T {
    let ptr = self.0.load(ordering);
    unsafe { (*ptr).clone() }
  }

  pub fn get(&self, ordering: Ordering) -> T {
    self.load(ordering)
  }
}

impl<T> Deref for AtomicValue<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.as_ref(Ordering::Relaxed)
  }
}

impl<T> DerefMut for AtomicValue<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut(Ordering::Relaxed)
  }
}

impl<T> Drop for AtomicValue<T> {
  fn drop(&mut self) {
    let ptr = self.0.load(Ordering::Relaxed);
    if ptr.is_null() {
      return;
    }
    unsafe { drop(Box::from_raw(ptr)) };
  }
}
