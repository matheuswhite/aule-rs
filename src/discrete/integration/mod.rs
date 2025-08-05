use ndarray::Array2;
use std::time::Duration;

pub mod euler;
pub mod runge_kutta;

/// Trait to estimate the change in state based on the current state.
/// This trait defines the interface for state estimation methods that can be used in conjunction with integrators.
/// It provides a method to estimate the new state of the system based on the current state and a time step.
///
/// # Example:
/// ```
/// use aule::prelude::*;
/// use ndarray::Array2;
/// use std::time::Duration;
///
/// struct MyStateEstimation;
///
/// impl StateEstimation for MyStateEstimation {
///     fn estimate(&self, dt: f32, state: Array2<f32>) -> Array2<f32> {
///         // Implement the state estimation logic here
///         state * dt // Example: simple scaling by dt
///     }
/// }
/// ```
pub trait StateEstimation {
    fn estimate(&self, dt: f32, state: Array2<f32>) -> Array2<f32>;
}

/// Trait to solve EDOs (Ordinary Differential Equations) integrating them.
/// This trait defines the interface for integrators that can be used to solve EDOs.
/// It provides a method to integrate the state of the system over a given time step.
///
/// # Example:
/// ```
/// use aule::prelude::*;
/// use ndarray::Array2;
/// use std::time::Duration;
///
/// struct MyIntegrator;
///
/// impl Integrator for MyIntegrator {
///     fn integrate(old_value: Array2<f32>, dt: Duration, state_estimation: &impl StateEstimation) -> Array2<f32> {
///         // Implement the integration logic here
///         let mut new_value = old_value.clone();
///         for k in 0..old_value.len() {
///             new_value[[k, 0]] += state_estimation.estimate(1.0, old_value.clone())[[k, 0]] * dt.as_secs_f32();
///         }
///         new_value
///     }
/// }
/// ```
pub trait Integrator {
    fn integrate(
        old_value: Array2<f32>,
        dt: Duration,
        state_estimation: &impl StateEstimation,
    ) -> Array2<f32>;
}
