use core::marker::PhantomData;

use crate::{
    block::Block,
    tier1::sync::sync_policy::{AtomicPolicy, RMWPolicy},
};

#[allow(private_bounds)]
pub struct Jackpot<T, P = AtomicPolicy>
where
    P: RMWPolicy<T>,
{
    storage: P::Storage,
    _marker: PhantomData<T>,
}

#[allow(private_bounds)]
impl<T, P> Jackpot<T, P>
where
    P: RMWPolicy<T>,
{
    pub const fn new() -> Self {
        Self {
            storage: P::INIT,
            _marker: PhantomData,
        }
    }

    pub fn as_staker(&'_ self) -> Staker<'_, T, P> {
        Staker { jackpot: self }
    }

    pub fn as_claimer(&'_ self) -> Claimer<'_, T, P> {
        Claimer { jackpot: self }
    }
}

#[allow(private_bounds)]
pub struct Staker<'a, T, P>
where
    P: RMWPolicy<T>,
{
    jackpot: &'a Jackpot<T, P>,
}

impl<'a, T, P> Block for Staker<'a, T, P>
where
    P: RMWPolicy<T>,
{
    type Input = T;
    type Output = ();

    fn block(
        &mut self,
        input: Self::Input,
        _sim_state: crate::prelude::SimulationState,
    ) -> Self::Output {
        P::stake(&self.jackpot.storage, input);
    }
}

#[allow(private_bounds)]
pub struct Claimer<'a, T, P>
where
    P: RMWPolicy<T>,
{
    jackpot: &'a Jackpot<T, P>,
}

impl<'a, T, P> Block for Claimer<'a, T, P>
where
    P: RMWPolicy<T>,
{
    type Input = ();
    type Output = T;

    fn block(
        &mut self,
        _input: Self::Input,
        _sim_state: crate::prelude::SimulationState,
    ) -> Self::Output {
        P::claim(&self.jackpot.storage)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Add;

    #[cfg(feature = "std")]
    use alloc::sync::Arc;
    use num_traits::Zero;

    use crate::prelude::AsSignal;
    use crate::simulation::EndlessSimulation;
    use crate::tier1::sync::atomic_repr::AtomicRepr;
    use crate::tier1::sync::sync_policy::CriticalSectionPolicy;

    #[derive(Clone, Default, Debug, PartialEq)]
    struct ComplexI16 {
        real: i16,
        imag: i16,
    }

    impl ComplexI16 {
        const ZERO: Self = Self { real: 0, imag: 0 };

        pub fn new(real: i16, imag: i16) -> Self {
            Self { real, imag }
        }
    }

    impl AtomicRepr for ComplexI16 {
        fn to_bits(self) -> u32 {
            ((self.real as u32) << 16) | (self.imag as u16 as u32)
        }

        fn from_bits(bits: u32) -> Self {
            let real = (bits >> 16) as i16;
            let imag = bits as u16 as i16;
            Self { real, imag }
        }
    }

    impl Zero for ComplexI16 {
        fn zero() -> Self {
            Self::ZERO
        }

        fn is_zero(&self) -> bool {
            self.real == 0 && self.imag == 0
        }
    }

    impl Add for ComplexI16 {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }

    #[test]
    fn test_jackpot_complex_i16() {
        use super::*;

        static JACKPOT: Jackpot<ComplexI16> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = ComplexI16::new(10, 0).as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = ComplexI16::new(0, 20).as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, ComplexI16::new(10, 20));
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, ComplexI16::ZERO);
    }

    #[test]
    fn test_jackpot_i8() {
        use super::*;

        static JACKPOT: Jackpot<i8> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_u8() {
        use super::*;

        static JACKPOT: Jackpot<u8> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_i16() {
        use super::*;

        static JACKPOT: Jackpot<i16> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_u16() {
        use super::*;

        static JACKPOT: Jackpot<u16> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_i32() {
        use super::*;

        static JACKPOT: Jackpot<i32> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_u32() {
        use super::*;

        static JACKPOT: Jackpot<u32> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[derive(Clone, Debug, PartialEq)]
    struct LineBuffer {
        buffer: Arc<[u8; 1024]>,
        counter: usize,
    }

    impl LineBuffer {
        fn new() -> Self {
            Self {
                buffer: Arc::new([0; 1024]),
                counter: 0,
            }
        }

        fn with_counter(mut self, counter: usize) -> Self {
            self.counter = counter;
            self
        }
    }

    impl Default for LineBuffer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Add for LineBuffer {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let mut obj = self;
            obj.counter += rhs.counter;
            obj
        }
    }

    #[test]
    fn test_jackpot_big_cs() {
        use super::*;

        static JACKPOT: Jackpot<LineBuffer, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = LineBuffer::new()
            .with_counter(10)
            .as_signal(sim.next().unwrap())
            * staker.as_block();
        let _ = LineBuffer::new()
            .with_counter(20)
            .as_signal(sim.next().unwrap())
            * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, LineBuffer::new().with_counter(30));
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, LineBuffer::new().with_counter(0));
    }

    #[test]
    fn test_jackpot_complex_i16_cs() {
        use super::*;

        static JACKPOT: Jackpot<ComplexI16, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = ComplexI16::new(10, 0).as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = ComplexI16::new(0, 20).as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, ComplexI16::new(10, 20));
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, ComplexI16::ZERO);
    }

    #[test]
    fn test_jackpot_i8_cs() {
        use super::*;

        static JACKPOT: Jackpot<i8, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_u8_cs() {
        use super::*;

        static JACKPOT: Jackpot<u8, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_i16_cs() {
        use super::*;

        static JACKPOT: Jackpot<i16, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_u16_cs() {
        use super::*;

        static JACKPOT: Jackpot<u16, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_i32_cs() {
        use super::*;

        static JACKPOT: Jackpot<i32, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }

    #[test]
    fn test_jackpot_u32_cs() {
        use super::*;

        static JACKPOT: Jackpot<u32, CriticalSectionPolicy> = Jackpot::new();

        let mut staker = JACKPOT.as_staker();
        let mut claimer = JACKPOT.as_claimer();
        let mut sim = EndlessSimulation::new(1.0);

        let _ = 10.as_signal(sim.next().unwrap()) * staker.as_block();
        let _ = 20.as_signal(sim.next().unwrap()) * staker.as_block();

        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 30);
        let claimed = sim.next().unwrap() * claimer.as_block();
        assert_eq!(claimed.value, 0);
    }
}
