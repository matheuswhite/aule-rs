use std::time::Duration;

use crate::{
    input::{AsInput, Input},
    signal::Signal,
};

pub struct Ramp {
    value: f32,
    sim_time: Duration,
}

impl Ramp {
    /// Creates a new Ramp instance with the specified value and duration.
    ///
    /// # Parameters
    /// * `value` - The value to set for the ramp input.
    /// # Returns
    /// A new `Ramp` instance with the specified value and duration.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    ///
    /// let mut ramp = Ramp::new(5.0);
    /// let signal = ramp.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 5.0);
    /// ```
    pub fn new(value: f32) -> Self {
        Ramp {
            value,
            sim_time: Duration::default(),
        }
    }
}

impl Input for Ramp {
    /// Outputs the current value of the ramp based on the simulation time.
    ///
    /// # Parameters
    /// * `dt` - The duration since the last output, which is used to update the simulation time.
    /// # Returns
    /// A `Signal` containing the current value of the ramp, which is calculated as `value * sim_time`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    ///
    /// let mut ramp = Ramp::new(5.0);
    /// let signal = ramp.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 5.0);
    /// let signal = ramp.output(std::time::Duration::from_secs(1));
    /// assert_eq!(signal.value, 10.0); // Subsequent calls return incremented values
    /// ```
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;
        let value = self.value * self.sim_time.as_secs_f32();
        Signal { value, dt }
    }
}

impl AsInput for Ramp {}
