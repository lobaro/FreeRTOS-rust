use crate::base::*;
use crate::prelude::v1::*;
use crate::shim::*;
use crate::units::*;

pub type Mutex<T> = MutexImpl<T, MutexNormal>;
pub type RecursiveMutex<T> = MutexImpl<T, MutexRecursive>;

unsafe impl<T: Sync + Send, M> Send for MutexImpl<T, M> {}

unsafe impl<T: Sync + Send, M> Sync for MutexImpl<T, M> {}

/// Mutual exclusion access to a contained value. Can be recursive -
/// the current owner of a lock can re-lock it.
pub struct MutexImpl<T: ?Sized, M> {
    mutex: M,
    data: UnsafeCell<T>,
}

impl<T: ?Sized, M> fmt::Debug for MutexImpl<T, M>
where
    M: MutexInnerImpl + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mutex address: {:?}", self.mutex)
    }
}

impl<T, M> MutexImpl<T, M>
where
    M: MutexInnerImpl,
{
    /// Create a new mutex with the given inner value
    pub fn new(value: T) -> Result<Self, FreeRtosError> {
        Ok(Self::from_parts(M::create()?, value))
    }

    /// Try to obtain a lock and mutable access to our inner value
    pub fn lock<D: DurationTicks>(&self, max_wait: D) -> Result<MutexGuard<T, M>, FreeRtosError> {
        self.mutex.take(max_wait)?;

        Ok(MutexGuard {
            __mutex: &self.mutex,
            __data: &self.data,
        })
    }

    /// Consume the mutex and return its inner value
    pub fn into_inner(self) -> T {
        self.into_parts().1
    }

    /// Get mutable reference to inner value.
    ///
    /// This method does not lock the mutex because mutable reference guarantees exclusive access.
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }

    /// Create owning mutex from non-owning mutex and inner value.
    ///
    /// It is safe to pass an already locked `mutex` although it is not recommended.
    pub fn from_parts(mutex: M, value: T) -> Self {
        Self {
            mutex,
            data: UnsafeCell::new(value),
        }
    }

    /// Split owning mutex into non-owning mutex and inner value.
    pub fn into_parts(self) -> (M, T) {
        (self.mutex, self.data.into_inner())
    }

    /// Get mutable reference to inner non-owning mutex.
    pub fn inner_mutex_mut(&mut self) -> &mut M {
        &mut self.mutex
    }
}

/// Holds the mutex until we are dropped
pub struct MutexGuard<'a, T: ?Sized + 'a, M: 'a>
where
    M: MutexInnerImpl,
{
    __mutex: &'a M,
    __data: &'a UnsafeCell<T>,
}

impl<'mutex, T: ?Sized, M> Deref for MutexGuard<'mutex, T, M>
where
    M: MutexInnerImpl,
{
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.__data.get() }
    }
}

impl<'mutex, T: ?Sized, M> DerefMut for MutexGuard<'mutex, T, M>
where
    M: MutexInnerImpl,
{
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.__data.get() }
    }
}

impl<'a, T: ?Sized, M> Drop for MutexGuard<'a, T, M>
where
    M: MutexInnerImpl,
{
    fn drop(&mut self) {
        self.__mutex.give();
    }
}

pub trait MutexInnerImpl
where
    Self: Sized,
{
    fn create() -> Result<Self, FreeRtosError>;
    fn take<D: DurationTicks>(&self, max_wait: D) -> Result<(), FreeRtosError>;
    fn give(&self);

    /// # Safety
    ///
    /// The type of `handle` (normal or recursive mutex) must match the type
    /// of instance being created ([`MutexNormal`] or [`MutexRecursive`] respectively).
    unsafe fn from_raw_handle(handle: FreeRtosSemaphoreHandle) -> Self;
    fn raw_handle(&self) -> FreeRtosSemaphoreHandle;
}

pub struct MutexNormal(FreeRtosSemaphoreHandle);

impl MutexInnerImpl for MutexNormal {
    fn create() -> Result<Self, FreeRtosError> {
        let m = unsafe { freertos_rs_create_mutex() };
        if m == 0 as *const _ {
            return Err(FreeRtosError::OutOfMemory);
        }
        Ok(MutexNormal(m))
    }

    fn take<D: DurationTicks>(&self, max_wait: D) -> Result<(), FreeRtosError> {
        let res = unsafe { freertos_rs_take_semaphore(self.0, max_wait.to_ticks()) };

        if res != 0 {
            return Err(FreeRtosError::MutexTimeout);
        }

        Ok(())
    }

    fn give(&self) {
        unsafe {
            freertos_rs_give_semaphore(self.0);
        }
    }

    #[inline]
    unsafe fn from_raw_handle(handle: FreeRtosSemaphoreHandle) -> Self {
        Self(handle)
    }

    #[inline]
    fn raw_handle(&self) -> FreeRtosSemaphoreHandle {
        self.0
    }
}

impl Drop for MutexNormal {
    fn drop(&mut self) {
        unsafe { freertos_rs_delete_semaphore(self.0) }
    }
}

impl fmt::Debug for MutexNormal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub struct MutexRecursive(FreeRtosSemaphoreHandle);

impl MutexInnerImpl for MutexRecursive {
    fn create() -> Result<Self, FreeRtosError> {
        let m = unsafe { freertos_rs_create_recursive_mutex() };
        if m == 0 as *const _ {
            return Err(FreeRtosError::OutOfMemory);
        }
        Ok(MutexRecursive(m))
    }

    fn take<D: DurationTicks>(&self, max_wait: D) -> Result<(), FreeRtosError> {
        let res = unsafe { freertos_rs_take_recursive_semaphore(self.0, max_wait.to_ticks()) };

        if res != 0 {
            return Err(FreeRtosError::MutexTimeout);
        }

        Ok(())
    }

    fn give(&self) {
        unsafe {
            freertos_rs_give_recursive_semaphore(self.0);
        }
    }

    #[inline]
    unsafe fn from_raw_handle(handle: FreeRtosSemaphoreHandle) -> Self {
        Self(handle)
    }

    #[inline]
    fn raw_handle(&self) -> FreeRtosSemaphoreHandle {
        self.0
    }
}

impl Drop for MutexRecursive {
    fn drop(&mut self) {
        unsafe { freertos_rs_delete_semaphore(self.0) }
    }
}

impl fmt::Debug for MutexRecursive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
