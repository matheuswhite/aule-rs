use core::time::Duration;
#[cfg(feature = "std")]
use std::thread::sleep;
#[cfg(feature = "std")]
use std::time::Instant;

/// Holds the time step and the total time elapsed.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// let time = Time::from(0.01);
///
/// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
/// assert_eq!(time.total_time(), Duration::default());
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    dt: Duration,
    total_time: Duration,
    max_time: Option<Duration>,
}

/// Real-time capable time iterator.
/// It will sleep the current thread to maintain real-time execution.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// let mut time = RTTime::new(Duration::from_secs_f32(0.01));
/// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
/// assert_eq!(time.total_time(), Duration::from_secs_f32(0.01));
/// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
/// assert_eq!(time.total_time(), Duration::from_secs_f32(0.02));
/// ```
#[cfg(feature = "std")]
pub struct RTTime {
    dt: Duration,
    total_time: Duration,
    max_time: Option<Duration>,
    last_instant: Instant,
}

impl Time {
    /// Creates a new `Time` instance with the specified time step.
    ///
    /// # Parameters
    /// - `dt`: The time step duration.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let dt = Duration::from_secs_f32(0.01);
    /// let time = Time::new(dt);
    ///
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.total_time(), Duration::default());
    /// ```
    pub fn new(dt: Duration) -> Self {
        Time {
            dt,
            total_time: Duration::default(),
            max_time: None,
        }
    }

    /// Sets the maximum time limit for iteration.
    ///
    /// # Parameters
    /// - `max_time`: The maximum time duration for the simulation.
    ///
    /// # Returns
    /// A new `Time` instance with the specified maximum time.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let dt = Duration::from_secs_f32(0.01);
    /// let max_time = Duration::from_secs_f32(1.0);
    /// let time = Time::new(dt).with_max_time(max_time);
    ///
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.max_time(), Some(Duration::from_secs_f32(1.0)));
    /// ```
    pub fn with_max_time(mut self, max_time: Duration) -> Self {
        self.max_time = Some(max_time);
        self
    }

    /// Returns the total time elapsed.
    ///
    /// # Returns
    /// The total time as a `Duration`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = Time::from(0.01);
    /// assert_eq!(time.total_time(), Duration::default());
    /// ```
    pub fn total_time(&self) -> Duration {
        self.total_time
    }

    /// Returns the time step duration.
    ///
    /// # Returns
    /// The time step as a `Duration`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = Time::from(0.01);
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// ```
    pub fn dt(&self) -> Duration {
        self.dt
    }

    /// Returns the maximum time limit for iteration, if set.
    ///
    /// # Returns
    /// An `Option<Duration>` representing the maximum time.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = Time::from((0.01, 1.0));
    /// assert_eq!(time.max_time(), Some(Duration::from_secs_f32(1.0)));
    /// ```
    pub fn max_time(&self) -> Option<Duration> {
        self.max_time.clone()
    }
}

#[cfg(feature = "std")]
impl RTTime {
    /// Creates a new `RTTime` instance with the specified time step.
    ///
    /// The iterator will attempt to maintain real-time execution by sleeping the current thread.
    ///
    /// # Parameters
    /// - `dt`: The time step duration.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let dt = Duration::from_secs_f32(0.01);
    /// let time = RTTime::new(dt);
    ///
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.total_time(), Duration::default());
    /// ```
    pub fn new(dt: Duration) -> Self {
        RTTime {
            dt,
            total_time: Duration::default(),
            max_time: None,
            last_instant: Instant::now(),
        }
    }

    /// Sets the maximum time limit for iteration.
    ///
    /// # Parameters
    /// - `max_time`: The maximum time duration for the simulation.
    ///
    /// # Returns
    /// A new `RTTime` instance with the specified maximum time.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let dt = Duration::from_secs_f32(0.01);
    /// let max_time = Duration::from_secs_f32(1.0);
    /// let time = RTTime::new(dt).with_max_time(max_time);
    ///
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.max_time(), Some(Duration::from_secs_f32(1.0)));
    /// ```
    pub fn with_max_time(mut self, max_time: Duration) -> Self {
        self.max_time = Some(max_time);
        self
    }

    /// Returns the total time elapsed.
    ///
    /// # Returns
    /// The total time as a `Duration`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = RTTime::new(Duration::from_secs_f32(0.01));
    /// assert_eq!(time.total_time(), Duration::default());
    /// ```
    pub fn total_time(&self) -> Duration {
        self.total_time
    }

    /// Returns the time step duration.
    ///
    /// # Returns
    /// The time step as a `Duration`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = RTTime::new(Duration::from_secs_f32(0.01));
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// ```
    pub fn dt(&self) -> Duration {
        self.dt
    }

    /// Returns the maximum time limit for iteration, if set.
    ///
    /// # Returns
    /// An `Option<Duration>` representing the maximum time.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = RTTime::new(Duration::from_secs_f32(0.01)).with_max_time(Duration::from_secs_f32(1.0));
    /// assert_eq!(time.max_time(), Some(Duration::from_secs_f32(1.0)));
    /// ```
    pub fn max_time(&self) -> Option<Duration> {
        self.max_time.clone()
    }

    fn wait_dt(&mut self) {
        let elapsed = self.last_instant.elapsed();
        if elapsed < self.dt {
            sleep(self.dt - elapsed);
        }

        self.last_instant = Instant::now();
    }
}

