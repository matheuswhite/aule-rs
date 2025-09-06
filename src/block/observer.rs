use crate::{
    prelude::{Integrator, MIMO, StateEstimation},
    signal::Signal,
};
use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
    time::Duration,
};
use ndarray::Array2;

/// Luenberger Observer for state estimation of a linear system.
/// This struct implements a Luenberger observer using the state-space representation
/// with matrices `A`, `B`, `C`, `D`, and observer gain `L`.
/// It uses an integrator to compute the state estimation over time.
///
/// # Type Parameters
/// - `I`: The integrator type used for state evolution. It must implement the `Integrator` trait.
/// - `N`: The number of states in the system.
///
/// # Example
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
/// use ndarray::array;
///
/// let a = array![[0.0, 1.0], [-5.0, -4.0]];
/// let b = [0.0, 1.0];
/// let c = [3.0, 0.0];
/// let d = 0.0;
/// let l = [10.0, 20.0];
/// let mut observer = Observer::new(a, b, c, d, l)
///     .with_initial_state([0.0, 0.0])
///     .with_integrator(Euler);
/// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
/// let output = observer.output(input);
/// assert_eq!(output[0].value, 0.0);
/// assert_eq!(output[1].value, 0.0);
/// assert_eq!(output[2].value, 1.0);
/// ```
pub struct Observer<I>
where
    I: Integrator + Debug,
{
    a: Array2<f32>,
    b: Array2<f32>,
    c: Array2<f32>,
    d: Array2<f32>,
    l: Array2<f32>,
    current_input: [f32; 2], // (u, y)
    state: Array2<f32>,
    last_output: Option<Vec<Signal>>, // (y, x_hat)
    _marker: PhantomData<I>,
}

impl<I> Observer<I>
where
    I: Integrator + Debug,
{
    /// Creates a new Luenberger observer with the given matrices and observer gain.
    ///
    /// # Parameters
    /// - `a`: The state matrix (n x n).
    /// - `b`: The input matrix (n x 1).
    /// - `c`: The output matrix (1 x n).
    /// - `d`: The feedthrough matrix (1 x 1).
    /// - `l`: The observer gain matrix (n x 1).
    ///
    /// # Returns
    /// An `Observer` instance with the specified matrices and zero initial state.
    ///
    /// # Panics
    /// Panics if the dimensions of the matrices do not match the expected sizes.
    ///
    /// - `a` must be square (n x n).
    /// - `b` must have n rows and 1 column (n x 1).
    /// - `c` must have 1 row and n columns (1 x n).
    /// - `d` must be a scalar (1 x 1).
    /// - `l` must have n rows and 1 column (n x 1).
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let mut observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
    /// let output = observer.output(input);
    /// assert_eq!(output[0].value, 0.0);
    /// assert_eq!(output[1].value, 0.0);
    /// assert_eq!(output[2].value, 1.0);
    /// ```
    pub fn new<const N: usize>(
        a: Array2<f32>,
        b: [f32; N],
        c: [f32; N],
        d: f32,
        l: [f32; N],
    ) -> Self {
        let an = a.shape()[0];
        let am = a.shape()[1];

        assert_eq!(
            an, N,
            "State dimension N must match the number of rows in 'a'."
        );
        assert_eq!(
            am, N,
            "State dimension N must match the number of columns in 'a'."
        );

        Self {
            a,
            b: Array2::from_shape_vec((N, 1), b.to_vec()).unwrap(),
            c: Array2::from_shape_vec((1, N), c.to_vec()).unwrap(),
            d: Array2::from_elem((1, 1), d),
            l: Array2::from_shape_vec((N, 1), l.to_vec()).unwrap(),
            state: Array2::zeros((N, 1)),
            last_output: None,
            current_input: [0.0; 2],
            _marker: PhantomData,
        }
    }

    /// Sets the initial state of the observer.
    ///
    /// # Parameters
    /// - `initial_state`: An array representing the initial state of the observer.
    ///
    /// # Returns
    /// An `Observer` instance with the specified initial state.
    ///
    /// # Panics
    /// Panics if the length of `initial_state` does not match the number of states.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let mut observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
    /// let output = observer.output(input);
    /// assert_eq!(output[0].value, 0.0);
    /// assert_eq!(output[1].value, 0.0);
    /// assert_eq!(output[2].value, 1.0);
    /// ```
    pub fn with_initial_state<const N: usize>(mut self, initial_state: [f32; N]) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            an, xn,
            "Initial state must match the number of rows in 'a'."
        );

        self.state = Array2::from_shape_vec((xn, 1), initial_state.to_vec()).unwrap();
        self
    }

    /// Sets the integrator for the observer.
    ///
    /// # Parameters
    /// - `_integrator`: The integrator type to be used for state evolution.
    ///
    /// # Returns
    /// An `Observer` instance with the specified integrator.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let mut observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
    /// let output = observer.output(input);
    /// assert_eq!(output[0].value, 0.0);
    /// assert_eq!(output[1].value, 0.0);
    /// assert_eq!(output[2].value, 1.0);
    /// ```
    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I> StateEstimation for Observer<I>
