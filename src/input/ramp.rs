use crate::{input::Input, signal::Signal};
use core::{ops::Mul, time::Duration};

pub struct Ramp<T> {
    value: T,
    sim_time: Duration,
}

impl<T> Ramp<T> {
    pub fn new(value: T) -> Self {
        Ramp {
            value,
            sim_time: Duration::default(),
        }
    }
}

impl<T: Mul<f32, Output = T> + Clone> Input for Ramp<T> {
    type Output = T;

    fn output(&mut self, dt: Duration) -> Signal<Self::Output> {
        self.sim_time += dt;
        let value = self.value.clone() * self.sim_time.as_secs_f32();
        Signal { value, dt }
    }
}
