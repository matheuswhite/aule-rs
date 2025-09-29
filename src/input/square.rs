use crate::{input::Input, signal::Signal};
use core::time::Duration;

pub struct Square {
    amplitude: f32,
    period: Duration,
    offset: f32,
    sim_time: Duration,
}

impl Square {
    pub fn new(amplitude: f32, period: Duration, offset: f32) -> Self {
        Square {
            amplitude,
            period,
            offset,
            sim_time: Duration::default(),
        }
    }
}

impl Input for Square {
    type Output = f32;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        self.sim_time += dt;

        let t = self.sim_time.as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value = if (t % period_secs) < (period_secs / 2.0) {
            self.amplitude
        } else {
            0.0
        } + self.offset;

        Signal { value, dt }
    }
}
