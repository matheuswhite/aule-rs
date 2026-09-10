use crate::{
    block::Block, simulation::SimulationState, tier1::sync::conveyor_storage::ConveyorQueue,
};
use bbqueue::{
    BBQueue,
    prod_cons::stream::{StreamConsumer, StreamProducer},
    traits::coordination::{ReadGrantError, WriteGrantError},
};
use core::{fmt::Debug, marker::PhantomData};
#[cfg(not(feature = "std"))]
use core::cell::Cell;
#[cfg(not(feature = "std"))]
use critical_section::Mutex;
#[cfg(feature = "std")]
use std::sync::Mutex;

pub struct Conveyor<T, const N: usize>
where
    T: 'static,
{
    queue: ConveyorQueue<T, N>,
    #[cfg(feature = "std")]
    has_feeder: Mutex<bool>,
    #[cfg(not(feature = "std"))]
    has_feeder: Mutex<Cell<bool>>,
    #[cfg(feature = "std")]
    has_picker: Mutex<bool>,
    #[cfg(not(feature = "std"))]
    has_picker: Mutex<Cell<bool>>,
}

impl<T, const N: usize> Conveyor<T, N> {
    pub const fn new() -> Self {
        Self {
            queue: BBQueue::new(),

            #[cfg(feature = "std")]
            has_feeder: Mutex::new(true),
            #[cfg(not(feature = "std"))]
            has_feeder: Mutex::new(Cell::new(true)),

            #[cfg(feature = "std")]
            has_picker: Mutex::new(true),
            #[cfg(not(feature = "std"))]
            has_picker: Mutex::new(Cell::new(true)),
        }
    }

    pub fn feeder(&self) -> Option<Feeder<'_, T, N>> {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let has_feeder = self.has_feeder.borrow(cs);
                if !has_feeder.get() {
                    return None;
                }
                has_feeder.set(false);

                Some(Feeder {
                    producer: self.queue.stream_producer(),
                    _marker: PhantomData,
                })
            })
        }
        #[cfg(feature = "std")]
        {
            let mut has_feeder = self.has_feeder.lock().unwrap();
            if !*has_feeder {
                return None;
            }
            *has_feeder = false;

            Some(Feeder {
                producer: self.queue.stream_producer(),
                _marker: PhantomData,
            })
        }
    }

    pub fn blocking_feeder(&self) -> Option<BlockingFeeder<'_, T, N>> {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let has_feeder = self.has_feeder.borrow(cs);
                if !has_feeder.get() {
                    return None;
                }
                has_feeder.set(false);

                Some(BlockingFeeder {
                    producer: self.queue.stream_producer(),
                    _marker: PhantomData,
                })
            })
        }
        #[cfg(feature = "std")]
        {
            let mut has_feeder = self.has_feeder.lock().unwrap();
            if !*has_feeder {
                return None;
            }
            *has_feeder = false;

            Some(BlockingFeeder {
                producer: self.queue.stream_producer(),
                _marker: PhantomData,
            })
        }
    }

    pub fn picker(&self) -> Option<Picker<'_, T, N>> {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let has_picker = self.has_picker.borrow(cs);
                if !has_picker.get() {
                    return None;
                }
                has_picker.set(false);

                Some(Picker {
                    consumer: self.queue.stream_consumer(),
                    _marker: PhantomData,
                })
            })
        }
        #[cfg(feature = "std")]
        {
            let mut has_picker = self.has_picker.lock().unwrap();
            if !*has_picker {
                return None;
            }
            *has_picker = false;

            Some(Picker {
                consumer: self.queue.stream_consumer(),
                _marker: PhantomData,
            })
        }
    }

    pub fn blocking_picker(&self) -> Option<BlockingPicker<'_, T, N>> {
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|cs| {
                let has_picker = self.has_picker.borrow(cs);
                if !has_picker.get() {
                    return None;
                }
                has_picker.set(false);

                Some(BlockingPicker {
                    consumer: self.queue.stream_consumer(),
                    _marker: PhantomData,
                })
            })
        }
        #[cfg(feature = "std")]
        {
            let mut has_picker = self.has_picker.lock().unwrap();
            if !*has_picker {
                return None;
            }
            *has_picker = false;

            Some(BlockingPicker {
                consumer: self.queue.stream_consumer(),
                _marker: PhantomData,
            })
        }
    }
}

