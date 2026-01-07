use crate::{block::Block, continuous::solver::StateEstimation, prelude::Solver, signal::Signal};
use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
};
use ndarray::{Array2, LinalgScalar};
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct SS<I, T>
where
    T: Copy + Zero,
    I: Solver<T> + Debug,
{
    a: Array2<T>,
    b: Array2<T>,
    c: Array2<T>,
    d: Array2<T>,
    state: Array2<T>,
    initial_state: Option<Vec<T>>,
    current_input: T,
    last_output: Option<T>,
    _marker: PhantomData<I>,
}

impl<I, T> SS<I, T>
where
    T: Copy + Zero,
    I: Solver<T> + Debug,
{
    pub fn new(a: Array2<T>, b: Vec<T>, c: Vec<T>, d: T) -> Self {
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
            initial_state: None,
            last_output: None,
            current_input: T::zero(),
            _marker: PhantomData,
        }
    }

    pub fn with_initial_state(mut self, initial_state: Vec<T>) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            an, xn,
            "Initial state must match the number of rows in 'a'."
        );

        self.initial_state = Some(initial_state.clone());
        self.state = Array2::from_shape_vec((xn, 1), initial_state).unwrap();
        self
    }

    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I, T> StateEstimation<T> for SS<I, T>
where
    T: Copy + Zero + LinalgScalar,
    I: Solver<T> + Debug,
{
    fn estimate(&self, state: Array2<T>) -> Array2<T> {
        let input_matrix = Array2::from_shape_vec((1, 1), vec![self.current_input]).unwrap();
        self.a.dot(&state) + self.b.dot(&input_matrix)
    }
}

impl<I, T> Block for SS<I, T>
where
    T: Copy + Zero + LinalgScalar,
    I: Solver<T> + Debug,
{
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.current_input = input.value;
        self.state = I::integrate(self.state.clone(), input.delta.dt(), self);

        let input_matrix = Array2::from_shape_vec((1, 1), vec![input.value]).unwrap();
        let output = self.c.dot(&self.state) + self.d.dot(&input_matrix);
        let output = input.replace(output[[0, 0]]);
        self.last_output = Some(output.value);

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        if let Some(initial_state) = &self.initial_state {
            self.state =
                Array2::from_shape_vec((initial_state.len(), 1), initial_state.clone()).unwrap();
        } else {
            self.state.fill(T::zero());
        }
        self.current_input = T::zero();
        self.last_output = None;
    }
}

impl<I, T> Display for SS<I, T>
where
    I: Solver<T> + Debug,
    T: Copy + Zero + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
