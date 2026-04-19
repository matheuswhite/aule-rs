use crate::signal::Signal;

pub struct LineEquation {
    x0: f64,
    y0: f64,
    m: f64,
}

impl LineEquation {
    pub fn new(x0: f64, y0: f64, m: f64) -> Self {
        Self { x0, y0, m }
    }

    pub fn value_at(&self, x: f64) -> f64 {
        self.m * (x - self.x0) + self.y0
    }

    pub fn time_at(&self, y: f64) -> f64 {
        ((y - self.y0) / self.m) + self.x0
    }

    pub fn from_signals_with_maximum_slope(
        mut signals: impl Iterator<Item = Signal<f64>>,
    ) -> Result<Self, LineEquationError> {
        let mut x0 = 0.0;
        let mut y0 = 0.0;
        let mut max_slope = 0.0_f64;

        let mut left = signals.next().ok_or(LineEquationError::NotEnoughSignals)?;
        let mut center = signals.next().ok_or(LineEquationError::NotEnoughSignals)?;
        let mut right = signals.next().ok_or(LineEquationError::NotEnoughSignals)?;

        loop {
            let h1 = center.sim_state.sim_time().as_secs_f64() - left.sim_state.sim_time().as_secs_f64();
            let h2 = right.sim_state.sim_time().as_secs_f64() - center.sim_state.sim_time().as_secs_f64();

            let slope = (h2 / (h1 + h2)) * ((center.value - left.value) / h1)
                + (h1 / (h1 + h2)) * ((right.value - center.value) / h2);

            if slope.abs() > max_slope.abs() {
                max_slope = slope;
                x0 = center.sim_state.sim_time().as_secs_f64();
                y0 = center.value;
            }

            left = center;
            center = right;
            match signals.next() {
                Some(r) => right = r,
                None => break,
            }
        }

        Ok(Self::new(x0, y0, max_slope))
    }
}

#[derive(Debug)]
pub enum LineEquationError {
    NotEnoughSignals,
}
