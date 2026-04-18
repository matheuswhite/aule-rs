use crate::block::Block;
use crate::signal::Signal;
use core::fmt::Display;
use faer::{Mat, mat, traits::ComplexField};
use num_traits::Zero;

pub struct DSS<T>
where
    T: Copy + Zero + ComplexField,
{
    a: Mat<T>,
    b: Mat<T>,
    c: Mat<T>,
    d: Mat<T>,
    initial_state: Option<Mat<T>>,
    state: Mat<T>,
    last_output: Option<T>,
}

impl<T> DSS<T>
where
    T: Copy + Zero + ComplexField,
{
    pub fn new(a: Mat<T>, b: Mat<T>, c: Mat<T>, d: T) -> Self {
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
            d: mat![[d]],
            state: Mat::zeros(n, 1),
            initial_state: None,
            last_output: None,
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
}

impl<T> Block for DSS<T>
where
    T: Copy + Zero + ComplexField,
{
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let input_matrix = mat![[input.value]];
        self.state = &self.a * &self.state + &self.b * &input_matrix;

        let output = &self.c * &self.state + &self.d * &input_matrix;
        let output = input.replace(output[(0, 0)]);
        self.last_output = Some(output.value);
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
    T: Copy + Zero + Display + ComplexField,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {:?}\n\tB: {:?}\n\tC: {:?}\n\tD: {:?}\n\tx: {:?}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
