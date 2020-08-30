use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::sync::atomic::{AtomicBool, Ordering};

struct WriteOnceLock<T> {
    data: AtomicRefCell<Option<T>>,
    has_written: AtomicBool,
}

impl<T> WriteOnceLock<T> {
    pub fn has_written(&self) -> bool {
        self.has_written.load(Ordering::SeqCst)
    }

    fn write_once(&self) -> AtomicRefMut<Option<T>> {
        let data = self.data.borrow_mut();

        if data.is_some() {
            panic!("Attempted to write to WriteOnceLock more than once.");
        }

        // TODO: can we remove the Option, since it must be Some?
        data
    }

    fn read(&self) -> AtomicRef<Option<T>> {
        self.data.borrow()
    }
}
