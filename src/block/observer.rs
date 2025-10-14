use crate::{
    block::Block,
    prelude::{Solver, StateEstimation},
    signal::Signal,
};
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
};
use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct Observer<I, const N: usize>
where
    I: Solver + Debug,
{
    a: Array2<f32>,
    b: Array2<f32>,
    c: Array2<f32>,
    d: Array2<f32>,
    l: Array2<f32>,
    current_input: (f32, f32), // (u, y)
    state: Array2<f32>,
    last_output: Option<Signal<(f32, [f32; N])>>, // (y, x_hat)
    _marker: PhantomData<I>,
}

impl<I, const N: usize> Observer<I, N>
where
    I: Solver + Debug,
{
    pub fn new(a: Array2<f32>, b: [f32; N], c: [f32; N], d: f32, l: [f32; N]) -> Self {
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
            current_input: (0.0, 0.0),
            _marker: PhantomData,
        }
    }

    pub fn with_initial_state(mut self, initial_state: [f32; N]) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            an, xn,
            "Initial state must match the number of rows in 'a'."
        );

        self.state = Array2::from_shape_vec((xn, 1), initial_state.to_vec()).unwrap();
        self
    }

    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I, const N: usize> StateEstimation for Observer<I, N>
where
    I: Solver + Debug,
{
    fn estimate(&self, state: Array2<f32>) -> Array2<f32> {
        let input_matrix = Array2::from_elem((1, 1), self.current_input.0);
        let y_hat = self.c.dot(&state) + self.d.dot(&input_matrix);
        let y = Array2::from_elem((1, 1), self.current_input.1);
        let y_err = y - y_hat;

        self.a.dot(&state) + self.b.dot(&input_matrix) + self.l.dot(&y_err)
    }
}

impl<I, const N: usize> Block for Observer<I, N>
where
    I: Solver + Debug,
{
    type Input = (f32, f32); // (u, y)
    type Output = (f32, [f32; N]); // (y, x_hat)

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let dt = input.dt;

        self.current_input = (input.value.0, input.value.1); // (u, y)
        self.state = I::integrate(self.state.clone(), dt, self);

        let u = Array2::from_elem((1, 1), input.value.0);
        let y = self.c.dot(&self.state) + self.d.dot(&u);
        let output = (y[[0, 0]], {
            let mut x_hat = [0.0; N];
            for i in 0..N {
                x_hat[i] = self.state[[i, 0]];
            }
            x_hat
        });
        let output = Signal { value: output, dt };
        self.last_output = Some(output.clone());

        output
    }

    fn last_output(&self) -> Option<Signal<Self::Output>> {
        self.last_output.clone()
    }
}

impl<I, const N: usize> Display for Observer<I, N>
where
    I: Solver + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tL: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.l, self.state
        )
    }
}
