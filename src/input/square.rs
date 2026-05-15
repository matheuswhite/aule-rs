use crate::{block::Block, prelude::SimulationState};
use core::{f32::consts::PI, ops::Add, time::Duration};
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Square<T> {
    amplitude: T,
    period: Duration,
    offset: T,
}

impl<T> Square<T> {
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Square {
            amplitude,
            period,
            offset,
        }
    }
}

impl<T> Default for Square<T>
where
    T: Zero + One,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            offset: T::zero(),
        }
    }
}

impl<T> Block for Square<T>
where
    T: Clone + Add<Output = T>,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let t = sim_state.sim_time().as_secs_f32();
        let period_secs = self.period.as_secs_f32();

        if (t % period_secs) < (period_secs / 2.0) {
            self.amplitude.clone() + self.offset.clone()
        } else {
            self.offset.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use nalgebra::{DMatrix, SMatrix};
    use num_complex::Complex;

    fn state_at(sim_time_s: f64, dt_s: f64) -> SimulationState {
        let mut sim = Simulation::new(dt_s as f32, (sim_time_s + dt_s * 2.0) as f32);
        let initial = sim.next().expect("simulation should yield at least one state");
        let delta = Duration::from_secs_f64(sim_time_s) - initial.sim_time();
        initial + delta
    }

    #[test]
    fn f32_high_and_low_phases() {
        let mut sq = Square::new(2.0_f32, Duration::from_secs_f64(1.0), 0.5_f32);
        // t=0.1 → low half → amplitude + offset = 2.5
        assert_eq!(sq.block((), state_at(0.1, 0.01)), 2.5);
        // t=0.6 → high half → offset = 0.5
        assert_eq!(sq.block((), state_at(0.6, 0.01)), 0.5);
    }

    #[test]
    fn f64_high_and_low_phases() {
        let mut sq = Square::new(2.0_f64, Duration::from_secs_f64(1.0), 0.5_f64);
        assert_eq!(sq.block((), state_at(0.1, 0.01)), 2.5);
        assert_eq!(sq.block((), state_at(0.6, 0.01)), 0.5);
    }

    #[test]
    fn complex_f32_high_and_low_phases() {
        let mut sq = Square::new(
            Complex::new(1.0_f32, 2.0),
            Duration::from_secs_f64(1.0),
            Complex::new(0.5_f32, 0.5),
        );
        assert_eq!(
            sq.block((), state_at(0.1, 0.01)),
            Complex::new(1.5_f32, 2.5)
        );
        assert_eq!(
            sq.block((), state_at(0.6, 0.01)),
            Complex::new(0.5_f32, 0.5)
        );
    }

    #[test]
    fn complex_f64_high_and_low_phases() {
        let mut sq = Square::new(
            Complex::new(1.0_f64, 2.0),
            Duration::from_secs_f64(1.0),
            Complex::new(0.5_f64, 0.5),
        );
        assert_eq!(
            sq.block((), state_at(0.1, 0.01)),
            Complex::new(1.5_f64, 2.5)
        );
        assert_eq!(
            sq.block((), state_at(0.6, 0.01)),
            Complex::new(0.5_f64, 0.5)
        );
    }

    #[test]
    fn dmatrix_f32_high_and_low_phases() {
        let amp = DMatrix::<f32>::from_row_slice(2, 1, &[1.0, 2.0]);
        let off = DMatrix::<f32>::from_row_slice(2, 1, &[10.0, 20.0]);
        let mut sq = Square::new(amp.clone(), Duration::from_secs_f64(1.0), off.clone());
        let high = sq.block((), state_at(0.1, 0.01));
        assert_eq!(high, DMatrix::<f32>::from_row_slice(2, 1, &[11.0, 22.0]));
        let low = sq.block((), state_at(0.6, 0.01));
        assert_eq!(low, off);
    }

    #[test]
    fn dmatrix_f64_high_and_low_phases() {
        let amp = DMatrix::<f64>::from_row_slice(2, 1, &[1.0, 2.0]);
        let off = DMatrix::<f64>::from_row_slice(2, 1, &[10.0, 20.0]);
        let mut sq = Square::new(amp.clone(), Duration::from_secs_f64(1.0), off.clone());
        let high = sq.block((), state_at(0.1, 0.01));
        assert_eq!(high, DMatrix::<f64>::from_row_slice(2, 1, &[11.0, 22.0]));
        let low = sq.block((), state_at(0.6, 0.01));
        assert_eq!(low, off);
    }

    #[test]
    fn smatrix_f32_high_and_low_phases() {
        let amp = SMatrix::<f32, 2, 1>::new(1.0, 2.0);
        let off = SMatrix::<f32, 2, 1>::new(10.0, 20.0);
        let mut sq = Square::new(amp, Duration::from_secs_f64(1.0), off);
        let high = sq.block((), state_at(0.1, 0.01));
        assert_eq!(high, SMatrix::<f32, 2, 1>::new(11.0, 22.0));
        let low = sq.block((), state_at(0.6, 0.01));
        assert_eq!(low, off);
    }

    #[test]
    fn smatrix_f64_high_and_low_phases() {
        let amp = SMatrix::<f64, 2, 1>::new(1.0, 2.0);
        let off = SMatrix::<f64, 2, 1>::new(10.0, 20.0);
        let mut sq = Square::new(amp, Duration::from_secs_f64(1.0), off);
        let high = sq.block((), state_at(0.1, 0.01));
        assert_eq!(high, SMatrix::<f64, 2, 1>::new(11.0, 22.0));
        let low = sq.block((), state_at(0.6, 0.01));
        assert_eq!(low, off);
    }
}
