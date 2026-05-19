use crate::{
    continuous::solver::StateEstimation,
    math::{float_point::FloatPoint, number::Number},
    prelude::Solver,
};
use core::time::Duration;
use nalgebra::{DMatrix, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RK4;

impl<T> Solver<T> for RK4
where
    T: Number + Scalar,
{
    fn integrate(
        old_value: DMatrix<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> DMatrix<T> {
        let dt = <T::Alpha as FloatPoint>::from_duration(dt);
        let scale = |m: DMatrix<T>, s: T::Alpha| m.map(|v| v.scale(s));
        let two = <T::Alpha as FloatPoint>::from_usize(2);
        let six = <T::Alpha as FloatPoint>::from_usize(6);

        let k1 = state_estimation.estimate(old_value.clone());
        let k2 = state_estimation.estimate(old_value.clone() + scale(k1.clone(), dt / two));
        let k3 = state_estimation.estimate(old_value.clone() + scale(k2.clone(), dt / two));
        let k4 = state_estimation.estimate(old_value.clone() + scale(k3.clone(), dt));

        let combo = k1 + scale(k2, two) + scale(k3, two) + k4;
        old_value + scale(combo, dt / six)
    }
}
