use crate::{prelude::SimulationState, signal::Signal};

pub trait Block {
    type Input;
    type Output;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let value = self.block(input.value, input.sim_state);
        Signal {
            value,
            sim_state: input.sim_state,
        }
    }

    fn last_output(&self) -> Option<Self::Output> {
        None
    }

    fn as_block(&mut self) -> &mut dyn Block<Input = Self::Input, Output = Self::Output>
    where
        Self: Sized + 'static,
    {
        self
    }

    fn reset(&mut self) {}
}
