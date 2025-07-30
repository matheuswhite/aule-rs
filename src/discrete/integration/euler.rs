use crate::{
    block::{AsBlock, Block, Signal},
    discrete::integration::Integrator,
    poly::Polynomial,
};

pub struct Euler {
    x_dot: Vec<f32>,
    b: Polynomial,
    y_dot: Vec<f32>,
    a: Polynomial,
    last_output: Option<Signal>,
}

impl Euler {
    pub fn new(inputs: Polynomial, outputs: Polynomial) -> Euler {
        if inputs.degree() < 0 || outputs.degree() < 0 {
            panic!("Input and output polynomials must have non-negative degrees.");
        }

        Euler {
            x_dot: vec![0.0; inputs.degree() as usize],
            y_dot: vec![0.0; outputs.degree() as usize],
            b: inputs,
            a: outputs,
            last_output: None,
        }
    }

    pub fn with_initial_condition(mut self, inputs: &[f32], outputs: &[f32]) -> Self {
        for i in 0..usize::min(self.x_dot.len(), inputs.len()) {
            self.x_dot[i] = inputs[i];
        }

        for i in 0..usize::min(self.y_dot.len(), outputs.len()) {
            self.y_dot[i] = outputs[i];
        }

        self
    }

    fn update_x_dot(&mut self, input: f32, dt: f32) {
        let mut new_x_dot = vec![];

        /* x[k] */
        new_x_dot.push(input);

        for i in 1..self.x_dot.len() {
            new_x_dot[i] = (new_x_dot[i - 1] - self.x_dot[i]) / dt;
        }

        self.x_dot = new_x_dot;
    }

    fn update_y_dot(&mut self, dt: f32) {
        for i in 0..self.y_dot.len() - 1 {
            self.y_dot[i] += dt * self.y_dot[i + 1];
        }

        let last_index = self.y_dot.len() - 1;
        self.y_dot[last_index] += dt * self.nth_output_derivative();
    }

    fn nth_output_derivative(&self) -> f32 {
        /* y^n(t) = (1/an)(∑bi*x^i(t) - ∑ai*y^i(t)) */

        let an_inv = 1.0 / self.a.lead_coeff();

        an_inv
            * self
                .b
                .coeff()
                .iter()
                .zip(self.x_dot.iter())
                .fold(0.0, |acc, (&cx, &x)| acc + cx * x)
            - self
                .a
                .coeff()
                .iter()
                .zip(self.y_dot.iter())
                .fold(0.0, |acc, (&cy, &y)| acc + cy * y)
    }
}

impl Block for Euler {
    fn output(&mut self, input: Signal) -> Signal {
        let dt = input.dt.as_secs_f32();

        self.update_x_dot(input.value, dt);
        self.update_y_dot(dt);

        let last_output = Signal {
            value: self.y_dot[0],
            dt: input.dt,
        };
        self.last_output = Some(last_output);

        last_output
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl Integrator for Euler {}

impl AsBlock for Euler {}
