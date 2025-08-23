use crate::{discrete::integration::StateEstimation, prelude::Integrator};
use core::time::Duration;
use ndarray::Array2;

/// Euler integration method for solving ordinary differential equations (ODEs).
///
/// This struct implements the `Integrator` trait, providing a method to integrate
/// the state of a system using the Euler method.
/// The Euler method is a simple numerical procedure for solving ordinary differential equations
/// with a given initial value and a time step.
///
/// x[k] = x[k-1] + f(x[k-1]) * dt
///
/// # Example:
/// ```
/// use aule::prelude::*;
/// use ndarray::Array2;
/// use std::time::Duration;
///
/// let old_value = Array2::from_shape_vec((3, 1), vec![1.0, 2.0, 3.0]).unwrap();
/// let dt = Duration::from_secs(1);
/// let state_estimation: SS<Euler> = SS::new(
///     Array2::from_shape_vec((3, 3), vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, -2.0, -3.0, 0.0]).unwrap(),
///     vec![0.0, 0.0, 1.0],
///     vec![1.0, 0.0, 0.0],
///     0.0,
/// );
/// let new_value = Euler::integrate(old_value, dt, &state_estimation);
/// assert_eq!(new_value, Array2::from_shape_vec((3, 1), vec![3.0, 5.0, -5.0]).unwrap());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Euler;

impl Integrator for Euler {
    /// Integrates the old value using the Euler method.
    /// Takes the old value, a time duration, and a slot estimation function.
    /// Returns the new value after integration.
    ///
    /// # Arguments:
    /// * `old_value` - The previous state of the system as an `Array2<f32>`.
    /// * `dt` - The time step as a `Duration`.
    /// * `slop_estimation` - A closure that estimates the change in state based on the current state.
    ///
    /// # Returns:
    /// An `Array2<f32>` representing the new state of the system after integration.
    ///
    /// # Example:
    /// ```
    /// use aule::prelude::*;
    /// use ndarray::Array2;
    /// use std::time::Duration;
    ///
    /// let old_value = Array2::from_shape_vec((3, 1), vec![1.0, 2.0, 3.0]).unwrap();
    /// let dt = Duration::from_secs(1);
    /// let state_estimation: SS<Euler> = SS::new(
    ///     Array2::from_shape_vec((3, 3), vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, -2.0, -3.0, 0.0]).unwrap(),
    ///     vec![0.0, 0.0, 1.0],
    ///     vec![1.0, 0.0, 0.0],
    ///     0.0,
    /// );
    /// let new_value = Euler::integrate(old_value, dt, &state_estimation);
    /// assert_eq!(new_value, Array2::from_shape_vec((3, 1), vec![3.0, 5.0, -5.0]).unwrap());
    /// ```
    fn integrate(
        old_value: Array2<f32>,
        dt: Duration,
        state_estimation: &impl StateEstimation,
    ) -> Array2<f32> {
        let dt_seconds = dt.as_secs_f32();
        let estimation = state_estimation.estimate(old_value.clone());
        old_value + estimation * dt_seconds
    }
}
