use crate::input::{AsInput, Input, Signal};
use core::time::Duration;

/// A setpoint input block that outputs a constant value.
/// The value can be set to any f32 value, and it will output that value whenever requested.
/// This is useful for providing a constant reference signal in control systems.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// let mut setpoint = Setpoint::new(5.0);
/// let signal = setpoint.output(std::time::Duration::from_secs(1));
/// assert_eq!(signal.value, 5.0);
/// ```
pub struct Setpoint {
    value: f32,
}

impl Setpoint {
    /// Creates a new Setpoint instance with the specified value.
    ///
    /// # Parameters
    /// * `value` - The value to set for the setpoint input.
    /// # Returns
    /// A new `Setpoint` instance with the specified value.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// let mut setpoint = Setpoint::new(5.0);
    /// let signal = setpoint.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 5.0);
    /// ```
    pub fn new(value: f32) -> Self {
        Setpoint { value }
    }
}

impl Input for Setpoint {
    /// Outputs the current value of the setpoint.
    ///
    /// # Parameters
    /// * `dt` - The duration since the last output, which is not used in this case.
    /// # Returns
    /// A `Signal` containing the current value of the setpoint.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// let mut setpoint = Setpoint::new(5.0);
    /// let signal = setpoint.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 5.0);
    /// ```
    fn output(&mut self, dt: Duration) -> Signal {
        Signal {
            value: self.value,
            dt,
        }
    }
}

impl AsInput for Setpoint {}
