use crate::{block::Block, discrete::PolynomialInverse, signal::Signal, time::Discrete};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct DTf {
    numerator: PolynomialInverse,
    denominator: PolynomialInverse,
    initial_conditions: Option<(Vec<f32>, Vec<f32>)>,
    last_inputs: Vec<f32>,
    last_outputs: Vec<f32>,
}

impl DTf {
    pub fn new(numerator: &[f32], denominator: &[f32]) -> Self {
        assert!(!denominator.is_empty(), "Denominator cannot be empty.");
        assert!(
            denominator.len() >= numerator.len(),
            "Denominator must have degree greater than or equal to numerator."
        );

        Self {
            numerator: PolynomialInverse::new(numerator),
            denominator: PolynomialInverse::new(denominator),
            last_inputs: vec![0.0; numerator.len()],
            last_outputs: vec![0.0; denominator.len() - 1],
            initial_conditions: None,
        }
    }

    pub fn with_initial_conditions(
        mut self,
        initial_inputs: Vec<f32>,
        initial_outputs: Vec<f32>,
    ) -> Self {
        assert_eq!(
            initial_inputs.len(),
            self.last_inputs.len(),
            "Initial inputs length must match numerator degree."
        );
        assert_eq!(
            initial_outputs.len(),
            self.last_outputs.len(),
            "Initial outputs length must match denominator degree minus one."
        );

        self.initial_conditions = Some((initial_inputs.clone(), initial_outputs.clone()));
        self.last_inputs = initial_inputs;
        self.last_outputs = initial_outputs;
        self
    }
}

impl Block for DTf {
    type Input = f32;
    type Output = f32;
    type TimeType = Discrete;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        self.last_inputs.insert(0, input.value);
        self.last_inputs.pop();

        let coeff = self.denominator.coeff();
        let leading_coeff = coeff[0];
        let mut den = coeff.iter().map(|c| -c / leading_coeff).collect::<Vec<_>>();
        den.remove(0);

        let mut output_value = 0.0;
        for (num, last_input) in self.numerator.coeff().iter().zip(self.last_inputs.iter()) {
            output_value += num * last_input;
        }
        for (den, last_output) in den.iter().zip(self.last_outputs.iter()) {
            output_value += den * last_output;
        }

        self.last_outputs.insert(0, output_value);
        self.last_outputs.pop();

        input.replace(output_value)
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_outputs.first().copied()
    }

    fn reset(&mut self) {
        if let Some((initial_inputs, initial_outputs)) = &self.initial_conditions {
            self.last_inputs = initial_inputs.clone();
            self.last_outputs = initial_outputs.clone();
        } else {
            self.last_inputs.fill(0.0);
            self.last_outputs.fill(0.0);
        }
    }
}
