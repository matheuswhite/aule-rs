use core::time::Duration;
use std::ops::{Mul, Shr, Sub};

pub trait Block {
    fn output(&mut self, input: Signal) -> Signal;
    fn last_output(&self) -> Option<Signal>;
}

pub trait Monitor {
    fn show(&mut self, inputs: Signal);
}

pub trait Input {
    fn output(&mut self, dt: Duration) -> Signal;
}

pub trait AsBlock: Sized + Block + 'static {
    fn as_block(&mut self) -> &mut dyn Block {
        self
    }
}

pub trait AsMonitor: Sized + Monitor + 'static {
    fn as_monitor(&mut self) -> &mut dyn Monitor {
        self
    }
}

pub trait AsInput: Sized + Input + 'static {
    fn as_input(&mut self) -> &mut dyn Input {
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Signal {
    pub value: f32,
    pub dt: Duration,
}

impl From<Duration> for Signal {
    fn from(dt: Duration) -> Self {
        Signal { value: 0.0, dt }
    }
}

impl Mul<f32> for Signal {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value * rhs,
            dt: self.dt,
        }
    }
}

impl Sub for Signal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            dt: self.dt,
        }
    }
}

impl Sub<f32> for Signal {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value - rhs,
            dt: self.dt,
        }
    }
}

impl Sub<Option<Signal>> for Signal {
    type Output = Self;

    fn sub(self, rhs: Option<Signal>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl Mul<&mut Box<dyn Block>> for Signal {
    type Output = Signal;

    fn mul(self, block: &mut Box<dyn Block>) -> Self::Output {
        block.output(self)
    }
}

impl Mul<&mut dyn Block> for Signal {
    type Output = Signal;

    fn mul(self, block: &mut dyn Block) -> Self::Output {
        block.output(self)
    }
}

impl Mul<Signal> for &mut Box<dyn Block> {
    type Output = Signal;

    fn mul(self, input: Signal) -> Self::Output {
        self.output(input)
    }
}

impl Shr<&mut dyn Monitor> for Signal {
    type Output = Signal;

    fn shr(self, monitor: &mut dyn Monitor) -> Self::Output {
        monitor.show(self);
        self
    }
}

impl Shr<&mut dyn Input> for Duration {
    type Output = Signal;

    fn shr(self, rhs: &mut dyn Input) -> Self::Output {
        rhs.output(self)
    }
}
