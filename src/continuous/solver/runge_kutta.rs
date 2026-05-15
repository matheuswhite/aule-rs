use crate::{continuous::solver::StateEstimation, prelude::Solver};
use core::{
    ops::{Add, Mul},
    time::Duration,
};
use nalgebra::{ClosedAddAssign, DMatrix, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RK4;

impl<T> Solver<T> for RK4
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T> + Scalar + ClosedAddAssign,
{
    fn integrate(
        old_value: DMatrix<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> DMatrix<T> {
        let dt_seconds = dt.as_secs_f64();
        let scale = |m: DMatrix<T>, s: f64| m.map(|v| v * s);

        let k1 = state_estimation.estimate(old_value.clone());
        let k2 = state_estimation.estimate(old_value.clone() + scale(k1.clone(), dt_seconds / 2.0));
        let k3 = state_estimation.estimate(old_value.clone() + scale(k2.clone(), dt_seconds / 2.0));
        let k4 = state_estimation.estimate(old_value.clone() + scale(k3.clone(), dt_seconds));

        let combo = k1 + scale(k2, 2.0) + scale(k3, 2.0) + k4;
        old_value + scale(combo, dt_seconds / 6.0)
    }
}
