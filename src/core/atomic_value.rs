use std::sync::{
  atomic::{AtomicPtr, Ordering},
  Arc,
};

pub struct AtomicValue<T>(AtomicPtr<Arc<T>>);

unsafe impl<T> Send for AtomicValue<T> {}
unsafe impl<T> Sync for AtomicValue<T> {}

impl<T> AtomicValue<T> {
  pub fn new(value: T) -> Self {
    let ptr = Box::into_raw(Box::new(Arc::new(value)));
    Self(AtomicPtr::new(ptr))
  }

  pub fn store(&self, value: T, ordering: Ordering) {
    let new_ptr = Box::into_raw(Box::new(Arc::new(value)));
    let prev_ptr = self.0.swap(new_ptr, ordering);
    unsafe {
      drop(Box::from_raw(prev_ptr));
    }
  }

  pub fn set(&self, value: T, ordering: Ordering) {
    self.store(value, ordering);
  }

  pub fn get(&self, ordering: Ordering) -> Arc<T> {
    let arc = unsafe { &*self.0.load(ordering) };
    arc.clone()
  }
}

impl<T> Drop for AtomicValue<T> {
  fn drop(&mut self) {
    let ptr = self.0.load(Ordering::Relaxed);
    unsafe { drop(Box::from_raw(ptr)) };
  }
}
