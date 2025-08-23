use crate::block::{AsBlock, Block};
use crate::error::ErrorMetric;
use crate::prelude::{GoodHart, IAE, ISE, ITAE};
use crate::signal::Signal;

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
#[derive(Debug, Clone, PartialEq)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    last_input: f32,
    last_integral: f32,
    last_output: Option<Signal>,
    iae: Option<IAE>,
    ise: Option<ISE>,
    itae: Option<ITAE>,
    good_hart: Option<GoodHart>,
    anti_windup: Option<(f32, f32)>,
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
            iae: None,
            ise: None,
            itae: None,
            good_hart: None,
            anti_windup: None,
        }
    }

    pub fn with_iae(mut self) -> Self {
        self.iae = Some(IAE::new());
        self
    }

    pub fn with_ise(mut self) -> Self {
        self.ise = Some(ISE::new());
        self
    }

    pub fn with_itae(mut self) -> Self {
        self.itae = Some(ITAE::new());
        self
    }

    pub fn with_good_hart(mut self, alpha1: f32, alpha2: f32, alpha3: f32) -> Self {
        self.good_hart = Some(GoodHart::new(alpha1, alpha2, alpha3));
        self
    }

    /// Enables anti-windup by clamping the output within the specified min and max bounds.
    ///
    /// # Parameters
    /// - `min`: Minimum output value.
    /// - `max`: Maximum output value.
    /// # Returns
    /// The modified `PID` instance with anti-windup enabled.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// let mut pid_anti_windup = PID::new(1.0, 0.5, 0.01).with_anti_windup(-50.0, 50.0);
    /// let mut pid = PID::new(1.0, 0.5, 0.01);
    /// for i in 0..100 {
    ///     let input_signal = Signal { value: 10.0, dt: std::time::Duration::from_secs(1) };
    ///     let output_signal = pid_anti_windup.output(input_signal);
    ///     assert!(-50.0 <= output_signal.value && output_signal.value <= 50.0);
    ///     assert!(pid_anti_windup.integral() <= 80.0, "Integral term should not grow excessively: {}", pid_anti_windup.integral());
    ///     let _output_signal = pid.output(input_signal);
    ///     assert!(pid.integral() >= pid_anti_windup.integral(), "Integral term should grow larger without anti-windup: {}", pid.integral());
    /// }
    /// ```
    /// # Note
    /// Anti-windup helps prevent the integral term from accumulating excessively when the controller
    /// output is saturated. This can improve the performance of the PID controller in systems
    /// where the actuator has limits. If you want only to limit output, without affecting the integral term,
    /// consider using a separate output clamping mechanism.
    pub fn with_anti_windup(mut self, min: f32, max: f32) -> Self {
        self.anti_windup = Some((min, max));
        self
    }

    pub fn error_metrics(&self) -> String {
        format!(
            "\n  IAE: {}\n  ISE: {}\n  ITAE: {}\n  Good Hart: {}",
            self.iae
                .as_ref()
                .map_or("N/A".to_string(), |e| e.value().to_string()),
            self.ise
                .as_ref()
                .map_or("N/A".to_string(), |e| e.value().to_string()),
            self.itae
                .as_ref()
                .map_or("N/A".to_string(), |e| e.value().to_string()),
            self.good_hart
                .as_ref()
                .map_or("N/A".to_string(), |gh| gh.value().to_string())
        )
    }

    pub fn clear_integral(&mut self) {
        self.last_integral = 0.0;
    }

    pub fn integral(&self) -> f32 {
        self.last_integral
    }

    pub fn error(&self) -> f32 {
        self.last_input
    }

    pub fn kp_mut(&mut self) -> &mut f32 {
        &mut self.kp
    }

    pub fn ki_mut(&mut self) -> &mut f32 {
        &mut self.ki
    }

    pub fn kd_mut(&mut self) -> &mut f32 {
        &mut self.kd
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
        if let Some(iae) = &mut self.iae {
            iae.update([input]);
        }

        if let Some(ise) = &mut self.ise {
            ise.update([input]);
        }

        if let Some(itae) = &mut self.itae {
            itae.update([input]);
        }

        let proportional = input.value;
        let integral = self.last_integral + input.value * input.dt.as_secs_f32();
        let derivative = (input.value - self.last_input) / input.dt.as_secs_f32();

        let output = self.kp * proportional + self.ki * integral + self.kd * derivative;
        let (output, integral) = if let Some((min, max)) = self.anti_windup {
            if output < min || output > max {
                (output.clamp(min, max), self.last_integral)
            } else {
                (output, integral)
            }
        } else {
            (output, integral)
        };

        let output = Signal {
            value: output,
            dt: input.dt,
        };

        self.last_output = Some(output);
        self.last_input = input.value;
        self.last_integral = integral;

        if let Some(good_hart) = &mut self.good_hart {
            good_hart.update([input, output]);
        }

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
