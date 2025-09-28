use crate::signal::Signal;

pub mod plotter;
pub mod printer;
pub mod writer;

pub trait Output {
    fn show(&mut self, inputs: &[Signal]);
}

pub trait AsOutput: Sized + Output + 'static {
    fn as_output(&mut self) -> &mut dyn Output {
        self
    }
}
