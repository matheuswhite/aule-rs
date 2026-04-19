use crate::{block::Block, prelude::SimulationState};
use num_traits::{Bounded, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse<T>
where
    T: Zero + Bounded,
{
    value: Option<T>,
}

impl<T> Impulse<T>
where
    T: Zero + Bounded,
{
    pub fn new(value: T) -> Self {
        Impulse { value: Some(value) }
    }
}

impl<T> Default for Impulse<T>
where
    T: Zero + Bounded,
{
    fn default() -> Self {
        Self {
            value: Some(T::max_value()),
        }
    }
}

impl<T> Block for Impulse<T>
where
    T: Zero + Bounded,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let Some(value) = self.value.take() else {
            return T::zero();
        };

        self.value = None;
        value
    }
}
