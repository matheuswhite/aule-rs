use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    dt: Duration,
    total_time: Duration,
    max_time: Option<Duration>,
}

impl Time {
    pub fn new(dt: Duration) -> Self {
        Time {
            dt,
            total_time: Duration::default(),
            max_time: None,
        }
    }

    pub fn with_max_time(mut self, max_time: Duration) -> Self {
        self.max_time = Some(max_time);
        self
    }

    pub fn total_time(&self) -> Duration {
        self.total_time
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn max_time(&self) -> Option<Duration> {
        self.max_time.clone()
    }
}

impl From<f32> for Time {
    fn from(dt: f32) -> Self {
        Time {
            dt: Duration::from_secs_f32(dt),
            total_time: Duration::default(),
            max_time: None,
        }
    }
}

impl From<(f32, f32)> for Time {
    fn from((dt, max_time): (f32, f32)) -> Self {
        Time {
            dt: Duration::from_secs_f32(dt),
            total_time: Duration::default(),
            max_time: Some(Duration::from_secs_f32(max_time)),
        }
    }
}

impl Iterator for Time {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(max_time) = self.max_time else {
            return Some(self.dt);
        };

        if self.total_time >= max_time {
            return None;
        }

        self.total_time += self.dt;
        Some(self.dt)
    }
}
