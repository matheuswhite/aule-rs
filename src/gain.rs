use crate::block::{AsBlock, Block, Signal};

/// Gain block that multiplies the input signal by a constant value.
/// This is useful for scaling the amplitude of a signal.
/// It can be used in audio processing or other signal manipulation tasks.
///
/// # Example:
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// let mut gain = Gain::new(2.0);
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// let output_signal = gain.output(input_signal);
/// assert_eq!(output_signal.value, 2.0);
/// ```
pub struct Gain {
    value: f32,
    last_output: Option<Signal>,
}

impl Gain {
    /// Creates a new Gain block with the specified gain value.
    ///
    /// # Parameters:
    /// - `value`: The gain factor by which the input signal will be multiplied.
    /// # Returns:
    /// A new instance of `Gain`.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    ///
    /// let gain = Gain::new(1.5);
    /// ```
    pub fn new(value: f32) -> Self {
        Gain {
            value,
            last_output: None,
        }
    }
}

impl Block for Gain {
    /// Processes the input signal by multiplying it with the gain value.
    ///
    /// # Parameters:
    /// - `input`: The input signal to be processed.
    /// # Returns:
    /// The output signal, which is the input signal multiplied by the gain value.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut gain = Gain::new(2.0);
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = gain.output(input_signal);
    /// assert_eq!(output_signal.value, 2.0);
    /// ```
    fn output(&mut self, input: Signal) -> Signal {
        let output = Signal {
            value: input.value * self.value,
            dt: input.dt,
        };

        self.last_output = Some(output);
        output
    }

    /// Returns the last output signal processed by this block.
    /// If no output has been produced yet, it returns `None`.
    ///
    /// # Returns:
    /// An `Option<Signal>` containing the last output signal, or `None` if no output has been produced.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut gain = Gain::new(2.0);
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// gain.output(input_signal);
    /// assert_eq!(gain.last_output().unwrap().value, 2.0);
    /// ```
    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for Gain {}
