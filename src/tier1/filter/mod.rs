use crate::{block::Block, signal::Signal, time::EndlessTime};
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
        F: FnMut(Signal<()>) -> Option<Signal<Self::SignalValue>> + 'static,
    {
        let dt = self.dt().as_secs_f32();
        EndlessTime::new(dt).map_while(move |dt| {
            let Some(input) = input_generator(dt.map(|_| ())) else {
                return None;
            };

            Some(self.output(input))
        })
    }
}
