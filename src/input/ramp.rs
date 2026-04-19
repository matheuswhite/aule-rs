use crate::{block::Block, prelude::SimulationState};
use core::ops::Mul;
use num_traits::One;

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    value: T,
}

impl<T> Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    pub fn new(value: T) -> Self {
        Ramp { value }
    }
}

impl<T> Default for Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    fn default() -> Self {
        Self { value: T::one() }
    }
}

impl<T> Block for Ramp<T>
where
    T: One + Copy + Mul<f64, Output = T>,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        self.value * sim_state.sim_time().as_secs_f64()
    }
}
