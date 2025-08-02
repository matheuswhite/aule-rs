use crate::{
    block::{AsBlock, Block},
    discrete::integration::Integrator,
    poly::Polynomial,
    signal::Signal,
};

/// A simple Euler integrator for discrete systems.
/// It uses polynomial coefficients to define the system dynamics.
/// The input is a polynomial representing the system's inputs,
/// and the output is a polynomial representing the system's outputs.
/// The integrator computes the state derivatives and updates the system state accordingly.
/// It is suitable for systems where the dynamics can be expressed as a polynomial relationship.
/// This implementation assumes that the input and output polynomials have non-negative degrees.
///
/// # Example:
/// ```
/// use aule::prelude::*;
/// use aule::poly::Polynomial;
///
/// let inputs = Polynomial::new(&[1.0]);
/// let outputs = Polynomial::new(&[0.05, 5.0]);
/// let mut euler = Euler::new(inputs, outputs)
///     .with_initial_condition(&[0.0], &[0.0]);
/// let input_signal = Signal { value: 1.0, dt: std::time::Duration::from_secs(1) };
/// let output_signal = euler.output(input_signal);
/// assert_eq!(output_signal.value, 20.0); // After one second, y should be 1.0
/// ```
pub struct Euler {
    x_dot: Vec<f32>,
    b: Polynomial,
    y_dot: Vec<f32>,
    a: Polynomial,
    last_output: Option<Signal>,
}

impl Euler {
    /// Creates a new Euler integrator with the given input and output polynomials.
    /// The input polynomial represents the system's inputs, and the output polynomial represents the system's outputs.
    ///
    /// # Panics
    /// Panics if the input or output polynomials have negative degrees.
    /// # Arguments
    /// * `inputs` - A polynomial representing the system's inputs.
    /// * `outputs` - A polynomial representing the system's outputs.
    /// # Returns
    /// A new instance of `Euler` integrator.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use aule::poly::Polynomial;
    ///
    /// let inputs = Polynomial::new(&[1.0]);
    /// let outputs = Polynomial::new(&[0.05, 5.0]);
    /// let euler = Euler::new(inputs, outputs);
    /// ```
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

    /// Sets the initial conditions for the integrator.
    /// This method initializes the state derivatives for both inputs and outputs.
    /// It allows the user to specify initial values for the input and output states.
    ///
    /// # Arguments
    /// * `inputs` - A slice of f32 representing the initial conditions for the input states.
    /// * `outputs` - A slice of f32 representing the initial conditions for the output states.
    ///
    /// # Returns
    /// A `Euler` instance with the initial conditions set.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use aule::poly::Polynomial;
    ///
    /// let inputs = Polynomial::new(&[1.0]);
    /// let outputs = Polynomial::new(&[0.05, 5.0]);
    /// let euler = Euler::new(inputs, outputs)
    ///     .with_initial_condition(&[1.0], &[2.0]);
    /// ```
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
    /// Computes the output of the Euler integrator based on the input signal.
    /// It updates the state derivatives and returns the last output signal.
    ///
    /// # Arguments
    /// * `input` - A `Signal` representing the input to the integrator.
    ///
    /// # Returns
    /// A `Signal` representing the output of the integrator.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use aule::poly::Polynomial;
    ///
    /// let inputs = Polynomial::new(&[1.0]);
    /// let outputs = Polynomial::new(&[0.05, 5.0]);
    /// let mut euler = Euler::new(inputs, outputs)
    ///     .with_initial_condition(&[0.0], &[0.0]);
    /// let input_signal = Signal { value: 1.0, dt: std::time::Duration::from_secs(1) };
    /// let output_signal = euler.output(input_signal);
    /// assert_eq!(output_signal.value, 20.0);
    /// ```
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

    /// Returns the last output signal of the integrator.
    /// This method is useful for retrieving the last computed output after processing an input signal.
    ///
    /// # Returns
    /// An `Option<Signal>` containing the last output signal if available, or `None` if no output has been computed yet.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    /// use aule::poly::Polynomial;
    ///
    /// let inputs = Polynomial::new(&[1.0]);
    /// let outputs = Polynomial::new(&[0.05, 5.0]);
    /// let mut euler = Euler::new(inputs, outputs)
    ///     .with_initial_condition(&[0.0], &[0.0]);
    /// let input_signal = Signal { value: 1.0, dt: std::time::Duration::from_secs(1) };
    /// let _ = euler.output(input_signal);
    /// let last_output = euler.last_output();
    /// assert!(last_output.is_some());
    /// assert_eq!(last_output.unwrap().value, 20.0);
    /// ```
    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl Integrator for Euler {}

impl AsBlock for Euler {}
