//! Synchronization primitives

use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use kernel::svc::{arbitrate_lock, arbitrate_unlock};
use core::fmt;
use core::ops::{Deref, DerefMut};

const HAS_LISTENERS: u32 = 0x40000000;

#[repr(transparent)]
#[derive(Debug)]
pub struct Mutex(AtomicUsize);

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex(AtomicUsize::new(0))
    }

    pub fn lock(&self) {
        let tag = ::tls::TlsStruct::get_thread_handle().unwrap_or(0);

        // A note about 0: when trying to lock mutexes very early in the init
        // (before TlsStruct::set_thread_handle is called), we will have a problem:
        // we do not yet know what our thread_handle is. We might still want to
        // lock those structs, however. What we do is, we get the handle as 0.
        //
        // This will let the CAS succeed, without actually locking the lock.
        // This is, obviously, unsafe. This is mostly useful for the earlydebug
        // logging mechanism. Only Loader::init is called in this weird state.
        loop {
            let cur = self.0.compare_and_swap(0, tag as usize, Ordering::SeqCst) as u32;

            if cur == 0 {
                // We won the race.
                return;
            }

            if cur & !HAS_LISTENERS == tag {
                // Kernel assigned it to us
                return;
            }

            if cur & HAS_LISTENERS != 0 {
                // The flag is already set, we can use the syscall.
                unsafe { arbitrate_lock(cur & !HAS_LISTENERS, self as *const _ as *mut _, tag); }
            } else {
                // The flag is not set, we need to set it
                let oldval = self.0.compare_and_swap(cur as usize, (cur | HAS_LISTENERS) as usize, Ordering::SeqCst) as u32;

                if oldval == cur {
                    // Flag was set successfully
                    unsafe { arbitrate_lock(cur, self as *const _ as *mut _, tag); }
                }
            }
        }
    }

    pub fn try_lock(&self) -> bool {
        let tag = ::tls::TlsStruct::get_thread_handle().unwrap_or(0);

        let cur = self.0.compare_and_swap(0, tag as usize, Ordering::SeqCst) as u32;

        if cur == 0 {
            // We won the race.
            return true;
        }

        if cur & !HAS_LISTENERS == tag {
            // Kernel assigned it to us
            return true;
        }

        return false;
    }

    pub fn unlock(&self) {
        let tag = ::tls::TlsStruct::get_thread_handle().unwrap_or(0);
        let old = self.0.compare_and_swap(tag as usize, 0, Ordering::SeqCst) as u32;
        if old & HAS_LISTENERS != 0 {
            unsafe { arbitrate_unlock(self as *const _ as *mut _); }
        }
    }
}


/// A lock used internally by megaton-hammer, based on spin, but using the
/// locking facility provided by the Kernel.
///
/// Homebrew should be using libstd Mutexes instead!
pub struct InternalMutex<T: ?Sized> {
    lock: Mutex,
    data: UnsafeCell<T>
}


/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
pub struct InternalMutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex,
    data: &'a mut T
}

unsafe impl<T: ?Sized + Send> Sync for InternalMutex<T> {}
unsafe impl<T: ?Sized + Send> Send for InternalMutex<T> {}

impl<T> InternalMutex<T> {
    /// Creates a new lock wrapping the supplied data.
    pub const fn new(user_data: T) -> InternalMutex<T> {
        InternalMutex {
            lock: Mutex::new(),
            data: UnsafeCell::new(user_data),
        }
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let InternalMutex { data, .. } = self;
        data.into_inner()
    }

    /// Unsafely gets access to the inner content.
    pub unsafe fn force_inner(&self) -> &mut T {
        &mut *self.data.get()
    }
}

impl<T: ?Sized> InternalMutex<T>
{
    /// Locks the spinlock and returns a guard.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    ///
    /// ```
    /// let mylock = spin::InternalMutex::new(0);
    /// {
    ///     let mut data = mylock.lock();
    ///     // The lock is now locked and the data can be accessed
    ///     *data += 1;
    ///     // The lock is implicitly dropped
    /// }
    ///
    /// ```
    pub fn lock(&self) -> InternalMutexGuard<T>
    {
        self.lock.lock();
        InternalMutexGuard
        {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }

    /// Tries to lock the InternalMutex. If it is already locked, it will return None. Otherwise it returns
    /// a guard within Some.
    pub fn try_lock(&self) -> Option<InternalMutexGuard<T>>
    {
        if self.lock.try_lock() {
            Some(
                InternalMutexGuard {
                    lock: &self.lock,
                    data: unsafe { &mut *self.data.get() },
                }
            )
        } else {
            None
        }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for InternalMutex<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.try_lock()
        {
            Some(guard) => write!(f, "InternalMutex {{ data: {:?} }}", &*guard),
            None => write!(f, "InternalMutex {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default> Default for InternalMutex<T> {
    fn default() -> InternalMutex<T> {
        InternalMutex::new(Default::default())
    }
}


impl<'a, T: ?Sized> Deref for InternalMutexGuard<'a, T>
{
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T { &*self.data }
}

impl<'a, T: ?Sized> DerefMut for InternalMutexGuard<'a, T>
{
    fn deref_mut<'b>(&'b mut self) -> &'b mut T { &mut *self.data }
}

impl<'a, T: ?Sized> Drop for InternalMutexGuard<'a, T>
{
    /// The dropping of the InternalMutexGuard will release the lock it was created from.
    fn drop(&mut self)
    {
        self.lock.unlock();
    }
}
