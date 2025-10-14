use crate::block::Block;
use crate::signal::Signal;

#[derive(Debug, Clone)]
pub struct Saturation<T: Ord + Clone> {
    min: T,
    max: T,
    last_output: Option<Signal<T>>,
}

impl<T: Ord + Clone> Block for Saturation<T> {
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<T>) -> Signal<T> {
        let saturated_value = input.value.clamp(self.min.clone(), self.max.clone());
        let output = Signal {
            value: saturated_value,
            dt: input.dt,
        };
        self.last_output = Some(output.clone());
        output
    }

    fn last_output(&self) -> Option<Signal<T>> {
        self.last_output.clone()
    }
}
