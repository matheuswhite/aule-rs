use crate::signal::Signal;

pub mod plotter;
pub mod printer;
pub mod writer;

/// The `Output` trait defines the interface for monitoring signals in a block-based system.
/// It provides a method to display or log the input signal.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyOutput;
///
/// impl Output for MyOutput {
///     fn show(&mut self, inputs: &[Signal]) {
///         // Display or log the input signal
///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
///     }
/// }
///
/// let mut monitor = MyOutput;
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// monitor.show(&[input_signal]);
/// ```
pub trait Output {
    fn show(&mut self, inputs: &[Signal]);
}

/// The `AsOutput` trait provides a way to treat any type that implements the `Output` trait as a dynamic monitor.
/// It allows for dynamic dispatch of the `show` method.
/// This is useful for scenarios where you want to work with monitors without knowing their concrete types at compile time.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyOutput;
///
/// impl Output for MyOutput {
///     fn show(&mut self, inputs: &[Signal]) {
///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
///     }
/// }
///
/// impl AsOutput for MyOutput {}
///
/// let mut monitor = MyOutput;
/// let mut monitor: &mut dyn Output = monitor.as_monitor();
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// monitor.show(&[input_signal]);
/// ```
pub trait AsOutput: Sized + Output + 'static {
    fn as_monitor(&mut self) -> &mut dyn Output {
        self
    }
}
