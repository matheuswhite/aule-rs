use crate::{block::Block, math::float_point::FloatPoint, signal::Signal};
use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationState {
    dt: Duration,
    sim_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Simulation {
    dt: Duration,
    sim_time: Duration,
    max_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndlessSimulation {
    dt: Duration,
    sim_time: Duration,
}

impl Simulation {
    pub fn new<T, U>(dt: T, max_time: U) -> Self
    where
        T: FloatPoint,
        U: FloatPoint,
    {
        Self {
            dt: dt.to_duration(),
            sim_time: Duration::default(),
            max_time: max_time.to_duration(),
        }
    }

    pub fn reset(&mut self) {
        self.sim_time = Duration::default();
    }

    pub fn max_time(&self) -> Duration {
        self.max_time
    }

    pub fn set_dt<T>(&mut self, dt: T)
    where
        T: FloatPoint,
    {
        self.dt = dt.to_duration();
    }
}

impl EndlessSimulation {
    pub fn new<T>(dt: T) -> Self
    where
        T: FloatPoint,
    {
        Self {
            dt: dt.to_duration(),
            sim_time: Duration::default(),
        }
    }

    pub fn set_dt<T>(&mut self, dt: T)
    where
        T: FloatPoint,
    {
        self.dt = dt.to_duration();
    }
}

impl SimulationState {
    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn sim_time(&self) -> Duration {
        self.sim_time
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            dt: self.dt.min(other.dt),
            sim_time: self.sim_time.min(other.sim_time),
        }
    }

    pub fn reset_sim_time(&mut self) {
        self.sim_time = Duration::default();
    }

    pub fn reset_dt(&mut self) {
        self.dt = Duration::default();
    }
}

impl<O> Mul<&mut dyn Block<Input = (), Output = O>> for SimulationState {
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = (), Output = O>) -> Self::Output {
        let output = block.block((), self);
        Signal {
            value: output,
            sim_state: self,
        }
    }
}

impl Add<(Duration, Duration)> for SimulationState {
    type Output = Self;

    fn add(self, rhs: (Duration, Duration)) -> Self::Output {
        Self {
            dt: self.dt + rhs.0,
            sim_time: self.sim_time + rhs.1,
        }
    }
}

impl AddAssign<(Duration, Duration)> for SimulationState {
    fn add_assign(&mut self, rhs: (Duration, Duration)) {
        self.dt += rhs.0;
        self.sim_time += rhs.1;
    }
}

impl Add<Duration> for SimulationState {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            dt: self.dt,
            sim_time: self.sim_time + rhs,
        }
    }
}

impl AddAssign<Duration> for SimulationState {
    fn add_assign(&mut self, rhs: Duration) {
        self.sim_time += rhs;
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(10.0),
        }
    }
}

impl Default for EndlessSimulation {
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
        }
    }
}

impl Iterator for Simulation {
    type Item = SimulationState;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim_time += self.dt;

        if self.sim_time <= self.max_time {
            Some(SimulationState {
                dt: self.dt,
                sim_time: self.sim_time,
            })
        } else {
            None
        }
    }
}

impl Iterator for EndlessSimulation {
    type Item = SimulationState;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim_time += self.dt;

        Some(SimulationState {
            dt: self.dt,
            sim_time: self.sim_time,
        })
    }
}

impl From<Duration> for EndlessSimulation {
    fn from(dt: Duration) -> Self {
        Self {
            dt,
            sim_time: Duration::default(),
        }
    }
}

impl From<(Duration, Duration)> for Simulation {
    fn from(value: (Duration, Duration)) -> Self {
        Self {
            dt: value.0,
            sim_time: Duration::default(),
            max_time: value.1,
        }
    }
}
