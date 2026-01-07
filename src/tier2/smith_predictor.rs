use crate::block::Block;
use crate::signal::Pack;
use crate::{prelude::Delay, signal::Signal};
use core::ops::{Mul, Sub};
use core::time::Duration;
use num_traits::Zero;

pub struct SmithPredictor<T, P>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T>,
{
    process: P,
    delay: Delay<T>,
    last_output: Option<T>,
}

pub struct SmithPredictorFiltered<T, P, F>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T>,
    F: Block<Input = T, Output = T>,
{
    process: P,
    filter: F,
    delay: Delay<T>,
    last_output: Option<T>,
}

pub struct SmithPredictorInput<T>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
{
    pub control_signal: Signal<T>,
    pub measured_output: Signal<T>,
}

impl<T, P> SmithPredictor<T, P>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T>,
{
    pub fn new(process: P, delay: Duration) -> Self {
        SmithPredictor {
            process,
            delay: Delay::new(delay),
            last_output: None,
        }
    }
}

impl<T, P, F> SmithPredictorFiltered<T, P, F>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T>,
    F: Block<Input = T, Output = T>,
{
    pub fn new(process: P, filter: F, delay: Duration) -> Self {
        SmithPredictorFiltered {
            process,
            filter,
            delay: Delay::new(delay),
            last_output: None,
        }
    }
}

impl<T, P> Block for SmithPredictor<T, P>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T>,
{
    type Input = SmithPredictorInput<T>;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let predicted_output = self.process.output(input.value.control_signal);
        let delayed_predicted_output = self.delay.output(input.value.measured_output);

        let output_diff = input.value.measured_output - delayed_predicted_output;

        let output = predicted_output + output_diff;
        self.last_output = Some(output.value);
        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        self.process.reset();
        self.delay.reset();
        self.last_output = None;
    }
}

impl<T, P, F> Block for SmithPredictorFiltered<T, P, F>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T>,
    F: Block<Input = T, Output = T>,
{
    type Input = SmithPredictorInput<T>;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let predicted_output = self.process.output(input.value.control_signal);
        let delayed_predicted_output = self.delay.output(predicted_output);

        let output_diff = input.value.measured_output - delayed_predicted_output;
        let output_diff_filtered = self.filter.output(output_diff);

        let output = predicted_output + output_diff_filtered;
        self.last_output = Some(output.value);
        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        self.process.reset();
        self.filter.reset();
        self.delay.reset();
        self.last_output = None;
    }
}

impl<T> Pack<SmithPredictorInput<T>> for (Signal<T>, Signal<T>)
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
{
    fn pack(self) -> Signal<SmithPredictorInput<T>> {
        let control_signal = self.0;
        let measured_output = self.1;
        let delta = self.0.delta.merge(self.1.delta);

        Signal {
            value: SmithPredictorInput {
                control_signal,
                measured_output,
            },
            delta,
        }
    }
}
