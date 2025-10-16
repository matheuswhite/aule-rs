use crate::block::Block;
use crate::{prelude::Delay, signal::Signal};
use core::time::Duration;

pub struct SmithPredictor<P>
where
    P: Block<Input = f32, Output = f32>,
{
    process: P,
    delay: Delay,
    last_output: Option<f32>,
}

pub struct SmithPredictorFiltered<P, F>
where
    P: Block<Input = f32, Output = f32>,
    F: Block<Input = f32, Output = f32>,
{
    process: P,
    filter: F,
    delay: Delay,
    last_output: Option<f32>,
}

impl<P> SmithPredictor<P>
where
    P: Block<Input = f32, Output = f32>,
{
    pub fn new(process: P, delay: Duration) -> Self {
        SmithPredictor {
            process,
            delay: Delay::new(delay),
            last_output: None,
        }
    }
}

impl<P, F> SmithPredictorFiltered<P, F>
where
    P: Block<Input = f32, Output = f32>,
    F: Block<Input = f32, Output = f32>,
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

impl<P> Block for SmithPredictor<P>
where
    P: Block<Input = f32, Output = f32>,
{
    type Input = (f32, f32); // (u, y)
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
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
}

impl<P, F> Block for SmithPredictorFiltered<P, F>
where
    P: Block<Input = f32, Output = f32>,
    F: Block<Input = f32, Output = f32>,
{
    type Input = (f32, f32); // (u, y)
    type Output = f32;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
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
}
