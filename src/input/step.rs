use crate::{block::Block, signal::Signal};
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

impl<T> Block for Step<T>
where
    T: One + Copy,
{
    type Input = ();
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        Signal {
            value: self.value,
            delta: input.delta,
        }
    }
}
