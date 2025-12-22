use crate::block::Block;
use crate::signal::Signal;
use crate::time::Discrete;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Display;
use ndarray::{Array2, LinalgScalar};
use num_traits::Zero;

pub struct DSS<T>
where
    T: Copy + Zero,
{
    a: Array2<T>,
    b: Array2<T>,
    c: Array2<T>,
    d: Array2<T>,
    initial_state: Option<Vec<T>>,
    state: Array2<T>,
    last_output: Option<T>,
}

impl<T> DSS<T>
where
    T: Copy + Zero,
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
        }
    }

    pub fn with_initial_state(mut self, initial_state: Vec<T>) -> Self {
        let an = self.a.shape()[0];
        let xn = initial_state.len();

        assert_eq!(
            xn, an,
            "Initial state vector must have the same length as the number of states."
        );

        self.initial_state = Some(initial_state.clone());
        self.state = Array2::from_shape_vec((xn, 1), initial_state).unwrap();
        self
    }
}

impl<T> Block for DSS<T>
where
    T: Copy + Zero + LinalgScalar,
{
    type Input = T;
    type Output = T;
    type TimeType = Discrete;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let input_matrix = Array2::from_shape_vec((1, 1), vec![input.value]).unwrap();
        self.state = self.a.dot(&self.state) + self.b.dot(&input_matrix);

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
            let xn = initial_state.len();
            self.state = Array2::from_shape_vec((xn, 1), initial_state.clone()).unwrap();
        } else {
            self.state.fill(T::zero());
        }
        self.last_output = None;
    }
}

impl<T> Display for DSS<T>
where
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
