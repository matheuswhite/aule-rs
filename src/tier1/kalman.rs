use crate::block::Block;
use crate::math::number::Number;
use crate::prelude::SimulationState;
use nalgebra::{ComplexField, DMatrix};

pub struct Kalman<T> {
    a: DMatrix<T>,
    b: DMatrix<T>,
    c: DMatrix<T>,
    x: DMatrix<T>,
    p: DMatrix<T>,
    q: DMatrix<T>,
    r: DMatrix<T>,
    i: DMatrix<T>,
    last_output: Option<DMatrix<T>>,
    initial_x: DMatrix<T>,
    initial_p: DMatrix<T>,
}

impl<T> Kalman<T>
where
    T: Number + 'static,
{
    pub fn new(a: DMatrix<T>, b: DMatrix<T>, c: DMatrix<T>, q: DMatrix<T>, r: DMatrix<T>) -> Self {
        let n = a.shape().0;

        assert_eq!(a.shape().0, a.shape().1, "A must be a square matrix");

        assert_eq!(b.shape().0, n, "B must has {} rows", n);
        assert_eq!(b.shape().1, 1, "B must be a column matrix");

        assert_eq!(c.shape().0, 1, "C must be a row matrix");
        assert_eq!(c.shape().1, n, "C must has {} columns", n);

        assert_eq!(q.shape().0, q.shape().1, "Q must be a square matrix");
        assert_eq!(q.shape().0, n, "Q must has {} rows", n);

        Self {
            a,
            b,
            c,
            x: DMatrix::zeros(n, 1),
            p: DMatrix::zeros(n, n),
            q,
            r,
            i: DMatrix::identity(n, n),
            initial_x: DMatrix::zeros(n, 1),
            initial_p: DMatrix::zeros(n, n),
            last_output: None,
        }
    }

    pub fn with_p(mut self, p: DMatrix<T>) -> Self {
        self.p = p;
        self.initial_p = self.p.clone();
        self
    }

    pub fn with_x(mut self, x: DMatrix<T>) -> Self {
        self.x = x;
        self.initial_x = self.x.clone();
        self
    }
}

impl<T> Block for Kalman<T>
where
    T: Number + ComplexField + 'static,
{
    type Input = (DMatrix<T>, DMatrix<T>); // (u, y)
    type Output = Option<DMatrix<T>>; // x_hat

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let (u, y) = input;

        let x_hat_minus = &self.a * &self.x + &self.b * u;
        let p_minus = &self.a * &self.p * self.a.transpose() + &self.q;
        let k = &p_minus
            * self.c.transpose()
            * (&self.c * p_minus * self.c.transpose() + &self.r)
                .lu()
                .try_inverse()?;
        let r = y - &self.c * &x_hat_minus;
        self.x = x_hat_minus + &k * r;
        self.p = (&self.i - k * &self.c) * &self.p;

        self.last_output = Some(self.x.clone());

        self.last_output.clone()
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output.clone().map(Some)
    }

    fn reset(&mut self) {
        self.x = self.initial_x.clone();
        self.p = self.initial_p.clone();
    }
}
