use crate::{block::Block, signal::Signal};
use core::{f32::consts::PI, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth {
    amplitude: f32,
    period: Duration,
    offset: f32,
}

impl Sawtooth {
    pub fn new(amplitude: f32, period: Duration, offset: f32) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
        }
    }
}

impl Default for Sawtooth {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            period: Duration::from_secs_f32(2.0 * PI),
            offset: 0.0,
        }
    }
}

impl Block for Sawtooth {
    type Input = ();
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let t = input.delta.sim_time().as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value = (self.amplitude / period_secs) * (t % period_secs) + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
