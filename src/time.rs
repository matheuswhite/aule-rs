use core::{
    ops::{Add, AddAssign},
    time::Duration,
};

use crate::signal::Signal;

#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub enum TimeType {
    #[default]
    Continuous,
    Discrete,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Delta {
    dt: Duration,
    sim_time: Duration,
    time_type: TimeType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    dt: Duration,
    sim_time: Duration,
    max_time: Duration,
    time_type: TimeType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndlessTime {
    dt: Duration,
    sim_time: Duration,
    time_type: TimeType,
}

impl Time {
    pub fn continuous(dt: f32, max_time: f32) -> Self {
        Time {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(max_time),
            time_type: TimeType::Continuous,
        }
    }

    pub fn discrete(dt: f32, max_time: f32) -> Self {
        Time {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(max_time),
            time_type: TimeType::Discrete,
        }
    }

    pub fn max_time(&self) -> Duration {
        self.max_time
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
    }
}

impl EndlessTime {
    pub fn continuous(dt: f32) -> Self {
        EndlessTime {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            time_type: TimeType::Continuous,
        }
    }

    pub fn discrete(dt: f32) -> Self {
        EndlessTime {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            time_type: TimeType::Discrete,
        }
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
    }
}

impl Delta {
    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn sim_time(&self) -> Duration {
        self.sim_time
    }

    pub fn time_type(&self) -> TimeType {
        self.time_type
    }

    pub fn merge(self, other: Self) -> Self {
        assert!(
            self.time_type == other.time_type,
            "Cannot merge Deltas of different TimeTypes"
        );

        Self {
            dt: self.dt.min(other.dt),
            sim_time: self.sim_time.min(other.sim_time),
            time_type: self.time_type,
        }
    }

    pub fn reset_sim_time(&mut self) {
        self.sim_time = Duration::default();
    }

    pub fn reset_dt(&mut self) {
        self.dt = Duration::default();
    }
}

impl Add<(Duration, Duration)> for Delta {
    type Output = Delta;

    fn add(self, rhs: (Duration, Duration)) -> Self::Output {
        Self {
            dt: self.dt + rhs.0,
            sim_time: self.sim_time + rhs.1,
            time_type: self.time_type,
        }
    }
}

impl AddAssign<(Duration, Duration)> for Delta {
    fn add_assign(&mut self, rhs: (Duration, Duration)) {
        self.dt += rhs.0;
        self.sim_time += rhs.1;
    }
}

impl Add<Duration> for Delta {
    type Output = Delta;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            dt: self.dt,
            sim_time: self.sim_time + rhs,
            time_type: self.time_type,
        }
    }
}

impl AddAssign<Duration> for Delta {
    fn add_assign(&mut self, rhs: Duration) {
        self.sim_time += rhs;
    }
}

impl TimeType {
    pub fn is_continuous(&self) -> bool {
        matches!(self, TimeType::Continuous)
    }

    pub fn is_discrete(&self) -> bool {
        matches!(self, TimeType::Discrete)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(10.0),
            time_type: TimeType::Continuous,
        }
    }
}

impl Default for EndlessTime {
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
            time_type: TimeType::Continuous,
        }
    }
}

impl From<(Duration, TimeType)> for EndlessTime {
    fn from((dt, time_type): (Duration, TimeType)) -> Self {
        Self {
            dt,
            sim_time: Duration::default(),
            time_type,
        }
    }
}

impl From<(Duration, Duration, TimeType)> for Time {
    fn from((dt, max_time, time_type): (Duration, Duration, TimeType)) -> Self {
        Time {
            dt,
            sim_time: Duration::default(),
            max_time,
            time_type,
        }
    }
}

impl Iterator for Time {
    type Item = Signal<()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim_time += self.dt;

        if self.sim_time <= self.max_time {
            Some(Signal {
                value: (),
                delta: Delta {
                    dt: self.dt,
                    sim_time: self.sim_time,
                    time_type: self.time_type,
                },
            })
        } else {
            None
        }
    }
}

impl Iterator for EndlessTime {
    type Item = Signal<()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim_time += self.dt;

        Some(Signal {
            value: (),
            delta: Delta {
                dt: self.dt,
                sim_time: self.sim_time,
                time_type: self.time_type,
            },
        })
    }
}