pub struct Feeder<'a, T, const N: usize> {
    producer: StreamProducer<&'a ConveyorQueue<T, N>>,
    _marker: PhantomData<T>,
}

impl<'a, T, const N: usize> Block for Feeder<'a, T, N>
where
    T: Debug,
{
    type Input = T;
    type Output = Result<(), WriteGrantError>;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let mut wgr = self.producer.grant_exact(size_of::<T>())?;

        // SAFETY: `wgr` is a write grant of exactly `size_of::<T>()` bytes obtained from
        // `grant_exact`, so the destination is valid for writes of that many bytes and, per
        // the `Coord` contract, does not overlap any range the consumer can observe. The
        // source is `&input`: a live, aligned, fully initialized `T` owned by this function.
        // `copy_nonoverlapping` is used rather than building a `&[u8]` precisely so that no
        // reference is ever formed over `T`'s padding bytes, which may be uninitialised.
        // Source and destination cannot overlap: `input` is a local, the grant points into
        // the queue's backing storage.
        //
        // Ownership transfer: the copy leaves two bitwise images of the same value alive.
        // `mem::forget(input)` suppresses the local's destructor, handing ownership to the
        // queue and leaving exactly one live image — the bytes in the buffer — which `Picker`
        // reclaims with `ptr::read`. Letting `input` drop here instead would double-drop the
        // value. On the error path no copy has happened, so `input` is dropped normally and
        // no ownership has been transferred.
        //
        // Alignment for the consumer's `ptr::read` holds by construction: the backing storage
        // is `[T; N]`, so its base is aligned to `align_of::<T>()`; every grant on this queue
        // is exactly `size_of::<T>()` bytes; and `size_of::<T>()` is always a multiple of
        // `align_of::<T>()`. Every grant offset is therefore `T`-aligned. This invariant is
        // void if the queue is ever granted a size other than `size_of::<T>()`, which is why
        // the inner queue is never exposed.
        unsafe {
            core::ptr::copy_nonoverlapping(
                &input as *const T as *const u8,
                wgr.as_mut_ptr(),
                size_of::<T>(),
            );
        }
        core::mem::forget(input);

        wgr.commit(size_of::<T>());

        Ok(())
    }
}

pub struct BlockingFeeder<'a, T, const N: usize> {
    producer: StreamProducer<&'a ConveyorQueue<T, N>>,
    _marker: PhantomData<T>,
}

impl<'a, T, const N: usize> Block for BlockingFeeder<'a, T, N>
where
    T: Debug,
{
    type Input = T;
    type Output = ();

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let mut wgr = 'write_space: loop {
            match self.producer.grant_exact(size_of::<T>()) {
                Ok(wgr) => break 'write_space wgr,
                Err(_) => continue,
            }
        };

        // SAFETY: `wgr` is a write grant of exactly `size_of::<T>()` bytes obtained from
        // `grant_exact`, so the destination is valid for writes of that many bytes and, per
        // the `Coord` contract, does not overlap any range the consumer can observe. The
        // source is `&input`: a live, aligned, fully initialized `T` owned by this function.
        // `copy_nonoverlapping` is used rather than building a `&[u8]` precisely so that no
        // reference is ever formed over `T`'s padding bytes, which may be uninitialised.
        // Source and destination cannot overlap: `input` is a local, the grant points into
        // the queue's backing storage.
        //
        // Ownership transfer: the copy leaves two bitwise images of the same value alive.
        // `mem::forget(input)` suppresses the local's destructor, handing ownership to the
        // queue and leaving exactly one live image — the bytes in the buffer — which `Picker`
        // reclaims with `ptr::read`. Letting `input` drop here instead would double-drop the
        // value. On the error path no copy has happened, so `input` is dropped normally and
        // no ownership has been transferred.
        //
        // Alignment for the consumer's `ptr::read` holds by construction: the backing storage
        // is `[T; N]`, so its base is aligned to `align_of::<T>()`; every grant on this queue
        // is exactly `size_of::<T>()` bytes; and `size_of::<T>()` is always a multiple of
        // `align_of::<T>()`. Every grant offset is therefore `T`-aligned. This invariant is
        // void if the queue is ever granted a size other than `size_of::<T>()`, which is why
        // the inner queue is never exposed.
        unsafe {
            core::ptr::copy_nonoverlapping(
                &input as *const T as *const u8,
                wgr.as_mut_ptr(),
                size_of::<T>(),
            );
        }
        core::mem::forget(input);

        wgr.commit(size_of::<T>());
    }
}

