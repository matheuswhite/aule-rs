use crate::input::{AsInput, Input, Signal};
use std::time::Duration;

/// A step input block that outputs a constant value after a specified duration.
/// The value can be set to any f32 value, and it will output that value after 1 second of simulation time.
/// If the simulation time is less than 1 second, it outputs 0.0.
/// This is useful for testing and simulating systems that require a step input signal.
///
/// # Example
/// ```
/// use aule::prelude::*;
///
/// let mut step = Step::new().with_value(5.0);
/// let signal = step.output(std::time::Duration::from_secs(2));
/// assert_eq!(signal.value, 5.0);
/// ```
pub struct Step {
    value: f32,
    sim_time: Duration,
}

impl Step {
    /// Creates a new Step instance with a default value of 1.0.
    ///
    /// # Returns
    /// A new `Step` instance with the default value set to 1.0.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut step = Step::new();
    ///
    /// let value = step.output(Duration::from_secs(2)).value;
    /// assert_eq!(value, 1.0);
    /// ```
    pub fn new() -> Self {
        Step {
            value: 1.0,
            sim_time: Duration::default(),
        }
    }

    /// Sets the value of the step input.
    ///
    /// # Parameters
    /// * `value` - The value to set for the step input.
    /// # Returns
    /// A new `Step` instance with the specified value.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut step = Step::new().with_value(5.0);
    ///
    /// let value = step.output(Duration::from_secs(2)).value;
    /// assert_eq!(value, 5.0);
    /// ```
    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
}

impl Input for Step {
    /// Outputs a signal based on the current simulation time.
    /// If the simulation time is greater than or equal to 1 second, it outputs the
    /// specified value; otherwise, it outputs 0.0.
    ///
    /// # Parameters
    /// * `dt` - The duration since the last output.
    /// # Returns
    /// A `Signal` containing the value and the duration since the last output.
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut step = Step::new().with_value(5.0);
    /// let signal = step.output(std::time::Duration::from_secs(2));
    /// assert_eq!(signal.value, 5.0);
    /// ```
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;

        Signal {
            value: self.value,
            dt,
        }
    }
}

impl AsInput for Step {}
