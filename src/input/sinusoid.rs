use std::time::Duration;

use crate::{
    input::{AsInput, Input},
    signal::Signal,
};

pub struct Sinusoid {
    amplitude: f32,
    frequency: f32,
    phase: f32,
    sim_time: Duration,
}

impl Sinusoid {
    /// Creates a new Sinusoid instance with the specified parameters.
    ///
    /// # Parameters
    /// * `amplitude` - The amplitude of the sinusoidal wave.
    /// * `frequency` - The frequency of the sinusoidal wave in Hz.
    /// * `phase` - The phase shift of the sinusoidal wave in radians.
    /// # Returns
    /// A new `Sinusoid` instance with the specified parameters.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    ///
    /// let mut sinusoid = Sinusoid::new(1.0, 1.0, 0.0);
    /// let signal = sinusoid.output(std::time::Duration::from_secs(1));
    /// assert!(signal.value >= -1.0 && signal.value <= 1.0); // Value should be within amplitude range
    /// ```
    pub fn new(amplitude: f32, frequency: f32, phase: f32) -> Self {
        Sinusoid {
            amplitude,
            frequency,
            phase,
            sim_time: Duration::default(),
        }
    }
}

impl Input for Sinusoid {
    /// Outputs the current value of the sinusoidal wave based on the simulation time.
    ///
    /// # Parameters
    /// * `dt` - The duration since the last output, which is used to update the simulation time.
    /// # Returns
    /// A `Signal` containing the current value of the sinusoidal wave.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    ///
    /// let mut sinusoid = Sinusoid::new(1.0, 1.0, 0.0);
    /// let signal = sinusoid.output(std::time::Duration::from_secs(1));
    /// assert!(signal.value >= -1.0 && signal.value <= 1.0); // Value should be within amplitude range
    /// ```
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;
        let t = self.sim_time.as_secs_f32();
        let value =
            self.amplitude * (2.0 * std::f32::consts::PI * self.frequency * t + self.phase).sin();
        Signal { value, dt }
    }
}

impl AsInput for Sinusoid {}
