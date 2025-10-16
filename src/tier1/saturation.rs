use crate::block::Block;
use crate::signal::Signal;

#[derive(Debug, Clone)]
pub struct Saturation<T: Ord + Clone> {
    min: T,
    max: T,
    last_output: Option<T>,
}

impl<T: Ord + Clone> Block for Saturation<T> {
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<T>) -> Signal<T> {
        let saturated_value = input.value.clamp(self.min.clone(), self.max.clone());
        let output = Signal {
            value: saturated_value,
            delta: input.delta,
        };
        self.last_output = Some(output.value.clone());
        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output.clone()
    }
}
