use crate::prelude::{Solver, StateEstimation};
use crate::signal::{Pack, Unpack};
use crate::{block::Block, signal::Signal};
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
};
use ndarray::{Array2, LinalgScalar};
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct Observer<I, const N: usize, T>
where
    T: Zero + Copy,
    I: Solver<T> + Debug,
{
    a: Array2<T>,
    b: Array2<T>,
    c: Array2<T>,
    d: Array2<T>,
    l: Array2<T>,
    initial_state: Option<[T; N]>,
    current_input: ObserverInput<T>,
    state: Array2<T>,
    last_output: Option<ObserverOutput<T, N>>,
    _marker: PhantomData<I>,
}

impl<I, const N: usize, T> Observer<I, N, T>
where
    T: Zero + Copy,
    I: Solver<T> + Debug,
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
            current_input: ObserverInput::default(),
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

impl<I, const N: usize, T> StateEstimation<T> for Observer<I, N, T>
where
    T: Zero + Copy + LinalgScalar,
    I: Solver<T> + Debug,
{
    fn estimate(&self, state: Array2<T>) -> Array2<T> {
        let input_matrix = Array2::from_elem((1, 1), self.current_input.control_input);
        let y_hat = self.c.dot(&state) + self.d.dot(&input_matrix);
        let y = Array2::from_elem((1, 1), self.current_input.measured_output);
        let y_err = y - y_hat;

        self.a.dot(&state) + self.b.dot(&input_matrix) + self.l.dot(&y_err)
    }
}

impl<I, const N: usize, T> Block for Observer<I, N, T>
where
    T: Zero + Copy + LinalgScalar,
    I: Solver<T> + Debug,
{
    type Input = ObserverInput<T>;
    type Output = ObserverOutput<T, N>;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let dt = input.delta.dt();

        self.current_input = input.value.clone();
        self.state = I::integrate(self.state.clone(), dt, self);

        let u = Array2::from_elem((1, 1), input.value.control_input);
        let y = self.c.dot(&self.state) + self.d.dot(&u);

        let output = ObserverOutput::new(y[[0, 0]], {
            let mut x_hat = [T::zero(); N];
            for (i, x) in x_hat.iter_mut().enumerate() {
                *x = self.state[[i, 0]];
            }
            x_hat
        });

        self.last_output = Some(output.clone());

        input.map(|_| output)
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output.clone()
    }

    fn reset(&mut self) {
        if let Some(initial_state) = self.initial_state {
            self.state = Array2::from_shape_vec((N, 1), initial_state.to_vec()).unwrap();
        } else {
            self.state = Array2::zeros((N, 1));
        }
        self.last_output = None;
        self.current_input = ObserverInput::default();
    }
}

impl<I, const N: usize, T> Display for Observer<I, N, T>
where
    T: Zero + Copy + Display,
    I: Solver<T> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tL: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.l, self.state
        )
    }
}

#[derive(Debug, Clone)]
pub struct ObserverInput<T>
where
    T: Zero + Copy,
{
    pub control_input: T,
    pub measured_output: T,
}

impl<T> Default for ObserverInput<T>
where
    T: Zero + Copy,
{
    fn default() -> Self {
        ObserverInput {
            control_input: T::zero(),
            measured_output: T::zero(),
        }
    }
}

impl<T> Pack<ObserverInput<T>> for (Signal<T>, Signal<T>)
where
    T: Zero + Copy,
{
    fn pack(self) -> Signal<ObserverInput<T>> {
        let control_input = self.0.value;
        let measured_output = self.1.value;
        let delta = self.0.delta.merge(self.1.delta);

        Signal {
            value: ObserverInput {
                control_input,
                measured_output,
            },
            delta,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObserverOutput<T, const N: usize>
where
    T: Zero + Copy,
{
    pub measured_output: T,
    pub state_estimate: [T; N],
}

impl<T, const N: usize> ObserverOutput<T, N>
where
    T: Zero + Copy,
{
    pub fn new(measured_output: T, state_estimate: [T; N]) -> Self {
        ObserverOutput {
            measured_output,
            state_estimate,
        }
    }
}

impl<T, const N: usize> Unpack<(Signal<T>, Signal<[T; N]>)> for Signal<ObserverOutput<T, N>>
where
    T: Zero + Copy,
{
    fn unpack(self) -> (Signal<T>, Signal<[T; N]>) {
        let measured_output_signal = Signal {
            value: self.value.measured_output,
            delta: self.delta,
        };
        let state_estimate_signal = Signal {
            value: self.value.state_estimate,
            delta: self.delta,
        };

        (measured_output_signal, state_estimate_signal)
    }
}
