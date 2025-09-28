use crate::{
    input::{AsInput, Input},
    signal::Signal,
};
use core::time::Duration;

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

impl Input for Sawtooth {
    fn output(&mut self, dt: Duration) -> Signal {
        self.sim_time += dt;

        let t = self.sim_time.as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value = (self.amplitude / period_secs) * (t % period_secs) + self.offset;

        Signal { value, dt }
    }
}

impl AsInput for Sawtooth {}
