use core::sync::atomic::Ordering;


pub struct Atomic<T> where T: PartialEq + Copy {
    inner: T,
}

const SIO_BASE: usize = 0xd000_0000;
const SPINLOCK_OFFSET: usize = 0x100;
const SPINLOCKS_COUNT: usize = 32;

type AtomicUsize = Atomic<usize>;
type AtomicBool = Atomic<bool>;

impl<T> Atomic<T> where T: PartialEq + Copy{
    pub fn new(initial: T) -> Self {
      Self {
        inner : initial,
      }
    }

    pub fn compare_exchange(
        &mut self,
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
                res = std::ptr::read(sl);
            }
            // Spinlock locked
            if self.inner == current {
                self.inner = new;
                result = Ok(new);
            }
            else {
                result = Err(current);
            }
            // Spinlock release
            std::ptr::write(sl, 0);
        }
        result
    }
}