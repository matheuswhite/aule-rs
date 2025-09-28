use crate::{discrete::solver::StateEstimation, prelude::Solver};
use core::time::Duration;
use ndarray::Array2;

#[derive(Debug, Clone, Copy)]
pub struct Euler;

impl Solver for Euler {
    fn integrate(
        old_value: Array2<f32>,
        dt: Duration,
        state_estimation: &impl StateEstimation,
    ) -> Array2<f32> {
        let dt_seconds = dt.as_secs_f32();
        let estimation = state_estimation.estimate(old_value.clone());
        old_value + estimation * dt_seconds
    }
}
