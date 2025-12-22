use crate::{continuous::solver::StateEstimation, prelude::Solver};
use core::{
    ops::{Add, Mul},
    time::Duration,
};
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Euler;

impl<T> Solver<T> for Euler
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T>,
{
    fn integrate(
        old_value: Array2<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> Array2<T> {
        let dt_seconds = dt.as_secs_f64();
        let estimation = state_estimation.estimate(old_value.clone());
        old_value + estimation * dt_seconds
    }
}
