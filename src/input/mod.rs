use std::{ops::Shr, time::Duration};

use crate::signal::Signal;

pub mod impulse;
pub mod ramp;
pub mod setpoint;
pub mod sinusoid;
pub mod step;

/// The `Input` trait defines the interface for input blocks in a block-based system.
/// It provides a method to output a signal based on a given time step.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyInput;
///
/// impl Input for MyInput {
///     fn output(&mut self, dt: Duration) -> Signal {
///         // Generate a signal based on the time step
///         Signal {
///             value: 1.0, // Example value
///             dt, // Use the provided duration
///         }
///     }
/// }
///
/// let mut input = MyInput;
/// let dt = Duration::from_secs(1);
/// let output_signal = input.output(dt);
/// assert_eq!(output_signal.value, 1.0);
/// assert_eq!(output_signal.dt, dt);
/// ```
pub trait Input {
    fn output(&mut self, dt: Duration) -> Signal;
}

/// The `AsInput` trait provides a way to treat any type that implements the `Input` trait as a dynamic input.
/// It allows for dynamic dispatch of the `output` method.
/// This is useful for scenarios where you want to work with inputs without knowing their concrete types at compile time.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyInput;
///
/// impl Input for MyInput {
///     fn output(&mut self, dt: Duration) -> Signal {
///         Signal {
///             value: 1.0, // Example value
///             dt, // Use the provided duration
///         }
///     }
/// }
///
/// impl AsInput for MyInput {}
///
/// let mut input = MyInput;
/// let mut input: &mut dyn Input = input.as_input();
/// let dt = Duration::from_secs(1);
/// let output_signal = input.output(dt);
/// assert_eq!(output_signal.value, 1.0);
/// assert_eq!(output_signal.dt, dt);
/// ```
pub trait AsInput: Sized + Input + 'static {
    fn as_input(&mut self) -> &mut dyn Input {
        self
    }
}

impl Shr<&mut dyn Input> for Duration {
    type Output = Signal;

    /// Puts the duration into an input block, producing a signal based on the duration.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyInput;
    ///
    /// impl Input for MyInput {
    ///     fn output(&mut self, dt: Duration) -> Signal {
    ///         Signal {
    ///             value: 1.0, // Example value
    ///             dt, // Use the provided duration
    ///         }
    ///     }
    /// }
    ///
    /// impl AsInput for MyInput {}
    ///
    /// let mut input = MyInput;
    /// let dt = Duration::from_secs(1);
    /// let output_signal = dt >> input.as_input();
    /// assert_eq!(output_signal.value, 1.0);
    /// assert_eq!(output_signal.dt, dt);
    /// ```
    fn shr(self, rhs: &mut dyn Input) -> Self::Output {
        rhs.output(self)
    }
}