pub struct Picker<'a, T, const N: usize> {
    consumer: StreamConsumer<&'a ConveyorQueue<T, N>>,
    _marker: PhantomData<T>,
}

impl<'a, T, const N: usize> Block for Picker<'a, T, N> {
    type Input = ();
    type Output = Result<T, ReadGrantError>;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let rgr = self.consumer.read()?;

        // SAFETY: The read grant begins at a byte range previously committed by a `Feeder` as
        // exactly one bitwise image of a `T`, so the bytes form a valid, initialized `T`. The
        // pointer is `T`-aligned by the argument documented on the producer side: the storage
        // base is `[T; N]`-aligned and every grant is exactly `size_of::<T>()` bytes.
        //
        // Ownership: the producer already relinquished its claim via `mem::forget`, so this
        // `ptr::read` does not duplicate a live value — it moves the single remaining image
        // out of the buffer. `release(size_of::<T>())` then advances the read pointer past
        // exactly those bytes, so the same `T` can never be yielded twice, and the `Coord`
        // contract guarantees no producer can overwrite the range while the grant is held.
        //
        // Grant length: `read()` can only expose committed bytes; every commit on this queue
        // is exactly `size_of::<T>()` bytes over a buffer sized `size_of::<T>() * N`, so any
        // available grant covers at least one whole `T`.
        let output = unsafe { core::ptr::read(rgr.as_ptr() as *const T) };

        rgr.release(size_of::<T>());

        Ok(output)
    }
}

pub struct BlockingPicker<'a, T, const N: usize> {
    consumer: StreamConsumer<&'a ConveyorQueue<T, N>>,
    _marker: PhantomData<T>,
}

