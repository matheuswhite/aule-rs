use crate::{block::Block, discrete::solver::StateEstimation, prelude::Solver, signal::Signal};
use alloc::vec;
use alloc::vec::Vec;
use ndarray::Array2;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

#[derive(Debug, Clone)]
pub struct SS<I>
where
    I: Solver + Debug,
{
    a: Array2<f32>,
    b: Array2<f32>,
    c: Array2<f32>,
    d: Array2<f32>,
    state: Array2<f32>,
    current_input: f32,
    last_output: Option<f32>,
    _marker: PhantomData<I>,
}

impl<I> SS<I>
where
    I: Solver + Debug,
{
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
            current_input: 0.0,
            _marker: PhantomData,
        }
    }

    pub fn with_initial_state(mut self, initial_state: Vec<f32>) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            an, xn,
            "Initial state must match the number of rows in 'a'."
        );

        self.state = Array2::from_shape_vec((xn, 1), initial_state).unwrap();
        self
    }

    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I> StateEstimation for SS<I>
where
    I: Solver + Debug,
{
    fn estimate(&self, state: Array2<f32>) -> Array2<f32> {
        let input_matrix = Array2::from_shape_vec((1, 1), vec![self.current_input]).unwrap();
        self.a.dot(&state) + self.b.dot(&input_matrix)
    }
}

impl<I> Block for SS<I>
where
    I: Solver + Debug,
{
    type Input = f32;
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        self.current_input = input.value;
        self.state = I::integrate(self.state.clone(), input.delta.dt(), self);

        let input_matrix = Array2::from_shape_vec((1, 1), vec![input.value]).unwrap();
        let output = self.c.dot(&self.state) + self.d.dot(&input_matrix);
        let output = Signal {
            value: output[[0, 0]],
            delta: input.delta,
        };
        self.last_output = Some(output.value);

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }
}

impl<I> Display for SS<I>
where
    I: Solver + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
