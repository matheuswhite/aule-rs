use crate::{
    continuous::solver::StateEstimation,
    math::{float_point::FloatPoint, number::Number},
    prelude::Solver,
};
use core::time::Duration;
use nalgebra::{DMatrix, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Euler;

impl<T> Solver<T> for Euler
where
    T: Number + Scalar,
{
    fn integrate(
        old_value: DMatrix<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> DMatrix<T> {
        let dt = <T::Alpha as FloatPoint>::from_duration(dt);
        let estimation = state_estimation.estimate(old_value.clone());
        old_value + estimation.map(|v| v.scale(dt))
    }
}
