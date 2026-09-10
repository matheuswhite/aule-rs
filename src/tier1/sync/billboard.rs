use crate::block::Block;
use crate::prelude::SimulationState;
use core::sync::atomic::Ordering;
use core::{
    cell::{Cell, UnsafeCell},
    sync::atomic::AtomicU8,
};
#[cfg(not(feature = "std"))]
use critical_section::Mutex;
#[cfg(feature = "std")]
use std::sync::Mutex;

pub struct Billboard<T> {
    buffer: [UnsafeCell<Option<T>>; 3],
    shared: AtomicU8,

    #[cfg(feature = "std")]
    has_poster: Mutex<bool>,
    #[cfg(not(feature = "std"))]
    has_poster: Mutex<Cell<bool>>,
    #[cfg(feature = "std")]
    has_viewer: Mutex<bool>,
    #[cfg(not(feature = "std"))]
    has_viewer: Mutex<Cell<bool>>,
}

impl<T> Billboard<T>
where
    T: 'static,
{
    pub const fn new() -> Self {
        Self {
            buffer: [
                UnsafeCell::new(None),
                UnsafeCell::new(None),
                UnsafeCell::new(None),
            ],
            shared: AtomicU8::new(1),
            #[cfg(feature = "std")]
            has_viewer: Mutex::new(true),
            #[cfg(not(feature = "std"))]
            has_viewer: Mutex::new(Cell::new(true)),
            #[cfg(feature = "std")]
            has_poster: Mutex::new(true),
            #[cfg(not(feature = "std"))]
            has_poster: Mutex::new(Cell::new(true)),
        }
    }

    pub fn poster(&'_ self) -> Option<Poster<'_, T>> {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let has_poster = self.has_poster.borrow(cs);
                if !has_poster.get() {
                    return None;
                }
                has_poster.set(false);

                Some(Poster {
                    private: 0,
                    billboard: self,
                })
            })
        }
        #[cfg(feature = "std")]
        {
            let mut has_poster = self.has_poster.lock().unwrap();
            if !*has_poster {
                return None;
            }
            *has_poster = false;

            Some(Poster {
                private: 0,
                billboard: self,
            })
        }
    }

    pub fn viewer(&'_ self) -> Option<Viewer<'_, T>> {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let has_viewer = self.has_viewer.borrow(cs);
                if !has_viewer.get() {
                    return None;
                }
                has_viewer.set(false);

                Some(Viewer {
                    private: 2,
                    billboard: self,
                    last_data: None,
                })
            })
        }
        #[cfg(feature = "std")]
        {
            let mut has_viewer = self.has_viewer.lock().unwrap();
            if !*has_viewer {
                return None;
            }
            *has_viewer = false;

            Some(Viewer {
                private: 2,
                billboard: self,
                last_data: None,
            })
        }
    }
}

// SAFETY: The three slot indices in play — the poster's private index, the low seven
// bits of `shared`, and the viewer's private index — are always a permutation of
// {0, 1, 2}. At construction, they are 0, 1 and 2. The only mutation of `shared` is
// `swap`, which atomically hands the caller's own index over and takes the one that
// was there, and both sides mask the result back into the low seven bits, so an index
// is never duplicated and never lost. Each side therefore dereferences a slot that no
// other side is able to name. Both sides take a `&mut` into their slot, so what the
// permutation has to discharge is mutual exclusivity, not merely the absence of a
// writer during a read — and it does, because holding the index is what grants the
// right to dereference.
//
// The permutation argument assumes exactly one poster and one viewer. That premise is
// what `has_poster` and `has_viewer` enforce: each handle is handed out once, under a
// mutex on `std` and a critical section otherwise, so the claim is decided even when
// two contexts race for it.
//
// Ordering rests entirely on the `swap`, which is the only point where the two sides
// meet. On the publishing side the payload store precedes a `swap(AcqRel)`, whose
// release half makes the store visible to whoever later acquires that index; on the
// observing side a `swap(AcqRel)` precedes the payload read, and its acquire half
// orders the read after that store. The chain is complete without a fence.
//
// `T: Send` is required and sufficient. A value is constructed in the publishing
// context and moved out of the slot in the observing one, so it changes context and
// must be `Send`. No `&T` ever crosses the boundary — the viewer takes ownership of
// the value and hands out clones of its own copy — so `T: Sync` is not needed.
unsafe impl<T: Send> Sync for Billboard<T> {}

pub struct Poster<'a, T> {
    private: u8,
    billboard: &'a Billboard<T>,
}

impl<'a, T> Block for Poster<'a, T> {
    type Input = T;
    type Output = ();

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let data = &self.billboard.buffer[self.private as usize];

        // SAFETY: `self.private` is one of {0, 1, 2} by the permutation invariant documented on
        // the `Sync` impl, so the index is in bounds, and the poster is the sole holder of that
        // index — no other reference to this slot can exist, which makes this `&mut` unique.
        //
        // `input` is *moved* into the slot, never copied, so no value is duplicated here. The
        // assignment drops whatever the slot held: `None` if the viewer consumed that edition,
        // or `Some(old)` if the poster reclaimed a slot the viewer never read. Either way the
        // poster holds that index exclusively at this moment, so the drop runs on a value it
        // alone owns.
        //
        // The store must precede publication, and it does: the `swap(AcqRel)` below carries the
        // release half that makes these bytes visible to a viewer acquiring this index.
        unsafe {
            let data: &mut Option<T> = &mut *data.get();
            *data = Some(input);
        }

        self.private = self
            .billboard
            .shared
            .swap(self.private | 0b1000_0000, Ordering::AcqRel)
            & 0b0111_1111;
    }
}

pub struct Viewer<'a, T> {
    private: u8,
    billboard: &'a Billboard<T>,
    last_data: Option<T>,
}

impl<'a, T> Block for Viewer<'a, T>
where
    T: Clone,
{
    type Input = ();
    type Output = Option<T>;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let shared = self.billboard.shared.load(Ordering::Acquire);
        let has_data = shared & 0b1000_0000 != 0;

        if has_data {
            self.private = self.billboard.shared.swap(self.private, Ordering::AcqRel) & 0b0111_1111;

            // SAFETY: `self.private` was just assigned from the masked result of the swap above, so
            // it is one of {0, 1, 2} and in bounds. That same swap transferred exclusive holding of
            // this index to the viewer in the one atomic step that gave up its previous index, so by
            // the permutation invariant no other side can name this slot, which makes this `&mut`
            // unique.
            //
            // The acquire half of that same swap orders this read after the poster's publishing
            // store, so what is taken is one complete edition rather than a mix of two.
            //
            // `take` *moves* the value out and leaves `None` behind, so ownership passes from the
            // slot to `last_data` and exactly one owner exists at every instant. That is why this
            // block needs no `Copy` bound and why the double-drop class of defect is inexpressible
            // here: nothing is ever duplicated. `take` itself cannot panic, and the `Clone` this
            // block once performed now happens outside it, on `last_data`, which the viewer owns
            // exclusively — so no user code runs while this `&mut` is live.
            unsafe {
                let data: &mut Option<T> = &mut *self.billboard.buffer[self.private as usize].get();
                self.last_data = (*data).take();
            }
        }

        self.last_data.clone()
    }
}
