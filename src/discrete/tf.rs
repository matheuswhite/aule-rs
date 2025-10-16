use crate::{block::Block, discrete::PolynomialInverse, signal::Signal, time::Discrete};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct DTf {
    numerator: PolynomialInverse,
    denominator: PolynomialInverse,
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
        }
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
}
