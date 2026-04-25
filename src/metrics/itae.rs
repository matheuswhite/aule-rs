use crate::{block::Block, prelude::SimulationState};
use num_traits::Float;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ITAE<T>
where
    T: Float,
{
    acc: T,
    n: usize,
}

impl<T> ITAE<T>
where
    T: Float,
{
    pub fn value(&self) -> T {
        if self.n == 0 {
            T::zero()
        } else {
            self.acc / T::from(self.n as f32).unwrap()
        }
    }
}

impl<T> Block for ITAE<T>
where
    T: Float,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        self.n += 1;
        self.acc = self.acc + input.abs() * T::from(self.n as f32).unwrap();
        input
    }

    fn reset(&mut self) {
        self.acc = T::zero();
        self.n = 0;
    }
}
