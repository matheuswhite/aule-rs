use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
    time::Duration,
};

use crate::{block::Block, signal::Signal};

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
    pub fn new(dt: f32, max_time: f32) -> Self {
        Self {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(max_time),
        }
    }

    pub fn reset(&mut self) {
        self.sim_time = Duration::default();
    }

    pub fn max_time(&self) -> Duration {
        self.max_time
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
    }
}

impl EndlessSimulation {
    pub fn new(dt: f32) -> Self {
        Self {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
        }
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
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
