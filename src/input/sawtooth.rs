use crate::{input::Input, signal::Signal};
use core::{
    ops::{Add, Div, Mul},
    time::Duration,
};

pub struct Sawtooth<T> {
    amplitude: T,
    period: Duration,
    offset: T,
    sim_time: Duration,
}

impl<T> Sawtooth<T> {
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
            sim_time: Duration::default(),
        }
    }
}

impl<T> Input for Sawtooth<T>
where
    T: Div<f32, Output = T> + Mul<f32, Output = T> + Add<T, Output = T> + Clone,
{
    type Output = T;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        self.sim_time += dt;

        let t = self.sim_time.as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        let value =
            (self.amplitude.clone() / period_secs) * (t % period_secs) + self.offset.clone();

        Signal { value, dt }
    }
}
