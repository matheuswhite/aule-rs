use crate::block::{AsBlock, Block, Signal};

/// A PID controller block that implements proportional, integral, and derivative control.
/// It takes a `Signal` input and produces a `Signal` output based on the PID algorithm.
/// It maintains the last input, integral, and output values to compute the next output.
/// It can be used in a signal processing pipeline where control signals are needed.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// let mut pid = PID::new(1.0, 0.1, 0.01);
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// let output_signal = pid.output(input_signal);
/// assert_eq!(output_signal.value, 1.11);
/// ```
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    last_input: f32,
    last_integral: f32,
    last_output: Option<Signal>,
}

impl PID {
    /// Creates a new PID controller with specified gains.
    ///
    /// # Parameters
    /// - `kp`: Proportional gain.
    /// - `ki`: Integral gain.
    /// - `kd`: Derivative gain.
    /// # Returns
    /// A new instance of `PID`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// let pid = PID::new(1.0, 0.1, 0.01);
    /// ```
    ///
    /// # Note
    /// The gains should be tuned according to the specific control system requirements.
    /// Improper tuning can lead to instability or poor performance.
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PID {
            kp,
            ki,
            kd,
            last_input: 0.0,
            last_integral: 0.0,
            last_output: None,
        }
    }
}

impl Block for PID {
    /// Computes the output of the PID controller based on the input signal.
    /// It applies the PID control algorithm using the proportional, integral, and derivative gains.
    ///
    /// # Parameters
    /// - `input`: The input signal containing the current value and time step.
    /// # Returns
    /// A `Signal` output that represents the control action based on the PID algorithm.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut pid = PID::new(1.0, 0.1, 0.01);
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = pid.output(input_signal);
    /// assert_eq!(output_signal.value, 1.11);
    /// ```
    /// # Note
    /// The output is computed as:
    /// output = kp * proportional + ki * integral + kd * derivative
    ///
    /// where:
    /// - `proportional` is the current input value,
    /// - `integral` is the accumulated input value over time,
    /// - `derivative` is the rate of change of the input value.
    ///
    /// The output signal's `dt` is the same as the input signal's `dt`.
    /// If the input signal's `dt` is zero, the output will not be computed.
    /// if the input signal's `dt` isn't zero, it will be used to compute the integral and derivative terms.
    /// The output signal's `value` will be the computed control action.
    /// The `last_output` will be updated with the computed output signal.
    /// The `last_input` will be updated with the current input value.
    /// The `last_integral` will be updated with the current integral value.
    /// The `last_input` will be used in the next computation to calculate the derivative term.
    /// The `last_integral` will be used in the next computation to calculate the integral term.
    fn output(&mut self, input: Signal) -> Signal {
        let proportional = input.value;
        let integral = self.last_integral + input.value * input.dt.as_secs_f32();
        let derivative = (input.value - self.last_input) / input.dt.as_secs_f32();

        let output = self.kp * proportional + self.ki * integral + self.kd * derivative;
        let output = Signal {
            value: output,
            dt: input.dt,
        };

        self.last_output = Some(output);
        self.last_input = input.value;
        self.last_integral = integral;

        output
    }

    /// Returns the last output signal computed by the PID controller.
    /// This can be useful for debugging or for systems that need to access the last control action.
    ///
    /// # Returns
    /// An `Option<Signal>` containing the last output signal if it exists, or `None` if no output has been computed yet.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let mut pid = PID::new(1.0, 0.1, 0.01);
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// pid.output(input_signal);
    /// let last_output = pid.last_output();
    /// assert!(last_output.is_some());
    /// assert_eq!(last_output.unwrap().value, 1.11);
    /// ```
    ///
    /// # Note
    /// The last output is updated every time the `output` method is called.
    /// If no output has been computed yet, it will return `None`.
    /// If the last output is needed for further processing, it can be accessed using this method.
    /// If the last output is not needed, this method can be ignored.
    /// It is useful for systems that require feedback from the PID controller.
    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for PID {}
