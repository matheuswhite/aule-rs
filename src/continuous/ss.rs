use crate::{
    block::Block,
    continuous::solver::StateEstimation,
    math::{number::Number, sample::Sample},
    prelude::{SimulationState, Solver},
};
use alloc::vec;
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
};
use nalgebra::{DMatrix, Scalar, dmatrix};

#[derive(Debug, Clone)]
pub struct SS<I, T>
where
    I: Solver<T>,
{
    a: DMatrix<T>,
    b: DMatrix<T>,
    c: DMatrix<T>,
    d: DMatrix<T>,
    state: DMatrix<T>,
    initial_state: Option<DMatrix<T>>,
    current_input: DMatrix<T>,
    last_output: Option<T>,
    _marker: PhantomData<I>,
}

impl<I, T> SS<I, T>
where
    T: Number + 'static,
    I: Solver<T>,
{
    pub fn new(a: DMatrix<T>, b: DMatrix<T>, c: DMatrix<T>, d: T) -> Self {
        let n = a.shape().0;
        assert_eq!(a.shape().0, a.shape().1, "A must be a square matrix");

        assert_eq!(b.shape().0, n, "B must has {} rows", n);
        assert_eq!(b.shape().1, 1, "B must be a column matrix");

        assert_eq!(c.shape().0, 1, "C must be a row matrix");
        assert_eq!(c.shape().1, n, "C must has {} columns", n);

        Self {
            a,
            b,
            c,
            d: dmatrix![d],
            state: DMatrix::zeros(n, 1),
            initial_state: None,
            last_output: None,
            current_input: dmatrix![Sample::zero()],
            _marker: PhantomData,
        }
    }

    pub fn with_initial_state(mut self, initial_state: DMatrix<T>) -> Self {
        let n = self.a.shape().0;
        assert_eq!(
            initial_state.shape().0,
            n,
            "Inicial state must has {} rows",
            n
        );
        assert_eq!(
            initial_state.shape().1,
            1,
            "Inicial state must be a column matrix"
        );

        self.initial_state = Some(initial_state.clone());
        self.state = initial_state;
        self
    }

    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I, T> StateEstimation<T> for SS<I, T>
where
    T: Number + Scalar,
    I: Solver<T>,
{
    fn estimate(&self, state: DMatrix<T>) -> DMatrix<T> {
        &self.a * state + &self.b * &self.current_input
    }
}

impl<I, T> Block for SS<I, T>
where
    T: Number + Scalar,
    I: Solver<T>,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        self.current_input[(0, 0)] = input;
        self.state = I::integrate(self.state.clone(), sim_state.dt(), self);

        let input_matrix = dmatrix![input];
        let output = &self.c * &self.state + &self.d * &input_matrix;
        let output = output[(0, 0)];
        self.last_output = Some(output);

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        if let Some(initial_state) = &self.initial_state {
            self.state = initial_state.clone();
        } else {
            self.state.fill(T::zero());
        }
        self.current_input[(0, 0)] = T::zero();
        self.last_output = None;
    }
}

impl<I, T> Display for SS<I, T>
where
    T: Number,
    I: Solver<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {:?}\n\tB: {:?}\n\tC: {:?}\n\tD: {:?}\n\tx: {:?}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
