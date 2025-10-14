use crate::{input::Input, signal::Signal};
use core::{f32::consts::PI, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth {
    amplitude: f32,
    period: Duration,
    offset: f32,
    sim_time: Duration,
}

impl Sawtooth {
    pub fn new(amplitude: f32, period: Duration, offset: f32) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
            sim_time: Duration::default(),
        }
    }
}

impl Default for Sawtooth {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            period: Duration::from_secs_f32(2.0 * PI),
            offset: 0.0,
            sim_time: Default::default(),
        }
    }
}

impl Input for Sawtooth {
    type Output = f32;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        self.sim_time += dt;

        let t = self.sim_time.as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value =
            (self.amplitude.clone() / period_secs) * (t % period_secs) + self.offset.clone();

        Signal { value, dt }
    }
}
