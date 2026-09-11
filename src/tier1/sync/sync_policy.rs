use crate::tier1::sync::atomic_repr::AtomicRepr;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
use core::{
    ops::Add,
    sync::atomic::{AtomicU32, Ordering},
};
#[cfg(not(feature = "std"))]
use critical_section::Mutex;
use num_traits::Zero;
#[cfg(feature = "std")]
use std::sync::Mutex;

pub trait SyncPolicy<T> {
    type Storage;

    const INIT: Self::Storage;

    fn set(s: &Self::Storage, value: T);
    fn get(s: &Self::Storage) -> T;
}

pub trait RMWPolicy<T> {
    type Storage;

    const INIT: Self::Storage;

    fn stake(s: &Self::Storage, delta: T);
    fn claim(s: &Self::Storage) -> T;
}

pub struct AtomicPolicy;

impl<T> SyncPolicy<T> for AtomicPolicy
where
    T: AtomicRepr + Add<T, Output = T> + Clone + Zero,
{
    type Storage = AtomicU32;

    const INIT: Self::Storage = AtomicU32::new(0);

    fn set(s: &Self::Storage, value: T) {
        s.store(value.to_bits(), Ordering::Release);
    }

    fn get(s: &Self::Storage) -> T {
        T::from_bits(s.load(Ordering::Acquire))
    }
}

#[cfg(target_has_atomic = "32")]
impl<T> RMWPolicy<T> for AtomicPolicy
where
    T: AtomicRepr + Add<T, Output = T> + Clone + Zero,
{
    type Storage = AtomicU32;

    const INIT: Self::Storage = AtomicU32::new(0);

    fn stake(s: &Self::Storage, delta: T) {
        let _ = s.fetch_update(Ordering::Release, Ordering::Acquire, move |bits| {
            Some((T::from_bits(bits) + delta.clone()).to_bits())
        });
    }

    fn claim(s: &Self::Storage) -> T {
        let bits = s
            .fetch_update(Ordering::Release, Ordering::Acquire, move |_| {
                Some((T::zero()).to_bits())
            })
            .unwrap();
        T::from_bits(bits)
    }
}

pub struct CriticalSectionPolicy;

impl<T> SyncPolicy<T> for CriticalSectionPolicy
where
    T: Clone + Default + Add<T, Output = T>,
{
    #[cfg(feature = "std")]
    type Storage = Mutex<Option<T>>;
    #[cfg(not(feature = "std"))]
    type Storage = Mutex<RefCell<Option<T>>>;

    #[cfg(feature = "std")]
    const INIT: Self::Storage = Mutex::new(None);
    #[cfg(not(feature = "std"))]
    const INIT: Self::Storage = Mutex::new(RefCell::new(None));

    fn set(s: &Self::Storage, value: T) {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let mut lock = s.borrow(cs).borrow_mut();
                *lock = Some(value);
            });
        }
        #[cfg(feature = "std")]
        {
            let mut lock = s.lock().unwrap();
            *lock = Some(value);
        }
    }

    fn get(s: &Self::Storage) -> T {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let lock = s.borrow(cs).borrow();
                lock.clone().unwrap_or_default()
            })
        }
        #[cfg(feature = "std")]
        {
            let lock = s.lock().unwrap();
            lock.clone().unwrap_or_default()
        }
    }
}

impl<T> RMWPolicy<T> for CriticalSectionPolicy
where
    T: Clone + Default + Add<T, Output = T>,
{
    #[cfg(feature = "std")]
    type Storage = Mutex<Option<T>>;
    #[cfg(not(feature = "std"))]
    type Storage = Mutex<RefCell<Option<T>>>;

    #[cfg(feature = "std")]
    const INIT: Self::Storage = Mutex::new(None);
    #[cfg(not(feature = "std"))]
    const INIT: Self::Storage = Mutex::new(RefCell::new(None));

    fn stake(s: &Self::Storage, delta: T) {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let mut lock = s.borrow(cs).borrow_mut();
                let current = lock.take().unwrap_or_default();
                *lock = Some(current + delta);
            });
        }
        #[cfg(feature = "std")]
        {
            let mut lock = s.lock().unwrap();
            let current = lock.take().unwrap_or_default();
            *lock = Some(current + delta);
        }
    }

    fn claim(s: &Self::Storage) -> T {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let binding = s.borrow(cs);
                let mut lock = binding.borrow_mut();
                lock.take().unwrap_or_default()
            })
        }
        #[cfg(feature = "std")]
        {
            let mut lock = s.lock().unwrap();
            lock.take().unwrap_or_default()
        }
    }
}
