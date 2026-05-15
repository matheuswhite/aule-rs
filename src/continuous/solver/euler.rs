use crate::{continuous::solver::StateEstimation, prelude::Solver};
use core::{
    ops::{Add, Mul},
    time::Duration,
};
use nalgebra::{ClosedAddAssign, DMatrix, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Euler;

impl<T> Solver<T> for Euler
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T> + Scalar + ClosedAddAssign,
{
    fn integrate(
        old_value: DMatrix<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> DMatrix<T> {
        let dt_seconds = dt.as_secs_f64();
        let estimation = state_estimation.estimate(old_value.clone());
        old_value + estimation.map(|v| v * dt_seconds)
    }
}
