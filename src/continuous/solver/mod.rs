use core::time::Duration;
use ndarray::Array2;

pub mod euler;
pub mod runge_kutta;

pub trait StateEstimation<T> {
    fn estimate(&self, state: Array2<T>) -> Array2<T>;
}

pub trait Solver<T> {
    fn integrate(
        old_value: Array2<T>,
        dt: Duration,
        state_estimation: &impl StateEstimation<T>,
    ) -> Array2<T>;
}
