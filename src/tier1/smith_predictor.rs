use crate::block::Block;
use crate::time::TimeType;
use crate::{prelude::Delay, signal::Signal};
use core::time::Duration;

pub struct SmithPredictor<P, TT>
where
    P: Block<Input = f32, Output = f32, TimeType = TT>,
    TT: TimeType,
{
    process: P,
    delay: Delay<TT>,
    last_output: Option<f32>,
}

pub struct SmithPredictorFiltered<P, F, TT>
where
    P: Block<Input = f32, Output = f32, TimeType = TT>,
    F: Block<Input = f32, Output = f32, TimeType = TT>,
    TT: TimeType,
{
    process: P,
    filter: F,
    delay: Delay<TT>,
    last_output: Option<f32>,
}

impl<P, TT> SmithPredictor<P, TT>
where
    P: Block<Input = f32, Output = f32, TimeType = TT>,
    TT: TimeType,
{
    pub fn new(process: P, delay: Duration) -> Self {
        SmithPredictor {
            process,
            delay: Delay::new(delay),
            last_output: None,
        }
    }
}

impl<P, F, TT> SmithPredictorFiltered<P, F, TT>
where
    P: Block<Input = f32, Output = f32, TimeType = TT>,
    F: Block<Input = f32, Output = f32, TimeType = TT>,
    TT: TimeType,
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

impl<P, TT> Block for SmithPredictor<P, TT>
where
    P: Block<Input = f32, Output = f32, TimeType = TT>,
    TT: TimeType,
{
    type Input = (f32, f32); // (u, y)
    type Output = f32;
    type TimeType = TT;

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
}

impl<P, F, TT> Block for SmithPredictorFiltered<P, F, TT>
where
    P: Block<Input = f32, Output = f32, TimeType = TT>,
    F: Block<Input = f32, Output = f32, TimeType = TT>,
    TT: TimeType,
{
    type Input = (f32, f32); // (u, y)
    type Output = f32;
    type TimeType = TT;

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
}
