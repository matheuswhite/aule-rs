use crate::{block::Block, discrete::PolynomialInverse, signal::Signal, time::Discrete};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::AddAssign;
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct DTf<T>
where
    T: Float + Default + AddAssign<T>,
{
    numerator: PolynomialInverse<T>,
    denominator: PolynomialInverse<T>,
    initial_conditions: Option<(Vec<T>, Vec<T>)>,
    last_inputs: Vec<T>,
    last_outputs: Vec<T>,
}

impl<T> DTf<T>
where
    T: Float + Default + AddAssign<T>,
{
    pub fn new(numerator: &[T], denominator: &[T]) -> Self {
        assert!(!denominator.is_empty(), "Denominator cannot be empty.");
        assert!(
            denominator.len() >= numerator.len(),
            "Denominator must have degree greater than or equal to numerator."
        );

        Self {
            numerator: PolynomialInverse::new(numerator),
            denominator: PolynomialInverse::new(denominator),
            last_inputs: vec![T::zero(); numerator.len()],
            last_outputs: vec![T::zero(); denominator.len() - 1],
            initial_conditions: None,
        }
    }

    pub fn with_initial_conditions(
        mut self,
        initial_inputs: Vec<T>,
        initial_outputs: Vec<T>,
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

impl<T> Block for DTf<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Input = T;
    type Output = T;
    type TimeType = Discrete;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        self.last_inputs.insert(0, input.value);
        self.last_inputs.pop();

        let coeff = self.denominator.coeff();
        let leading_coeff = coeff[0];
        let mut den = coeff
            .iter()
            .map(|c| -*c / leading_coeff)
            .collect::<Vec<_>>();
        den.remove(0);

        let mut output_value = T::zero();
        for (num, last_input) in self.numerator.coeff().iter().zip(self.last_inputs.iter()) {
            output_value += *num * *last_input;
        }
        for (den, last_output) in den.iter().zip(self.last_outputs.iter()) {
            output_value += *den * *last_output;
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
            self.last_inputs.fill(T::zero());
            self.last_outputs.fill(T::zero());
        }
    }
}