where
    I: Integrator + Debug,
{
    /// Estimates the next state of the system based on the current state and input.
    ///
    /// # Parameters
    /// - `state`: The current state of the system as a 2D array (n x 1).
    ///
    /// # Returns
    /// A 2D array representing the estimated next state of the system (n x 1).
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let mut observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
    /// let output = observer.output(input);
    /// assert_eq!(output[0].value, 0.0);
    /// assert_eq!(output[1].value, 0.0);
    /// assert_eq!(output[2].value, 1.0);
    /// ```
    fn estimate(&self, state: Array2<f32>) -> Array2<f32> {
        let input_matrix = Array2::from_elem((1, 1), self.current_input[0]);
        let y_hat = self.c.dot(&state) + self.d.dot(&input_matrix);
        let y = Array2::from_elem((1, 1), self.current_input[1]);
        let y_err = y - y_hat;

        self.a.dot(&state) + self.b.dot(&input_matrix) + self.l.dot(&y_err)
    }
}

impl<I> MIMO for Observer<I>
where
    I: Integrator + Debug,
{
    /// Processes the input signals and computes the output signals based on the observer model.
    ///
    /// # Parameters
    /// - `input`: A vector of input signals, where the first signal is the control input (u)
    ///   and the second signal is the measured output (y).
    ///
    /// # Returns
    /// A vector of output signals, where the first signal is the estimated output (y_hat)
    ///   and the subsequent signals are the estimated states (x_hat).
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let mut observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
    /// let output = observer.output(input);
    /// assert_eq!(output[0].value, 0.0);
    /// assert_eq!(output[1].value, 0.0);
    /// assert_eq!(output[2].value, 1.0);
    /// ```
    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
        self.current_input = [input[0].value, input[1].value]; // (u, y)
        let dt = (input[0].dt + input[1].dt).as_secs_f32() / 2.0;
        self.state = I::integrate(self.state.clone(), Duration::from_secs_f32(dt), self);

        let u = Array2::from_elem((1, 1), input[0].value);
        let y = self.c.dot(&self.state) + self.d.dot(&u);
        let dt = Duration::from_secs_f32(dt);
        let output: Vec<Signal> = vec![Signal::default(); self.dimensions().1]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i == 0 {
                    Signal {
                        value: y[[0, 0]],
                        dt,
                    }
                } else {
                    Signal {
                        value: self.state[[i - 1, 0]],
                        dt,
                    }
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.last_output = Some(output.clone());

        output
    }

    /// Returns the last output signals computed by the observer.
    ///
    /// # Returns
    /// An `Option<Vec<Signal>>` containing the last output signals, or `None` if no output has been computed yet.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let mut observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let input = [Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///              Signal { value: 0.0, dt: Duration::from_secs(1) }].to_vec(); // (u, y)
    /// let _ = observer.output(input);
    /// let last_output = observer.last_output();
    /// assert!(last_output.is_some());
    /// assert_eq!(last_output.clone().unwrap()[0].value, 0.0); // y_hat
    /// assert_eq!(last_output.clone().unwrap()[1].value, 0.0); // x_hat[0]
    /// assert_eq!(last_output.clone().unwrap()[2].value, 1.0); // x_hat[1]
    /// ```
    fn last_output(&self) -> Option<Vec<Signal>> {
        self.last_output.clone()
    }

    /// Returns the dimensions of the observer's output.
    ///
    /// The output consists of the estimated output and the estimated states.
    ///
    /// # Returns
    /// A tuple `(rows, columns)` representing the dimensions of the output.
    /// - `rows`: Always 2 (one for the estimated output and one for each state).
    /// - `columns`: Number of states + 1 (for the estimated output).
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    /// use ndarray::array;
    ///
    /// let a = array![[0.0, 1.0], [-5.0, -4.0]];
    /// let b = [0.0, 1.0];
    /// let c = [3.0, 0.0];
    /// let d = 0.0;
    /// let l = [10.0, 20.0];
    /// let observer = Observer::new(a, b, c, d, l)
    ///     .with_initial_state([0.0, 0.0])
    ///     .with_integrator(Euler);
    /// let dimensions = observer.dimensions();
    /// assert_eq!(dimensions, (2, 3)); // 2 rows (y_hat, x_hat) and 3 columns (1 output + 2 states)
    /// ```
    fn dimensions(&self) -> (usize, usize) {
        (2, self.state.shape()[0] + 1)
    }
}

impl<I> Display for Observer<I>
where
    I: Integrator + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tL: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.l, self.state
        )
    }
}
