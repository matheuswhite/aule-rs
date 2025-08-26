use crate::block::{AsBlock, Block};
use crate::signal::Signal;

/// A block that limits the output signal to a specified minimum and maximum value.
/// This is useful for preventing signal values from exceeding certain thresholds.
///
/// # Examples:
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// let mut saturation = Saturation::from((-1.0, 1.0));
/// let input_signal = Signal { value: 2.0, dt: Duration::from_secs(1) };
/// let output_signal = saturation.output(input_signal);
/// assert_eq!(output_signal.value, 1.0); // Clamped to max
/// let input_signal = Signal { value: -2.0, dt: Duration::from_secs(1) };
/// let output_signal = saturation.output(input_signal);
/// assert_eq!(output_signal.value, -1.0); // Clamped to min
/// ```
pub struct Saturation {
    min: f32,
    max: f32,
    last_output: Option<Signal>,
}

impl From<f32> for Saturation {
    /// Creates a new `Saturation` block with symmetric limits.
    /// The output will be clamped between `-value` and `value`.
    ///
    /// # Parameters:
    /// - `value`: The absolute value for the saturation limits.
    /// # Returns:
    /// A new instance of `Saturation`.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    ///
    /// let saturation = Saturation::from(1.0);
    /// ```
    fn from(value: f32) -> Self {
        Saturation {
            min: -value,
            max: value,
            last_output: None,
        }
    }
}

impl From<(f32, f32)> for Saturation {
    /// Creates a new `Saturation` block with specified minimum and maximum limits.
    /// The output will be clamped between `min` and `max`.
    ///
    /// # Parameters:
    /// - `min`: The minimum limit for the output signal.
    /// - `max`: The maximum limit for the output signal.
    /// # Returns:
    /// A new instance of `Saturation`.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    ///
    /// let saturation = Saturation::from((-1.0, 1.0));
    /// ```
    fn from((min, max): (f32, f32)) -> Self {
        Saturation {
            min,
            max,
            last_output: None,
        }
    }
}

impl Block for Saturation {
    /// Processes the input signal by clamping its value between the defined minimum and maximum limits.
    /// The time delta (`dt`) of the input signal is preserved in the output signal.
    /// The last output signal is stored and can be retrieved using the `last_output` method.
    ///
    /// # Parameters:
    /// - `input`: The input signal to be processed.
    /// # Returns:
    /// The output signal, which is the input signal clamped between the minimum and maximum limits.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut saturation = Saturation::from((-1.0, 1.0));
    /// let input_signal = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let output_signal = saturation.output(input_signal);
    /// assert_eq!(output_signal.value, 1.0); // Clamped to max
    /// ```
    fn output(&mut self, input: Signal) -> Signal {
        let saturated_value = input.value.clamp(self.min, self.max);
        let output = Signal {
            value: saturated_value,
            dt: input.dt,
        };
        self.last_output = Some(output);
        output
    }

    /// Returns the last output signal produced by the block, if any.
    /// This can be useful for tracking the most recent output state of the block.
    ///
    /// # Returns:
    /// An `Option<Signal>` containing the last output signal, or `None` if no output has been produced yet.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut saturation = Saturation::from((-1.0, 1.0));
    /// let input_signal = Signal { value: 0.5, dt: Duration::from_secs(1) };
    /// let _ = saturation.output(input_signal);
    /// assert_eq!(saturation.last_output().unwrap().value, 0.5);
    /// ```
    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for Saturation {}
