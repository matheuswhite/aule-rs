use crate::block::Block;
use crate::signal::Signal;
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

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let saturated_value = input.value.clamp(self.min, self.max);
        let output = Signal {
            value: saturated_value,
            delta: input.delta,
        };
        self.last_output = Some(output.value);
        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        self.last_output = None;
    }
}