impl<'a, T, const N: usize> Block for BlockingPicker<'a, T, N> {
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let rgr = 'read_loop: loop {
            match self.consumer.read() {
                Ok(rgr) => break 'read_loop rgr,
                Err(_) => continue,
            }
        };

        // SAFETY: The read grant begins at a byte range previously committed by a `Feeder` as
        // exactly one bitwise image of a `T`, so the bytes form a valid, initialized `T`. The
        // pointer is `T`-aligned by the argument documented on the producer side: the storage
        // base is `[T; N]`-aligned and every grant is exactly `size_of::<T>()` bytes.
        //
        // Ownership: the producer already relinquished its claim via `mem::forget`, so this
        // `ptr::read` does not duplicate a live value — it moves the single remaining image
        // out of the buffer. `release(size_of::<T>())` then advances the read pointer past
        // exactly those bytes, so the same `T` can never be yielded twice, and the `Coord`
        // contract guarantees no producer can overwrite the range while the grant is held.
        //
        // Grant length: `read()` can only expose committed bytes; every commit on this queue
        // is exactly `size_of::<T>()` bytes over a buffer sized `size_of::<T>() * N`, so any
        // available grant covers at least one whole `T`.
        let output = unsafe { core::ptr::read(rgr.as_ptr() as *const T) };

        rgr.release(size_of::<T>());

        output
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use core::assert_eq;

    use crate::{
        block::Block,
        signal::AsSignal,
        simulation::{EndlessSimulation, Simulation},
        tier1::sync::conveyor::Conveyor,
    };

    #[test]
    fn test_feeder_picker() {
        static CONVEYOR: Conveyor<f32, 5> = Conveyor::new();

        let t1 = std::thread::spawn(|| {
            let Some(mut feeder) = CONVEYOR.feeder() else {
                return;
            };

            for sim_state in Simulation::new(1.0, 5.0) {
                let signal = sim_state.sim_time().as_secs_f32().as_signal(sim_state);
                let _ = signal * feeder.as_block();
            }
        });

        let t2 = std::thread::spawn(|| {
            let mut outputs = Vec::new();
            let Some(mut picker) = CONVEYOR.picker() else {
                return;
            };

            for sim_state in EndlessSimulation::new(1.0) {
                let input = ().as_signal(sim_state);
                let signal = input * picker.as_block();

                if let Ok(value) = signal.value {
                    outputs.push(value);
                }

                if outputs.len() >= 5 {
                    break;
                }
            }

            assert_eq!(outputs, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        });

        let _ = t1.join();
        let _ = t2.join();
    }

    #[test]
    fn test_blocking_feeder_picker() {
        static CONVEYOR: Conveyor<f32, 5> = Conveyor::new();

        let t1 = std::thread::spawn(|| {
            let Some(mut feeder) = CONVEYOR.blocking_feeder() else {
                return;
            };

            for sim_state in Simulation::new(1.0, 5.0) {
                let signal = sim_state.sim_time().as_secs_f32().as_signal(sim_state);
                let _ = signal * feeder.as_block();
            }
        });

        let t2 = std::thread::spawn(|| {
            let mut outputs = Vec::new();
            let Some(mut picker) = CONVEYOR.picker() else {
                return;
            };

            for sim_state in EndlessSimulation::new(1.0) {
                let input = ().as_signal(sim_state);
                let signal = input * picker.as_block();

                if let Ok(value) = signal.value {
                    outputs.push(value);
                }

                if outputs.len() >= 5 {
                    break;
                }
            }

            assert_eq!(outputs, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        });

        let _ = t1.join();
        let _ = t2.join();
    }

    #[test]
    fn test_feeder_blocking_picker() {
        static CONVEYOR: Conveyor<f32, 5> = Conveyor::new();

        let t1 = std::thread::spawn(|| {
            let Some(mut feeder) = CONVEYOR.feeder() else {
                return;
            };

            for sim_state in Simulation::new(1.0, 5.0) {
                let signal = sim_state.sim_time().as_secs_f32().as_signal(sim_state);
                let _ = signal * feeder.as_block();
            }
        });

        let t2 = std::thread::spawn(|| {
            let mut outputs = Vec::new();
            let Some(mut picker) = CONVEYOR.blocking_picker() else {
                return;
            };

            for sim_state in Simulation::new(1.0, 5.0) {
                let input = ().as_signal(sim_state);
                let signal = input * picker.as_block();
                outputs.push(signal.value);
            }

            assert_eq!(outputs, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        });

        let _ = t1.join();
        let _ = t2.join();
    }

    #[test]
    fn test_blocking_feeder_blocking_picker() {
        static CONVEYOR: Conveyor<f32, 5> = Conveyor::new();

        let t1 = std::thread::spawn(|| {
            let Some(mut feeder) = CONVEYOR.blocking_feeder() else {
                return;
            };

            for sim_state in Simulation::new(1.0, 5.0) {
                let signal = sim_state.sim_time().as_secs_f32().as_signal(sim_state);
                let _ = signal * feeder.as_block();
            }
        });

        let t2 = std::thread::spawn(|| {
            let mut outputs = Vec::new();
            let Some(mut picker) = CONVEYOR.blocking_picker() else {
                return;
            };

            for sim_state in Simulation::new(1.0, 5.0) {
                let input = ().as_signal(sim_state);
                let signal = input * picker.as_block();
                outputs.push(signal.value);
            }

            assert_eq!(outputs, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        });

        let _ = t1.join();
        let _ = t2.join();
    }
}
