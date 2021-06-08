use core::sync::atomic::Ordering;
use core::cell::UnsafeCell;

pub struct Atomic<T> {
    inner: UnsafeCell<T>,
}

const SIO_BASE: usize = 0xd000_0000;
const SPINLOCK_OFFSET: usize = 0x100;
const SPINLOCKS_COUNT: usize = 32;

pub type AtomicUsize = Atomic<usize>;
pub type AtomicBool = Atomic<bool>;

impl<T> Atomic<T> {
    pub const fn new(initial: T) -> Self {
        Self { inner : UnsafeCell::new(initial) }
    }
}

impl<T> Atomic<T> where T: PartialEq 
    + Copy {

    pub fn load(
        &self,
         _: Ordering
    ) -> T {
        let result;
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            result = *self.inner.get();
            // Spinlock release
            core::ptr::write(sl, 0);
        }
        result
    }

    pub fn store(
        &self,
        value: T,
         _: Ordering
    ) {
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            *self.inner.get() = value;
            // Spinlock release
            core::ptr::write(sl, 0);
        }
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn compare_exchange(
        &self,
        current: T,
        new: T,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<T, T> {
        let result;
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            if *self.inner.get() == current {
                *self.inner.get() = new;
                result = Ok(new);
            }
            else {
                result = Err(current);
            }
            // Spinlock release
            core::ptr::write(sl, 0);
        }
        result
    }
    
    pub fn compare_exchange_weak(
        &self,
        current: T,
        new: T,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<T, T> {
      self.compare_exchange(current, new, _success, _failure)
    }

}


impl<T> Atomic<T> where T: PartialEq 
                          + Copy
                          + core::ops::BitAndAssign
                          + core::ops::BitOrAssign {
    
    pub fn fetch_and(&self, val: T, _: Ordering) -> T {
        let result;
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            result = *self.inner.get();
            *self.inner.get() &= val;
            // Spinlock release
            core::ptr::write(sl, 0);
        }
        result
    }

    pub fn fetch_or(&self, val: T, _: Ordering) -> T {
        let result;
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            result = *self.inner.get();
            *self.inner.get() |= val;
            // Spinlock release
            core::ptr::write(sl, 0);
        }
        result
    }
}

impl<T> Atomic<T> where T: PartialEq 
                        + Copy
                        + core::ops::AddAssign
                        + core::ops::SubAssign {
    pub fn fetch_add(&self, val: T, _: Ordering) -> T {
        let result;
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            result = *self.inner.get();
            *self.inner.get() += val;
            // Spinlock release
            core::ptr::write(sl, 0);
        }
        result
    }

    pub fn fetch_sub(&self, val: T, _: Ordering) -> T {
        let result;
        unsafe {
            let sl = (SIO_BASE+SPINLOCK_OFFSET) as *mut u32;
            let mut res = 0;
            while res == 0 {
                res = core::ptr::read(sl);
            }
            // Spinlock locked
            result = *self.inner.get();
            *self.inner.get() -= val;
            // Spinlock release
            core::ptr::write(sl, 0);
        }
        result
    }
}