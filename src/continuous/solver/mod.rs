use core::time::Duration;
use faer::Mat;

pub mod euler;
pub mod runge_kutta;

pub trait StateEstimation<T> {
    fn estimate(&self, state: Mat<T>) -> Mat<T>;
}

pub trait Solver<T> {
    fn integrate(
        old_value: Mat<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> Mat<T>;
}
