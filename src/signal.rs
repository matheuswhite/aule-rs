use crate::block::Block;
use crate::simulation::SimulationState;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Signal<T> {
    pub value: T,
    pub sim_state: SimulationState,
}

impl<T> Copy for Signal<T> where T: Copy {}

impl<T> Signal<Signal<T>> {
    pub fn flatten(self) -> Signal<T> {
        Signal {
            value: self.value.value,
            sim_state: self.value.sim_state.merge(self.sim_state),
        }
    }
}

impl<T> From<SimulationState> for Signal<T>
where
    T: Default,
{
    fn from(sim_state: SimulationState) -> Self {
        Signal {
            value: T::default(),
            sim_state,
        }
    }
}

impl<T> From<(T, SimulationState)> for Signal<T> {
    fn from((value, sim_state): (T, SimulationState)) -> Self {
        Signal { value, sim_state }
    }
}

impl<T> Neg for Signal<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            sim_state: self.sim_state,
        }
    }
}

impl<T> Sub for Signal<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            sim_state: self.sim_state.merge(rhs.sim_state),
        }
    }
}

impl<T> Sub<T> for Signal<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value - rhs,
            sim_state: self.sim_state,
        }
    }
}

impl<T> Sub<Option<T>> for Signal<T>
where
    T: Sub<Output = T> + Default,
{
    type Output = Self;

    fn sub(self, rhs: Option<T>) -> Self::Output {
        Signal {
            value: self.value - rhs.unwrap_or_default(),
            sim_state: self.sim_state,
        }
    }
}

impl<T> Sub<Option<Signal<T>>> for Signal<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Option<Signal<T>>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl<T> Add for Signal<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            sim_state: self.sim_state.merge(rhs.sim_state),
        }
    }
}

impl<T> Add<T> for Signal<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value + rhs,
            sim_state: self.sim_state,
        }
    }
}

impl<T> Add<Option<Signal<T>>> for Signal<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Option<Signal<T>>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl<T> Div for Signal<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            sim_state: self.sim_state.merge(rhs.sim_state),
        }
    }
}

impl<T> Div<T> for Signal<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value / rhs,
            sim_state: self.sim_state,
        }
    }
}

impl<T> Mul for Signal<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            sim_state: self.sim_state.merge(rhs.sim_state),
        }
    }
}

impl<T> Mul<T> for Signal<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value * rhs,
            sim_state: self.sim_state,
        }
    }
}

impl<I, O> Mul<&mut dyn Block<Input = I, Output = O>> for Signal<I> {
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = I, Output = O>) -> Self::Output {
        block.output(self)
    }
}

impl<I, O> Mul<&mut dyn Block<Input = [I; 1], Output = [O; 1]>> for Signal<I>
where
    O: Clone,
{
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = [I; 1], Output = [O; 1]>) -> Self::Output {
        let output = block.block([self.value], self.sim_state);
        Signal {
            value: output[0].clone(),
            sim_state: self.sim_state,
        }
    }
}

pub trait Pack<P> {
    fn pack(self) -> Signal<P>;
}

impl<T, const N: usize> Pack<[T; N]> for [Signal<T>; N]
where
    T: Copy,
{
    fn pack(self) -> Signal<[T; N]> {
        let values = self.map(|signal| signal.value);
        let deltas = self.map(|signal| signal.sim_state);
        let merged_delta = deltas
            .into_iter()
            .fold(deltas[0], |acc, sim_state| acc.merge(sim_state));

        Signal {
            value: values,
            sim_state: merged_delta,
        }
    }
}

impl<T> Pack<(T, T)> for (Signal<T>, Signal<T>)
where
    T: Copy,
{
    fn pack(self) -> Signal<(T, T)> {
        let (signal_a, signal_b) = self;
        let packed_value = (signal_a.value, signal_b.value);
        let packed_delta = signal_a.sim_state.merge(signal_b.sim_state);

        Signal {
            value: packed_value,
            sim_state: packed_delta,
        }
    }
}

impl<T> Pack<(T, T, T)> for (Signal<T>, Signal<T>, Signal<T>)
where
    T: Copy,
{
    fn pack(self) -> Signal<(T, T, T)> {
        let (signal_a, signal_b, signal_c) = self;
        let packed_value = (signal_a.value, signal_b.value, signal_c.value);
        let packed_delta = signal_a
            .sim_state
            .merge(signal_b.sim_state)
            .merge(signal_c.sim_state);

        Signal {
            value: packed_value,
            sim_state: packed_delta,
        }
    }
}

pub trait Unpack<U> {
    fn unpack(self) -> U;
}

impl<T> Unpack<(Signal<T>, Signal<T>)> for Signal<(T, T)>
where
    T: Copy,
{
    fn unpack(self) -> (Signal<T>, Signal<T>) {
        let Signal { value, sim_state } = self;
        let (v1, v2) = value;
        (
            Signal {
                value: v1,
                sim_state,
            },
            Signal {
                value: v2,
                sim_state,
            },
        )
    }
}

impl<T> Unpack<(Signal<T>, Signal<T>, Signal<T>)> for Signal<(T, T, T)>
where
    T: Copy,
{
    fn unpack(self) -> (Signal<T>, Signal<T>, Signal<T>) {
        let Signal { value, sim_state } = self;
        let (v1, v2, v3) = value;
        (
            Signal {
                value: v1,
                sim_state,
            },
            Signal {
                value: v2,
                sim_state,
            },
            Signal {
                value: v3,
                sim_state,
            },
        )
    }
}

impl<T, const N: usize> Unpack<[Signal<T>; N]> for Signal<[T; N]>
where
    T: Copy,
{
    fn unpack(self) -> [Signal<T>; N] {
        let Signal { value, sim_state } = self;
        value.map(|v| Signal {
            value: v,
            sim_state,
        })
    }
}

impl<T> Unpack<Option<Signal<T>>> for Signal<Option<T>> {
    fn unpack(self) -> Option<Signal<T>> {
        let Signal { value, sim_state } = self;
        value.map(|v| Signal {
            value: v,
            sim_state,
        })
    }
}

pub trait AsSignal {
    #[allow(clippy::wrong_self_convention)]
    fn as_signal(self, sim_state: SimulationState) -> Signal<Self>
    where
        Self: Sized,
    {
        Signal {
            value: self,
            sim_state,
        }
    }
}

impl<T> AsSignal for T {}
