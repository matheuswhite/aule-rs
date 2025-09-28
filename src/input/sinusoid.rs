use crate::{
    input::{AsInput, Input},
    signal::Signal,
};
use core::time::Duration;

pub struct Sinusoid {
    amplitude: f32,
    frequency: f32,
    phase: f32,
    sim_time: Duration,
}

impl Sinusoid {
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
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;
        let t = self.sim_time.as_secs_f32();
        let value =
            self.amplitude * (2.0 * core::f32::consts::PI * self.frequency * t + self.phase);
        #[cfg(feature = "std")]
        let value = value.sin();
        #[cfg(not(feature = "std"))]
        let value = libm::sin(value as f64) as f32;
        Signal { value, dt }
    }
}

impl AsInput for Sinusoid {}
