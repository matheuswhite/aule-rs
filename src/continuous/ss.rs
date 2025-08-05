use crate::{
    block::{AsBlock, Block},
    discrete::integration::StateEstimation,
    prelude::Integrator,
    signal::Signal,
};
use ndarray::Array2;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// State-Space representation of a linear time-invariant system.
/// This struct implements the state-space model with matrices `A`, `B`, `C`, and `D`.
/// It uses an integrator to compute the state evolution over time.
///
/// # Type Parameters
/// - `I`: The integrator type used for state evolution. It must implement the `Integrator` trait.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
/// use ndarray::array;
///
/// let a = array![[0.0, 1.0], [-5.0, -4.0]];
/// let b = vec![0.0, 1.0];
/// let c = vec![3.0, 0.0];
/// let d = 0.0;
/// let mut ss: SS<Euler> = SS::new(a, b, c, d)
///     .with_initial_state(vec![0.0, 0.0], Some(0.0));
/// let input = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// let output = ss.output(input);
/// assert_eq!(output.value, 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct SS<I>
where
    I: Integrator + Debug,
{
    a: Array2<f32>,
    b: Array2<f32>,
    c: Array2<f32>,
    d: Array2<f32>,
    state: Array2<f32>,
    last_input: f32,
    current_input: f32,
    last_output: Option<Signal>,
    _marker: PhantomData<I>,
}

impl<I> SS<I>
where
    I: Integrator + Debug,
{
    /// Creates a new state-space representation with the given matrices.
    ///
    /// # Parameters
    /// - `a`: The state matrix (n x n).
    /// - `b`: The input matrix (n x 1).
    /// - `c`: The output matrix (1 x n).
    /// - `d`: The feedthrough matrix (1 x 1).
    ///
    /// # Returns
    /// A new `SS` instance initialized with the provided matrices.
    ///
    /// # Panics
    /// Panics if the dimensions of the matrices do not match the expected sizes.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-2.0, -3.0]];
    /// let b = vec![0.0, 1.0];
    /// let c = vec![1.0, 0.0];
    /// let d = 0.0;
    /// let ss: SS<Euler> = SS::new(a, b, c, d);
    /// ```
    pub fn new(a: Array2<f32>, b: Vec<f32>, c: Vec<f32>, d: f32) -> Self {
        let an = a.shape()[0];
        let am = a.shape()[1];
        let bn = b.len();
        let cn = c.len();

        assert_eq!(an, am, "Matrix 'a' must be square.");
        assert_eq!(
            bn, an,
            "Matrix 'b' must have the same number of rows as 'a'."
        );
        assert_eq!(
            cn, an,
            "Matrix 'c' must have the same number of columns as 'a'."
        );

        Self {
            a,
            b: Array2::from_shape_vec((bn, 1), b).unwrap(),
            c: Array2::from_shape_vec((1, cn), c).unwrap(),
            d: Array2::from_shape_vec((1, 1), vec![d]).unwrap(),
            state: Array2::zeros((an, 1)),
            last_output: None,
            last_input: 0.0,
            current_input: 0.0,
            _marker: PhantomData,
        }
    }

    /// Sets the initial state and inputs for the state-space model.
    ///
    /// # Parameters
    /// - `initial_state`: A vector representing the initial state of the system.
    /// - `initial_inputs`: A vector representing the initial inputs to the system.
    ///
    /// # Returns
    /// A mutable reference to `self` for method chaining.
    ///
    /// # Panics
    /// Panics if the length of `initial_state` does not match the number of rows in matrix `A`.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use ndarray::array;
    ///
    /// let mut ss: SS<Euler> = SS::new(
    ///     array![[0.0, 1.0], [-2.0, -3.0]],
    ///     vec![0.0, 1.0],
    ///     vec![1.0, 0.0],
    ///     0.0,
    /// ).with_initial_state(vec![0.0, 0.0], Some(0.0));
    /// ```
    pub fn with_initial_state(
        mut self,
        initial_state: Vec<f32>,
        initial_input: Option<f32>,
    ) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            an, xn,
            "Initial state must match the number of rows in 'a'."
        );

        self.state = Array2::from_shape_vec((xn, 1), initial_state).unwrap();
        self.last_input = initial_input.unwrap_or(0.0);
        self
    }
}

