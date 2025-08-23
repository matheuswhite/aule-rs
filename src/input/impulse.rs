use crate::{
    input::{AsInput, Input},
    signal::Signal,
};
use core::time::Duration;

/// An impulse input block that outputs a value once and then resets to zero.
/// The value can be set to any f32 value, and it will output that value once when requested.
/// This is useful for generating a one-time signal in control systems.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// let mut impulse = Impulse::new(5.0);
/// let signal = impulse.output(std::time::Duration::from_secs(1));
/// assert_eq!(signal.value, 5.0);
/// let signal = impulse.output(std::time::Duration::from_secs(1));
/// assert_eq!(signal.value, 0.0); // Subsequent calls return 0.0
/// ```
pub struct Impulse {
    value: Option<f32>,
}

impl Impulse {
    /// Creates a new Impulse instance with the specified value.
    ///
    /// # Parameters
    /// * `value` - The value to set for the impulse input.
    /// # Returns
    /// A new `Impulse` instance with the specified value.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// let mut impulse = Impulse::new(5.0);
    /// let signal = impulse.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 5.0);
    /// let signal = impulse.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 0.0); // Subsequent calls return 0.0
    /// ```
    pub fn new(value: f32) -> Self {
        Impulse { value: Some(value) }
    }
}

impl Input for Impulse {
    /// Outputs the current value of the impulse and resets it to zero.
    ///
    /// # Parameters
    /// * `dt` - The duration since the last output, which is not used in this case.
    /// # Returns
    /// A `Signal` containing the current value of the impulse, which is then reset to zero.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// let mut impulse = Impulse::new(5.0);
    /// let signal = impulse.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 5.0);
    /// let signal = impulse.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 0.0); // Subsequent calls return 0.0
    /// ```
    fn output(&mut self, dt: Duration) -> Signal {
        match self.value.take() {
            Some(value) => {
                self.value = None; // Reset value after output
                Signal { value, dt }
            }
            None => Signal { value: 0.0, dt }, // If no value is set, return 0.0
        }
    }
}

impl AsInput for Impulse {}
