use crate::{continuous::solver::StateEstimation, prelude::Solver};
use core::{
    ops::{Add, Mul},
    time::Duration,
};
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RK4;

impl<T> Solver<T> for RK4
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T>,
{
    fn integrate(
        old_value: Array2<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> Array2<T> {
        let dt_seconds = dt.as_secs_f64();
        let k1 = state_estimation.estimate(old_value.clone());
        let k2 = state_estimation.estimate(old_value.clone() + k1.clone() * (dt_seconds / 2.0));
        let k3 = state_estimation.estimate(old_value.clone() + k2.clone() * (dt_seconds / 2.0));
        let k4 = state_estimation.estimate(old_value.clone() + k3.clone() * dt_seconds);

        old_value + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt_seconds / 6.0)
    }
}
