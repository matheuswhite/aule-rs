use crate::block::Block;
use crate::prelude::{SimulationState, Solver, StateEstimation};
use core::{
    fmt::{Debug, Display},
    marker::PhantomData,
};
use faer::traits::ComplexField;
use faer::{Mat, mat};
use num_traits::Zero;

#[derive(Debug, Clone)]
pub struct Observer<I, T>
where
    T: Zero + Copy + ComplexField,
    I: Solver<T> + Debug,
{
    a: Mat<T>,
    b: Mat<T>,
    c: Mat<T>,
    d: Mat<T>,
    l: Mat<T>,
    initial_state: Option<Mat<T>>,
    current_input: ObserverInput<T>,
    state: Mat<T>,
    last_output: Option<ObserverOutput<T>>,
    _marker: PhantomData<I>,
}

impl<I, T> Observer<I, T>
where
    T: Zero + Copy + ComplexField,
    I: Solver<T> + Debug,
{
    pub fn new(a: Mat<T>, b: Mat<T>, c: Mat<T>, d: T, l: Mat<T>) -> Self {
        let n = a.shape().0;

        assert_eq!(a.shape().0, a.shape().1, "A must be a square matrix");

        assert_eq!(b.shape().0, n, "B must has {} rows", n);
        assert_eq!(b.shape().1, 1, "B must be a column matrix");

        assert_eq!(c.shape().0, 1, "C must be a row matrix");
        assert_eq!(c.shape().1, n, "C must has {} columns", n);

        assert_eq!(l.shape().0, n, "L must has {} rows", n);
        assert_eq!(l.shape().1, 1, "L must be a column matrix");

        Self {
            a,
            b,
            c,
            d: mat![[d]],
            l,
            state: Mat::zeros(n, 1),
            initial_state: None,
            last_output: None,
            current_input: ObserverInput::default(),
            _marker: PhantomData,
        }
    }

    pub fn with_initial_state(mut self, initial_state: Mat<T>) -> Self {
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

impl<I, T> StateEstimation<T> for Observer<I, T>
where
    T: Zero + Copy + ComplexField,
    I: Solver<T> + Debug,
{
    fn estimate(&self, state: Mat<T>) -> Mat<T> {
        let input_matrix = mat![[self.current_input.control_input]];
        let y_hat = &self.c * &state + &self.d + &input_matrix;
        let y = mat![[self.current_input.measured_output]];
        let y_err = y - y_hat;

        &self.a * &state + &self.b * &input_matrix + &self.l * &y_err
    }
}

impl<I, T> Block for Observer<I, T>
where
    T: Zero + Copy + ComplexField,
    I: Solver<T> + Debug,
{
    type Input = ObserverInput<T>;
    type Output = ObserverOutput<T>;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let dt = sim_state.dt();

        self.current_input = input.clone();
        self.state = I::integrate(self.state.clone(), dt, self);

        let u = mat![[input.control_input]];
        let y = &self.c * &self.state + &self.d * &u;

        let output = ObserverOutput::new(y[(0, 0)], self.state.clone());
        self.last_output = Some(output.clone());

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output.clone()
    }

    fn reset(&mut self) {
        if let Some(initial_state) = &self.initial_state {
            self.state = initial_state.clone();
        } else {
            self.state.fill(T::zero());
        }
        self.last_output = None;
        self.current_input = ObserverInput::default();
    }
}

impl<I, T> Display for Observer<I, T>
where
    T: Zero + Copy + Display + ComplexField,
    I: Solver<T> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {:?}\n\tB: {:?}\n\tC: {:?}\n\tD: {:?}\n\tL: {:?}\n\tx: {:?}",
            self.a, self.b, self.c, self.d, self.l, self.state
        )
    }
}

#[derive(Debug, Clone)]
pub struct ObserverInput<T>
where
    T: Zero + Copy + ComplexField,
{
    pub control_input: T,
    pub measured_output: T,
}

impl<T> Default for ObserverInput<T>
where
    T: Zero + Copy + ComplexField,
{
    fn default() -> Self {
        ObserverInput {
            control_input: T::zero(),
            measured_output: T::zero(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObserverOutput<T>
where
    T: Zero + Copy + ComplexField,
{
    pub measured_output: T,
    pub state_estimate: Mat<T>,
}

impl<T> ObserverOutput<T>
where
    T: Zero + Copy + ComplexField,
{
    pub fn new(measured_output: T, state_estimate: Mat<T>) -> Self {
        ObserverOutput {
            measured_output,
            state_estimate,
        }
    }
}
