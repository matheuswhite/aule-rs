use crate::block::Block;
use crate::prelude::SimulationState;
use num_traits::Float;

#[derive(Debug, Clone)]
pub struct Saturation<T>
where
    T: Float,
{
    min: T,
    max: T,
    last_output: Option<T>,
}

impl<T> Saturation<T>
where
    T: Float,
{
    pub fn new(min: T, max: T) -> Self {
        Self {
            min,
            max,
            last_output: None,
        }
    }
}

impl<T> Block for Saturation<T>
where
    T: Float,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let saturated_value = input.clamp(self.min, self.max);
        self.last_output = Some(saturated_value);
        saturated_value
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        self.last_output = None;
    }
}
