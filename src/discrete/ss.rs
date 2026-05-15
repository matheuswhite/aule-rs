use crate::{block::Block, prelude::SimulationState};
use core::fmt::Display;
use alloc::vec;
use nalgebra::{ClosedAddAssign, ClosedMulAssign, DMatrix, Scalar, dmatrix};
use num_traits::{One, Zero};

pub struct DSS<T>
where
    T: Copy + Zero + One + Scalar + ClosedAddAssign + ClosedMulAssign,
{
    a: DMatrix<T>,
    b: DMatrix<T>,
    c: DMatrix<T>,
    d: DMatrix<T>,
    initial_state: Option<DMatrix<T>>,
    state: DMatrix<T>,
    last_output: Option<T>,
}

impl<T> DSS<T>
where
    T: Copy + Zero + One + Scalar + ClosedAddAssign + ClosedMulAssign,
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
}

impl<T> Block for DSS<T>
where
    T: Copy + Zero + One + Scalar + ClosedAddAssign + ClosedMulAssign,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let input_matrix = dmatrix![input];
        self.state = &self.a * &self.state + &self.b * &input_matrix;

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
        self.last_output = None;
    }
}

impl<T> Display for DSS<T>
where
    T: Copy + Zero + One + Display + Scalar + ClosedAddAssign + ClosedMulAssign,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {:?}\n\tB: {:?}\n\tC: {:?}\n\tD: {:?}\n\tx: {:?}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
