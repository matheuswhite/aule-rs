use crate::block::Block;
use crate::math::float_point::FloatPoint;
use crate::prelude::SimulationState;

#[derive(Debug, Clone)]
pub struct Saturation<T> {
    min: T,
    max: T,
    last_output: Option<T>,
}

impl<T> Saturation<T> {
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
    T: Clone + FloatPoint,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let saturated_value = input.clamp(self.min, self.max);
        self.last_output = Some(saturated_value);
        saturated_value
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output.clone()
    }

    fn reset(&mut self) {
        self.last_output = None;
    }
}
