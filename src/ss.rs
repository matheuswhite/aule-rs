use crate::block::{AsBlock, Block, Signal};
use ndarray::{Array, Dim};

pub type Matrix<T> = Array<T, Dim<[usize; 2]>>;

pub struct StateSpace {
    a: Matrix<f32>,
    b: Matrix<f32>,
    c: Matrix<f32>,
    d: Matrix<f32>,
    x: Matrix<f32>,
    last_output: Option<Signal>,
}

#[derive(Default)]
pub struct StateSpaceState {}

impl StateSpace {
    pub fn new<const N: usize>(a: [[f32; N]; N], b: [f32; N], c: [f32; N], d: f32) -> Self {
        StateSpace {
            a: Array::from_shape_vec((N, N), a.iter().flatten().cloned().collect()).unwrap(),
            b: Array::from_shape_vec((N, 1), b.to_vec()).unwrap(),
            c: Array::from_shape_vec((1, N), c.to_vec()).unwrap(),
            d: Array::from_shape_vec((1, 1), vec![d]).unwrap(),
            x: Array::from_shape_vec((N, 1), vec![0.0; N]).unwrap(),
            last_output: None,
        }
    }
}

impl Block for StateSpace {
    fn output(&mut self, input: Signal) -> Signal {
        let u = Array::from_shape_vec((1, 1), vec![input.value]).unwrap();
        let new_x = self.a.dot(&self.x) + self.b.dot(&u);
        let output = self.c.dot(&self.x) + self.d.dot(&u);
        self.x = new_x;

        let output = Signal {
            value: output[[0, 0]],
            dt: input.dt,
        };
        self.last_output = Some(output);
        output
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for StateSpace {}
