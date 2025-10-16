use crate::block::Block;
use crate::signal::Signal;
use crate::time::Discrete;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Display;
use ndarray::Array2;

pub struct DSS {
    a: Array2<f32>,
    b: Array2<f32>,
    c: Array2<f32>,
    d: Array2<f32>,
    initial_state: Option<Vec<f32>>,
    state: Array2<f32>,
    last_output: Option<f32>,
}

impl DSS {
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
            initial_state: None,
            last_output: None,
        }
    }

    pub fn with_initial_state(mut self, initial_state: Vec<f32>) -> Self {
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

impl Block for DSS {
    type Input = f32;
    type Output = f32;
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
            self.state.fill(0.0);
        }
        self.last_output = None;
    }
}

impl Display for DSS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "A: {}\n\tB: {}\n\tC: {}\n\tD: {}\n\tx: {}",
            self.a, self.b, self.c, self.d, self.state
        )
    }
}
