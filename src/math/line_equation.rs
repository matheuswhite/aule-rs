use crate::{math::float_point::FloatPoint, signal::Signal};

pub struct LineEquation<T> {
    x0: T,
    y0: T,
    m: T,
}

impl<T> LineEquation<T>
where
    T: FloatPoint,
{
    pub fn new(x0: T, y0: T, m: T) -> Self {
        Self { x0, y0, m }
    }

    pub fn value_at(&self, x: T) -> T {
        self.m * (x - self.x0) + self.y0
    }

    pub fn time_at(&self, y: T) -> T {
        ((y - self.y0) / self.m) + self.x0
    }

    pub fn from_signals_with_maximum_slope(
        mut signals: impl Iterator<Item = Signal<T>>,
    ) -> Result<Self, LineEquationError> {
        let mut x0 = T::zero();
        let mut y0 = T::zero();
        let mut max_slope = T::zero();

        let mut left = signals.next().ok_or(LineEquationError::NotEnoughSignals)?;
        let mut center = signals.next().ok_or(LineEquationError::NotEnoughSignals)?;
        let mut right = signals.next().ok_or(LineEquationError::NotEnoughSignals)?;

        loop {
            let h1 = T::from_duration(center.sim_state.sim_time())
                - T::from_duration(left.sim_state.sim_time());
            let h2 = T::from_duration(right.sim_state.sim_time())
                - T::from_duration(center.sim_state.sim_time());

            let slope = (h2 / (h1 + h2)) * ((center.value - left.value) / h1)
                + (h1 / (h1 + h2)) * ((right.value - center.value) / h2);

            if slope.absolute() > max_slope.absolute() {
                max_slope = slope;
                x0 = T::from_duration(center.sim_state.sim_time());
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
