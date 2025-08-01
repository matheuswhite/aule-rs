use crate::block::Block;

pub mod euler;
pub mod runge_kutta;

/// A trait for discretizing continuous-time systems into discrete-time systems.
/// This trait provides a method to convert a continuous-time system into a discrete-time system
/// using a specific discretization method.
///
/// # Type Parameters
/// `T`: The type of the discrete-time system that implements the `Integrator` trait.
///
/// # Implementations
/// Implementations of this trait should provide the logic for converting the continuous-time system
/// into a discrete-time system, typically by applying a specific integration method such as Euler's method
/// or Runge-Kutta methods.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// use aule::poly::Polynomial;
///
/// struct MyContinuousSystem;
///
/// impl Discretizable<Euler> for MyContinuousSystem {
///     fn discretize(self) -> Euler {
///         // Logic to convert the continuous system into a discrete system using Euler's method
///         Euler::new(Polynomial::new(&[1.0]), Polynomial::new(&[1.0, 0.0]))
///     }
/// }
/// ```
pub trait Discretizable<T> {
    fn discretize(self) -> T
    where
        T: Integrator;
}

/// A trait for discrete-time integrators.
/// This trait extends the `Block` trait, allowing discrete-time systems to be treated as blocks
/// in a signal processing context.
///
/// # Implementations
/// Implementations of this trait should provide the logic for processing input signals and producing output signals
/// in a discrete-time manner.
///
/// # Example
/// ```
/// use aule::prelude::*;
///
/// struct MyIntegrator;
///
/// impl Block for MyIntegrator {
///     fn output(&mut self, input: Signal) -> Signal {
///         // Logic to compute the output based on the input signal
///         Signal { value: input.value * 2.0, dt: input.dt }
///     }
///
///     fn last_output(&self) -> Option<Signal> {
///         None
///     }
/// }
///
/// impl Integrator for MyIntegrator {}
/// ```
pub trait Integrator: Block {}
