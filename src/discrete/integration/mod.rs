use core::time::Duration;
use ndarray::Array2;

pub mod euler;
pub mod runge_kutta;

pub trait StateEstimation {
    fn estimate(&self, state: Array2<f32>) -> Array2<f32>;
}

pub trait Integrator {
    fn integrate(
        old_value: Array2<f32>,
        dt: Duration,
        state_estimation: &impl StateEstimation,
    ) -> Array2<f32>;
}
