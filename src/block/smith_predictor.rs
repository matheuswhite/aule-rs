use crate::{
    prelude::{AsMIMO, Delay, MIMO, SISO},
    signal::Signal,
};
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;

pub struct SmithPredictor<P>
where
    P: SISO,
{
    process: P,
    delay: Delay,
    last_output: Option<Signal>,
}

pub struct SmithPredictorFiltered<P, F>
where
    P: SISO,
    F: SISO,
{
    process: P,
    filter: F,
    delay: Delay,
    last_output: Option<Signal>,
}

impl<P> SmithPredictor<P>
where
    P: SISO,
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
    P: SISO,
    F: SISO,
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

impl<P> MIMO for SmithPredictor<P>
where
    P: SISO,
{
    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
        let control_signal = input[0];
        let measured_output = input[1];

        let predicted_output = self.process.output(control_signal);
        let delayed_predicted_output = self.delay.output(predicted_output);

        let output_diff = measured_output - delayed_predicted_output;

        let output = predicted_output + output_diff;
        self.last_output = Some(output);
        vec![output]
    }

    fn last_output(&self) -> Option<Vec<Signal>> {
        self.last_output.map(|s| vec![s])
    }

    fn dimensions(&self) -> (usize, usize) {
        (2, 1)
    }
}

impl<P, F> MIMO for SmithPredictorFiltered<P, F>
where
    P: SISO,
    F: SISO,
{
    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
        let control_signal = input[0];
        let measured_output = input[1];

        let predicted_output = self.process.output(control_signal);
        let delayed_predicted_output = self.delay.output(predicted_output);

        let output_diff = measured_output - delayed_predicted_output;
        let output_diff_filtered = self.filter.output(output_diff);

        let output = predicted_output + output_diff_filtered;
        self.last_output = Some(output);
        vec![output]
    }

    fn last_output(&self) -> Option<Vec<Signal>> {
        self.last_output.map(|s| vec![s])
    }

    fn dimensions(&self) -> (usize, usize) {
        (2, 1)
    }
}

impl<P> AsMIMO for SmithPredictor<P> where P: SISO + 'static {}
impl<P, F> AsMIMO for SmithPredictorFiltered<P, F>
where
    P: SISO + 'static,
    F: SISO + 'static,
{
}
