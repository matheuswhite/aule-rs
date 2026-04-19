use crate::{
    block::Block, prelude::SimulationState, signal::Signal, simulation::EndlessSimulation,
};
use core::time::Duration;

pub mod first_order;
pub mod second_order;

pub trait Filter: Block<Input = Self::SignalValue, Output = Self::SignalValue> {
    type SignalValue;

    fn dt(&self) -> Duration;
    fn filter<F>(
        &mut self,
        mut input_generator: F,
    ) -> impl Iterator<Item = Signal<Self::SignalValue>>
    where
        F: FnMut(SimulationState) -> Option<Signal<Self::SignalValue>> + 'static,
    {
        let dt = self.dt().as_secs_f32();
        EndlessSimulation::new(dt).map_while(move |sim_state| {
            let Some(input) = input_generator(sim_state) else {
                return None;
            };

            Some(self.output(input))
        })
    }
}
