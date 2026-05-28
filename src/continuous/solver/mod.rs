use core::time::Duration;
use nalgebra::DMatrix;

pub mod euler;
pub mod runge_kutta;

pub trait StateEstimation<T> {
    fn estimate(&self, state: DMatrix<T>) -> DMatrix<T>;
}

pub trait Solver<T> {
    fn integrate(
        old_value: DMatrix<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> DMatrix<T>;
}
