use core::{cell::UnsafeCell, mem::MaybeUninit, ptr::NonNull};

#[cfg(target_has_atomic = "ptr")]
use bbqueue::traits::coordination::cas::AtomicCoord;
#[cfg(not(target_has_atomic = "ptr"))]
use bbqueue::traits::coordination::cs::CsCoord;
use bbqueue::{
    BBQueue,
    export::ConstInit,
    traits::{notifier::polling::Polling, storage::Storage},
};

#[repr(transparent)]
pub struct ConveyorStorage<T, const N: usize>(UnsafeCell<MaybeUninit<[T; N]>>);

// SAFETY: `ConveyorStorage` is an opaque byte buffer, not a container of live `T`
// values. Its only access path is `Storage::ptr_len`, which hands the base pointer
// and the length to `BBQueue`; this type never dereferences the `UnsafeCell` itself
// and never observes a `T`. Every decision about which byte range a producer or a
// consumer may touch is made by the `Coord` implementation (`AtomicCoord` via
// compare-exchange, `CsCoord` via a critical section), which guarantees that a write
// grant and a read grant never overlap. Sharing `&ConveyorStorage` across execution
// contexts therefore introduces no unsynchronised access of its own.
//
// This impl deliberately carries no `T: Send` bound, because the `Send`-ness of the
// payload is enforced at the handle level instead: `Feeder` and `Picker` each hold a
// `PhantomData<T>`, so they are `Send` only when `T: Send`, and no value can reach
// another context without a handle going there first. That `PhantomData` is
// load-bearing for this invariant — removing it would let a `!Send` payload such as
// `Rc` cross contexts through the queue.
unsafe impl<T, const N: usize> Sync for ConveyorStorage<T, N> {}

impl<T, const N: usize> ConveyorStorage<T, N> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(MaybeUninit::zeroed()))
    }
}

impl<T, const N: usize> Default for ConveyorStorage<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Storage for ConveyorStorage<T, N> {
    /// # Safety
    ///
    /// The returned pointer and length describe the entire backing buffer
    /// (`size_of::<T>() * N` bytes) and remain valid for as long as `self` lives: the
    /// buffer is inline in `self` and is never moved or reallocated. The caller must be
    /// the sole authority over that region — it may form references only into byte ranges
    /// it has handed out as a grant, and must never alias a write grant with a read
    /// grant. The caller must not assume the bytes are initialised: the buffer starts
    /// zeroed, but holds `MaybeUninit` data as far as `T` is concerned.
    unsafe fn ptr_len(&self) -> (core::ptr::NonNull<u8>, usize) {
        if N == 0 {
            return (NonNull::dangling(), size_of::<T>() * N);
        }

        let ptr: *mut MaybeUninit<[T; N]> = self.0.get();
        let ptr: *mut u8 = ptr.cast();
        // SAFETY: UnsafeCell and MaybeUninit are both repr transparent, cast is
        // sound to get to first byte element
        let nn_ptr = unsafe { NonNull::new_unchecked(ptr) };
        (nn_ptr, size_of::<T>() * N)
    }
}

impl<T, const N: usize> ConstInit for ConveyorStorage<T, N> {
    const INIT: Self = Self::new();
}

#[cfg(target_has_atomic = "ptr")]
pub type ConveyorQueue<T, const N: usize> = BBQueue<ConveyorStorage<T, N>, AtomicCoord, Polling>;
#[cfg(not(target_has_atomic = "ptr"))]
pub type ConveyorQueue<T, const N: usize> = BBQueue<ConveyorStorage<T, N>, CsCoord, Polling>;
