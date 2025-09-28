use crate::{
    prelude::{MIMO, Solver, StateEstimation},
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

pub struct Observer<I>
where
    I: Solver + Debug,
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
    I: Solver + Debug,
{
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

    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I> StateEstimation for Observer<I>
where
    I: Solver + Debug,
{
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
    I: Solver + Debug,
{
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

    fn last_output(&self) -> Option<Vec<Signal>> {
        self.last_output.clone()
    }

    fn dimensions(&self) -> (usize, usize) {
        (2, self.state.shape()[0] + 1)
    }
}

impl<I> Display for Observer<I>
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
