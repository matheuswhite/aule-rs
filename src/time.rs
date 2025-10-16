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
pub struct Delta<T>
where
    T: TimeType,
{
    dt: Duration,
    sim_time: Duration,
    _marker: PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time<T>
where
    T: TimeType,
{
    dt: Duration,
    sim_time: Duration,
    max_time: Duration,
    _marker: PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndlessTime<T>
where
    T: TimeType,
{
    dt: Duration,
    sim_time: Duration,
    _marker: PhantomData<T>,
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

impl<T> Time<T>
where
    T: TimeType,
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

impl<T> EndlessTime<T>
where
    T: TimeType,
{
    pub fn set_dt(&mut self, dt: f32) {
        self.dt = Duration::from_secs_f32(dt);
    }
}

impl<T> Delta<T>
where
    T: TimeType,
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

impl<T> Add<(Duration, Duration)> for Delta<T>
where
    T: TimeType,
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

impl<T> AddAssign<(Duration, Duration)> for Delta<T>
where
    T: TimeType,
{
    fn add_assign(&mut self, rhs: (Duration, Duration)) {
        self.dt += rhs.0;
        self.sim_time += rhs.1;
    }
}

impl<T> Add<Duration> for Delta<T>
where
    T: TimeType,
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

impl<T> AddAssign<Duration> for Delta<T>
where
    T: TimeType,
{
    fn add_assign(&mut self, rhs: Duration) {
        self.sim_time += rhs;
    }
}

impl<T> Default for Time<T>
where
    T: TimeType,
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

impl<T> Default for EndlessTime<T>
where
    T: TimeType,
{
    fn default() -> Self {
        Self {
            dt: Duration::from_secs_f32(1e-3),
            sim_time: Duration::default(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<(Duration, T)> for EndlessTime<T>
where
    T: TimeType,
{
    fn from((dt, _time_type): (Duration, T)) -> Self {
        Self {
            dt,
            sim_time: Duration::default(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<(Duration, Duration, T)> for Time<T>
where
    T: TimeType,
{
    fn from((dt, max_time, _time_type): (Duration, Duration, T)) -> Self {
        Self {
            dt,
            sim_time: Duration::default(),
            max_time,
            _marker: PhantomData,
        }
    }
}

impl<T> Iterator for Time<T>
where
    T: TimeType,
{
    type Item = Signal<(), T>;

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

impl<T> Iterator for EndlessTime<T>
where
    T: TimeType,
{
    type Item = Signal<(), T>;

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
