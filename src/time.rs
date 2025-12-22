use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign},
    time::Duration,
};

use crate::signal::Signal;

pub trait TimeType: Debug + Clone + Copy + PartialEq {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Continuous;
impl TimeType for Continuous {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Discrete;
impl TimeType for Discrete {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Delta<K>
where
    K: TimeType,
{
    dt: Duration,
    sim_time: Duration,
    _marker: PhantomData<K>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time<K>
where
    K: TimeType,
{
    dt: Duration,
    sim_time: Duration,
    max_time: Duration,
    _marker: PhantomData<K>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndlessTime<K>
where
    K: TimeType,
{
    dt: Duration,
    sim_time: Duration,
    _marker: PhantomData<K>,
}

impl Time<Continuous> {
    pub fn continuous(dt: f32, max_time: f32) -> Self {
        Self {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(max_time),
            _marker: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        self.sim_time = Duration::default();
    }
}

impl Time<Discrete> {
    pub fn discrete(dt: f32, max_time: f32) -> Self {
        Self {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(max_time),
            _marker: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        self.sim_time = Duration::default();
    }
}

impl<K> Time<K>
where
    K: TimeType,
{
    pub fn max_time(&self) -> Duration {
        self.max_time
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
    }
}

impl EndlessTime<Continuous> {
    pub fn continuous(dt: f32) -> Self {
        Self {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            _marker: PhantomData,
        }
    }
}

impl EndlessTime<Discrete> {
    pub fn discrete(dt: f32) -> Self {
        EndlessTime {
            dt: Duration::from_secs_f32(dt),
            sim_time: Duration::default(),
            _marker: PhantomData,
        }
    }
}

impl<K> EndlessTime<K>
where
    K: TimeType,
{
    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
    }
}

impl<K> Delta<K>
where
    K: TimeType,
{
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
            _marker: PhantomData,
        }
    }

    pub fn reset_sim_time(&mut self) {
        self.sim_time = Duration::default();
    }

    pub fn reset_dt(&mut self) {
        self.dt = Duration::default();
    }
}

impl<K> Add<(Duration, Duration)> for Delta<K>
where
    K: TimeType,
{
    type Output = Self;

    fn add(self, rhs: (Duration, Duration)) -> Self::Output {
        Self {
            dt: self.dt + rhs.0,
            sim_time: self.sim_time + rhs.1,
            _marker: PhantomData,
        }
    }
}

impl<K> AddAssign<(Duration, Duration)> for Delta<K>
where
    K: TimeType,
{
    fn add_assign(&mut self, rhs: (Duration, Duration)) {
        self.dt += rhs.0;
        self.sim_time += rhs.1;
    }
}

impl<K> Add<Duration> for Delta<K>
where
    K: TimeType,
{
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            dt: self.dt,
            sim_time: self.sim_time + rhs,
            _marker: PhantomData,
        }
    }
}

impl<K> AddAssign<Duration> for Delta<K>
where
    K: TimeType,
{
    fn add_assign(&mut self, rhs: Duration) {
        self.sim_time += rhs;
    }
}

impl<K> Default for Time<K>
where
    K: TimeType,
{
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
            max_time: Duration::from_secs_f32(10.0),
            _marker: PhantomData,
        }
    }
}

impl<K> Default for EndlessTime<K>
where
    K: TimeType,
{
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
            _marker: PhantomData,
        }
    }
}

impl<K> From<(Duration, K)> for EndlessTime<K>
where
    K: TimeType,
{
    fn from((dt, _time_type): (Duration, K)) -> Self {
        Self {
            dt,
            sim_time: Duration::default(),
            _marker: PhantomData,
        }
    }
}

impl<K> From<(Duration, Duration, K)> for Time<K>
where
    K: TimeType,
{
    fn from((dt, max_time, _time_type): (Duration, Duration, K)) -> Self {
        Self {
            dt,
            sim_time: Duration::default(),
            max_time,
            _marker: PhantomData,
        }
    }
}

impl<K> Iterator for Time<K>
where
    K: TimeType,
{
    type Item = Signal<(), K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim_time += self.dt;

        if self.sim_time <= self.max_time {
            Some(Signal {
                value: (),
                delta: Delta {
                    dt: self.dt,
                    sim_time: self.sim_time,
                    _marker: PhantomData,
                },
            })
        } else {
            None
        }
    }
}

impl<K> Iterator for EndlessTime<K>
where
    K: TimeType,
{
    type Item = Signal<(), K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sim_time += self.dt;

        Some(Signal {
            value: (),
            delta: Delta {
                dt: self.dt,
                sim_time: self.sim_time,
                _marker: PhantomData,
            },
        })
    }
}
