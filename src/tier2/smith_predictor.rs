use num_traits::Zero;

use crate::block::Block;
use crate::time::TimeType;
use crate::{prelude::Delay, signal::Signal};
use core::ops::{Mul, Sub};
use core::time::Duration;

pub struct SmithPredictor<T, P, K>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T, TimeType = K>,
    K: TimeType,
{
    process: P,
    delay: Delay<T, K>,
    last_output: Option<T>,
}

pub struct SmithPredictorFiltered<T, P, F, K>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T, TimeType = K>,
    F: Block<Input = T, Output = T, TimeType = K>,
    K: TimeType,
{
    process: P,
    filter: F,
    delay: Delay<T, K>,
    last_output: Option<T>,
}

impl<T, P, K> SmithPredictor<T, P, K>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T, TimeType = K>,
    K: TimeType,
{
    pub fn new(process: P, delay: Duration) -> Self {
        SmithPredictor {
            process,
            delay: Delay::new(delay),
            last_output: None,
        }
    }
}

impl<T, P, F, K> SmithPredictorFiltered<T, P, F, K>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T, TimeType = K>,
    F: Block<Input = T, Output = T, TimeType = K>,
    K: TimeType,
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

impl<T, P, K> Block for SmithPredictor<T, P, K>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T, TimeType = K>,
    K: TimeType,
{
    type Input = (T, T); // (u, y)
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let (control_signal, measured_output) = input.unzip();

        let predicted_output = self.process.output(control_signal);
        let delayed_predicted_output = self.delay.output(measured_output);

        let output_diff = measured_output - delayed_predicted_output;

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

impl<T, P, F, K> Block for SmithPredictorFiltered<T, P, F, K>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
    P: Block<Input = T, Output = T, TimeType = K>,
    F: Block<Input = T, Output = T, TimeType = K>,
    K: TimeType,
{
    type Input = (T, T); // (u, y)
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let (control_signal, measured_output) = input.unzip();

        let predicted_output = self.process.output(control_signal);
        let delayed_predicted_output = self.delay.output(predicted_output);

        let output_diff = measured_output - delayed_predicted_output;
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
