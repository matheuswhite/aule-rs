use crate::prelude::{Solver, StateEstimation};
use crate::{block::Block, signal::Signal, time::TimeType};
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
};
use ndarray::{Array2, LinalgScalar};
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct Observer<I, const N: usize, T, K>
where
    T: Zero + Copy,
    I: Solver<T> + Debug,
    K: TimeType,
{
    a: Array2<T>,
    b: Array2<T>,
    c: Array2<T>,
    d: Array2<T>,
    l: Array2<T>,
    initial_state: Option<[T; N]>,
    current_input: (T, T), // (u, y)
    state: Array2<T>,
    last_output: Option<(T, [T; N])>, // (y, x_hat)
    _marker: PhantomData<(I, K)>,
}

impl<I, const N: usize, T, K> Observer<I, N, T, K>
where
    T: Zero + Copy,
    I: Solver<T> + Debug,
    K: TimeType,
{
    pub fn new(a: Array2<T>, b: [T; N], c: [T; N], d: T, l: [T; N]) -> Self {
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
            initial_state: None,
            last_output: None,
            current_input: (T::zero(), T::zero()),
            _marker: PhantomData,
        }
    }

    pub fn with_initial_state(mut self, initial_state: [T; N]) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            an, xn,
            "Initial state must match the number of rows in 'a'."
        );

        self.initial_state = Some(initial_state);
        self.state = Array2::from_shape_vec((xn, 1), initial_state.to_vec()).unwrap();
        self
    }

    pub fn with_integrator(self, _integrator: I) -> Self {
        self
    }
}

impl<I, const N: usize, T, K> StateEstimation<T> for Observer<I, N, T, K>
where
    T: Zero + Copy + LinalgScalar,
    I: Solver<T> + Debug,
    K: TimeType,
{
    fn estimate(&self, state: Array2<T>) -> Array2<T> {
        let input_matrix = Array2::from_elem((1, 1), self.current_input.0);
        let y_hat = self.c.dot(&state) + self.d.dot(&input_matrix);
        let y = Array2::from_elem((1, 1), self.current_input.1);
        let y_err = y - y_hat;

        self.a.dot(&state) + self.b.dot(&input_matrix) + self.l.dot(&y_err)
    }
}

impl<I, const N: usize, T, K> Block for Observer<I, N, T, K>
where
    T: Zero + Copy + LinalgScalar,
    I: Solver<T> + Debug,
    K: TimeType,
{
    type Input = (T, T); // (u, y)
    type Output = (T, [T; N]); // (y, x_hat)
    type TimeType = K;
    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let dt = input.delta.dt();

        self.current_input = (input.value.0, input.value.1); // (u, y)
        self.state = I::integrate(self.state.clone(), dt, self);

        let u = Array2::from_elem((1, 1), input.value.0);
        let y = self.c.dot(&self.state) + self.d.dot(&u);
        let output = (y[[0, 0]], {
            let mut x_hat = [T::zero(); N];
            for (i, x) in x_hat.iter_mut().enumerate() {
                *x = self.state[[i, 0]];
            }
            x_hat
        });
        let output = Signal {
            value: output,
            delta: input.delta,
        };
        self.last_output = Some(output.value);

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        if let Some(initial_state) = self.initial_state {
            self.state = Array2::from_shape_vec((N, 1), initial_state.to_vec()).unwrap();
        } else {
            self.state = Array2::zeros((N, 1));
        }
        self.last_output = None;
        self.current_input = (T::zero(), T::zero());
    }
}

impl<I, const N: usize, T, K> Display for Observer<I, N, T, K>
where
    T: Zero + Copy + Display,
    I: Solver<T> + Debug,
    K: TimeType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tL: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.l, self.state
        )
    }
}
