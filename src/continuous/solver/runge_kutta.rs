use crate::{continuous::solver::StateEstimation, prelude::Solver};
use core::time::Duration;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RK4;

impl Solver for RK4 {
    fn integrate(
        old_value: Array2<f32>,
        dt: Duration,
        state_estimation: &impl StateEstimation,
    ) -> Array2<f32> {
        let dt_seconds = dt.as_secs_f32();
        let k1 = state_estimation.estimate(old_value.clone());
        let k2 = state_estimation.estimate(old_value.clone() + k1.clone() * (dt_seconds / 2.0));
        let k3 = state_estimation.estimate(old_value.clone() + k2.clone() * (dt_seconds / 2.0));
        let k4 = state_estimation.estimate(old_value.clone() + k3.clone() * dt_seconds);

        old_value + (k1 + 2.0 * k2 + 2.0 * k3 + k4) * (dt_seconds / 6.0)
    }
}
