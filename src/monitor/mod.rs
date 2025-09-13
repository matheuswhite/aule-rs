use crate::signal::Signal;
use alloc::vec::Vec;

pub mod plotter;
pub mod printer;
pub mod writer;

/// The `Monitor` trait defines the interface for monitoring signals in a block-based system.
/// It provides a method to display or log the input signal.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyMonitor;
///
/// impl Monitor for MyMonitor {
///     fn show(&mut self, inputs: Vec<Signal>) {
///         // Display or log the input signal
///         println!("Monitoring signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
///     }
/// }
///
/// let mut monitor = MyMonitor;
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// monitor.show(vec![input_signal]);
/// ```
pub trait Monitor {
    fn show(&mut self, inputs: Vec<Signal>);
}

/// The `AsMonitor` trait provides a way to treat any type that implements the `Monitor` trait as a dynamic monitor.
/// It allows for dynamic dispatch of the `show` method.
/// This is useful for scenarios where you want to work with monitors without knowing their concrete types at compile time.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyMonitor;
///
/// impl Monitor for MyMonitor {
///     fn show(&mut self, inputs: Vec<Signal>) {
///         println!("Monitoring signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
///     }
/// }
///
/// impl AsMonitor for MyMonitor {}
///
/// let mut monitor = MyMonitor;
/// let mut monitor: &mut dyn Monitor = monitor.as_monitor();
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// monitor.show(vec![input_signal]);
/// ```
pub trait AsMonitor: Sized + Monitor + 'static {
    fn as_monitor(&mut self) -> &mut dyn Monitor {
        self
    }
}
