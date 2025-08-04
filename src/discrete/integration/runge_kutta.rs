use crate::prelude::Integrator;
use ndarray::Array2;
use std::time::Duration;

/// Runge-Kutta 4th order integration method for solving ordinary differential equations (ODEs).
///
/// This struct implements the `Integrator` trait, providing a method to integrate
/// the state of a system using the Runge-Kutta method.
/// The Runge-Kutta method is a more accurate numerical procedure for solving ordinary differential equations
/// compared to the Euler method, using multiple estimates to calculate the next state.
/// The RK4 method is defined as follows:
/// x[k] = x[k-1] + (k1 + 2*k2 + 2*k3 + k4) * dt / 6
/// where:
/// k1 = f(k-2, x[k-1])
/// k2 = f(k-1, x[k-1] + k1 * dt / 2)
/// k3 = f(k-1,x[k-1] + k2 * dt / 2)
/// k4 = f(k, x[k-1] + k3 * dt)
///
/// # Note:
/// This implementation uses the previous time as midpoint for k2 and k3 calculations, and the 2nd previous time for k1 and the current time for k4.
///
/// # Example:
/// ```
/// use aule::prelude::*;
/// use ndarray::Array2;
/// use std::time::Duration;
///
/// let old_value = Array2::from_shape_vec((3, 1), vec![1.0, 2.0, 3.0]).unwrap();
/// let dt = Duration::from_secs(1);
/// let mut slop_estimation = |_, x: Array2<f32>| x * 0.1; // Example estimation function
/// let new_value = RK4::integrate(old_value, dt, slop_estimation);
/// assert_eq!(new_value, Array2::from_shape_vec((3, 1), vec![1.1051708, 2.2103417, 3.3155127]).unwrap());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RK4;

impl Integrator for RK4 {
    /// Integrates the old value using the Runge-Kutta method.
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
    /// let mut slop_estimation = |_, x: Array2<f32>| x * 0.1; // Example estimation function
    /// let new_value = RK4::integrate(old_value, dt, slop_estimation);
    /// assert_eq!(new_value, Array2::from_shape_vec((3, 1), vec![1.1051708, 2.2103417, 3.3155127]).unwrap());
    /// ```
    fn integrate<F>(old_value: Array2<f32>, dt: Duration, mut slop_estimation: F) -> Array2<f32>
    where
        F: FnMut(f32, Array2<f32>) -> Array2<f32>,
    {
        let dt_seconds = dt.as_secs_f32();
        let k1 = slop_estimation(0.0, old_value.clone());
        let k2 = slop_estimation(0.5, old_value.clone() + k1.clone() * (dt_seconds / 2.0));
        let k3 = slop_estimation(0.5, old_value.clone() + k2.clone() * (dt_seconds / 2.0));
        let k4 = slop_estimation(1.0, old_value.clone() + k3.clone() * dt_seconds);

        old_value + (k1 + 2.0 * k2 + 2.0 * k3 + k4) * (dt_seconds / 6.0)
    }
}
