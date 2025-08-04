use ndarray::Array2;
use std::time::Duration;

pub mod euler;
pub mod runge_kutta;

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
///     fn integrate<F>(old_value: Array2<f32>, dt: Duration, mut slop_estimation: F) -> Array2<f32>
///     where
///         F: FnMut(f32, Array2<f32>) -> Array2<f32> {
///         // Implement the integration logic here
///         let mut new_value = old_value.clone();
///         for k in 0..old_value.len() {
///             new_value[[k, 0]] += slop_estimation(1.0, old_value.clone())[[k, 0]] * dt.as_secs_f32();
///         }
///         new_value
///     }
/// }
/// ```
pub trait Integrator {
    fn integrate<F>(old_value: Array2<f32>, dt: Duration, slop_estimation: F) -> Array2<f32>
    where
        F: FnMut(f32, Array2<f32>) -> Array2<f32>;
}
