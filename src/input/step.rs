use crate::{block::Block, prelude::SimulationState};
use core::fmt::Display;
use num_traits::One;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step<T>
where
    T: One + Copy,
{
    value: T,
}

impl<T> Step<T>
where
    T: One + Copy,
{
    pub fn new(value: T) -> Self {
        Step { value }
    }
}

impl<T> Default for Step<T>
where
    T: One + Copy,
{
    fn default() -> Self {
        Step { value: T::one() }
    }
}

impl<T> Display for Step<T>
where
    T: One + Copy + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Step({})", self.value)
    }
}

impl<T> Block for Step<T>
where
    T: One + Copy,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        self.value
    }
}
