use std::sync::{Arc, Mutex, Weak};

pub trait WithLock<T> {
    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R;
    fn with_lock_try<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R>;
}

impl<T> WithLock<T> for Arc<Mutex<T>> {
    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let state = &mut self.lock().expect("Could not lock mutex");
        f(state)
    }

    fn with_lock_try<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        match &mut self.try_lock() {
            Ok(state) => Some(f(state)),
            Err(_) => None,
        }
    }
}

impl<T> WithLock<T> for Weak<Mutex<T>> {
    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let arc = self.upgrade().expect("Could not upgrade weak reference");
        let lock = &mut arc.lock().expect("Could not lock mutex");
        f(lock)
    }

    fn with_lock_try<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        match &mut self.upgrade() {
            Some(lock) => {
                return match &mut lock.try_lock() {
                    Ok(state) => Some(f(state)),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}

pub trait ArcMutexClone<T: Clone> {
    fn get_clone(&self) -> T;
    fn try_get_clone(&self) -> Option<T>;
}

impl<T: Clone> ArcMutexClone<T> for Arc<Mutex<T>> {
    fn get_clone(&self) -> T {
        let lock = self.lock().expect("Could not lock mutex");
        lock.clone()
    }

    fn try_get_clone(&self) -> Option<T> {
        match self.try_lock() {
            Ok(lock) => Some(lock.clone()),
            Err(_) => None
        }
    }
}