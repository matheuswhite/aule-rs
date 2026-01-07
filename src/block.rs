use crate::signal::Signal;

pub trait Block {
    type Input;
    type Output;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output>;

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

    fn save_state(&mut self) {}

    fn restore_state(&mut self) {}
}