impl<I> StateEstimation for SS<I>
where
    I: Integrator + Debug,
{
    /// Estimates the new state of the system based on the current state and a time step.
    ///
    /// This method implements the state-space equation:
    /// x' = A * x + B * u
    /// where x is the state vector, u is the input, A is the state matrix, and B is the input matrix.
    /// The input is interpolated between the last and current input based on the time step `dt`.
    ///
    /// # Arguments
    /// * `dt` - The time step for which to estimate the state.
    /// * `state` - The current state of the system.
    ///
    /// # Returns
    /// An `Array2<f32>` representing the estimated state of the system.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use ndarray::array;
    /// use std::time::Duration;
    ///
    /// let a = array![[0.0, 1.0], [-2.0, -3.0]];
    /// let b = vec![0.0, 1.0];
    /// let c = vec![1.0, 0.0];
    /// let d = 0.0;
    /// let mut ss: SS<Euler> = SS::new(a, b, c, d)
    ///     .with_initial_state(vec![0.0, 0.0], Some(0.0));
    /// let state = array![[0.0], [0.0]];
    /// let dt = 0.1;
    /// let estimated_state = ss.estimate(dt, state);
    /// ```
    fn estimate(&self, dt: f32, state: Array2<f32>) -> Array2<f32> {
        let input = (1.0 - dt) * self.last_input + dt * self.current_input;
        let input_matrix = Array2::from_shape_vec((1, 1), vec![input]).unwrap();
        self.a.dot(&state) + self.b.dot(&input_matrix)
    }
}

impl<I> Block for SS<I>
where
    I: Integrator + Debug,
{
    /// Processes the input signal and computes the output based on the state-space model.
    ///
    /// # Parameters
    /// - `input`: The input signal containing the value and the time step.
    ///
    /// # Returns
    /// The output signal computed from the state-space model.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-2.0, -3.0]];
    /// let b = vec![0.0, 1.0];
    /// let c = vec![1.0, 0.0];
    /// let d = 0.0;
    /// let mut ss: SS<Euler> = SS::new(a, b, c, d)
    ///     .with_initial_state(vec![0.0, 0.0], Some(0.0));
    /// let input = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output = ss.output(input);
    /// assert_eq!(output.value, 0.0);
    /// ```
    fn output(&mut self, input: Signal) -> Signal {
        self.current_input = input.value;
        // TODO: Is this correct? Should I put the result into the self.state? Or should I need to do some shift?
        self.state = I::integrate(self.state.clone(), input.dt, self);

        let input_matrix = Array2::from_shape_vec((1, 1), vec![input.value]).unwrap();
        let output = self.c.dot(&self.state) + self.d.dot(&input_matrix);
        let output = Signal {
            value: output[[0, 0]],
            dt: input.dt,
        };
        self.last_output = Some(output);
        self.last_input = input.value;

        output
    }

    /// Returns the last output signal computed by the state-space model.
    ///
    /// # Returns
    /// An `Option<Signal>` containing the last output signal, or `None` if no output has been computed yet.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-2.0, -3.0]];
    /// let b = vec![0.0, 1.0];
    /// let c = vec![1.0, 0.0];
    /// let d = 0.0;
    /// let mut ss: SS<Euler> = SS::new(a, b, c, d)
    ///     .with_initial_state(vec![0.0, 0.0], Some(0.0));
    /// let input = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let _ = ss.output(input);
    /// let last_output = ss.last_output();
    /// assert!(last_output.is_some());
    /// assert_eq!(last_output.unwrap().value, 0.0);
    /// ```
    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl<I> AsBlock for SS<I> where I: Integrator + Debug + 'static {}

impl<I> Display for SS<I>
where
    I: Integrator + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
