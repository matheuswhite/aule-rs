use crate::{block::Block, signal::Signal};
use core::{f32::consts::PI, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    amplitude: f32,
    period: Duration,
    offset: f32,
}

impl Square {
    pub fn new(amplitude: f32, period: Duration, offset: f32) -> Self {
        Square {
            amplitude,
            period,
            offset,
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            period: Duration::from_secs_f32(2.0 * PI),
            offset: 0.0,
        }
    }
}

impl Block for Square {
    type Input = ();
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let t = input.delta.sim_time().as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value = if (t % period_secs) < (period_secs / 2.0) {
            self.amplitude
        } else {
            0.0
        } + self.offset;

        Signal {
            value,
            delta: input.delta,
        }
    }
}
