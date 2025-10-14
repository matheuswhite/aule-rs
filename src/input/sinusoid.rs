use crate::{input::Input, signal::Signal};
use core::{f32::consts::PI, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Sinusoid {
    amplitude: f32,
    period: Duration,
    phase: f32,
    sim_time: Duration,
}

impl Sinusoid {
    pub fn new(amplitude: f32, period: Duration, phase: f32) -> Self {
        Sinusoid {
            amplitude,
            period,
            phase,
            sim_time: Duration::default(),
        }
    }
}

impl Default for Sinusoid {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            period: Duration::from_secs_f32(2.0 * PI),
            phase: 0.0,
            sim_time: Default::default(),
        }
    }
}

impl Input for Sinusoid {
    type Output = f32;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        self.sim_time += dt;
        let t = self.sim_time.as_secs_f32();
        let value = self.amplitude * (t / self.period.as_secs_f32() + self.phase);
        #[cfg(feature = "std")]
        let value = value.sin();
        #[cfg(not(feature = "std"))]
        let value = libm::sin(value as f64) as f32;
        Signal { value, dt }
    }
}