impl From<f32> for Time {
    /// Creates a `Time` instance from a float representing seconds.
    ///
    /// # Parameters
    /// - `dt`: The time step in seconds.
    ///
    /// # Returns
    /// A new `Time` instance with the specified time step.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = Time::from(0.01);
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.total_time(), Duration::default());
    /// ```
    fn from(dt: f32) -> Self {
        Time {
            dt: Duration::from_secs_f32(dt),
            total_time: Duration::default(),
            max_time: None,
        }
    }
}

#[cfg(feature = "std")]
impl From<f32> for RTTime {
    /// Creates a `RTTime` instance from a float representing seconds.
    ///
    /// # Parameters
    /// - `dt`: The time step in seconds.
    ///
    /// # Returns
    /// A new `RTTime` instance with the specified time step.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = RTTime::from(0.01);
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.total_time(), Duration::default());
    /// ```
    fn from(dt: f32) -> Self {
        RTTime {
            dt: Duration::from_secs_f32(dt),
            total_time: Duration::default(),
            max_time: None,
            last_instant: Instant::now(),
        }
    }
}

impl From<(f32, f32)> for Time {
    /// Creates a `Time` instance from a tuple of two floats representing seconds.
    ///
    /// # Parameters
    /// - `(dt, max_time)`: A tuple where `dt` is the time step and `max_time` is the maximum time limit.
    ///
    /// # Returns
    /// A new `Time` instance with the specified time step and maximum time.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = Time::from((0.01, 1.0));
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.max_time(), Some(Duration::from_secs_f32(1.0)));
    /// ```
    fn from((dt, max_time): (f32, f32)) -> Self {
        Time {
            dt: Duration::from_secs_f32(dt),
            total_time: Duration::default(),
            max_time: Some(Duration::from_secs_f32(max_time)),
        }
    }
}

#[cfg(feature = "std")]
impl From<(f32, f32)> for RTTime {
    /// Creates a `RTTime` instance from a tuple of two floats representing seconds.
    ///
    /// # Parameters
    /// - `(dt, max_time)`: A tuple where `dt` is the time step and `max_time` is the maximum time limit.
    ///
    /// # Returns
    /// A new `RTTime` instance with the specified time step and maximum time.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let time = RTTime::from((0.01, 1.0));
    /// assert_eq!(time.dt(), Duration::from_secs_f32(0.01));
    /// assert_eq!(time.max_time(), Some(Duration::from_secs_f32(1.0)));
    /// ```
    fn from((dt, max_time): (f32, f32)) -> Self {
        RTTime {
            dt: Duration::from_secs_f32(dt),
            total_time: Duration::default(),
            max_time: Some(Duration::from_secs_f32(max_time)),
            last_instant: Instant::now(),
        }
    }
}

impl Iterator for Time {
    type Item = Duration;

    /// Advances the time by the specified time step and returns the new time.
    ///
    /// If the total time exceeds the maximum time, it returns `None`.
    ///
    /// # Returns
    /// An `Option<Duration>` representing the new time step, or `None` if the maximum time has been reached.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let max_time = Duration::from_secs_f32(0.1);
    /// let mut time = Time::from(0.01).with_max_time(max_time);
    /// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
    /// assert_eq!(time.total_time(), Duration::from_secs_f32(0.01));
    ///
    /// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
    /// assert_eq!(time.total_time(), Duration::from_secs_f32(0.02));
    /// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
    /// assert_eq!(time.total_time(), Duration::from_millis(30));
    fn next(&mut self) -> Option<Self::Item> {
        self.total_time += self.dt;

        match self.max_time {
            Some(max_time) if self.total_time > max_time => None,
            _ => Some(self.dt),
        }
    }
}

#[cfg(feature = "std")]
impl Iterator for RTTime {
    type Item = Duration;

    /// Advances the time by the specified time step, sleeps to maintain real-time execution,
    /// and returns the new time.
    ///
    /// If the total time exceeds the maximum time, it returns `None`.
    ///
    /// # Returns
    /// An `Option<Duration>` representing the new time step, or `None` if the maximum time has been reached.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let max_time = Duration::from_secs_f32(0.1);
    /// let mut time = RTTime::new(Duration::from_secs_f32(0.01)).with_max_time(max_time);
    /// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
    /// assert_eq!(time.total_time(), Duration::from_secs_f32(0.01));
    ///
    /// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
    /// assert_eq!(time.total_time(), Duration::from_secs_f32(0.02));
    /// assert_eq!(time.next(), Some(Duration::from_secs_f32(0.01)));
    /// assert_eq!(time.total_time(), Duration::from_millis(30));
    fn next(&mut self) -> Option<Self::Item> {
        self.total_time += self.dt;

        match self.max_time {
            Some(max_time) if self.total_time > max_time => None,
            _ => {
                self.wait_dt();
                Some(self.dt)
            }
        }
    }
}
