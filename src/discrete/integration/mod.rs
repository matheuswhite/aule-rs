use crate::block::Block;

pub mod euler;
pub mod runge_kutta;

pub trait Discretizable<T> {
    /// Converts a continuous-time system into a discrete-time system.
    ///
    /// # Returns
    /// A new system that implements the `Integrator` trait.
    ///
    /// # Type Parameters
    /// `T`: The type of the discrete-time system.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    ///
    /// let tf = 1.0 / (s - 1.0);
    /// let discrete_tf: Euler = tf.discretize();
    /// ```
    fn discretize(self) -> T
    where
        T: Integrator;
}

pub trait Integrator: Block {}
