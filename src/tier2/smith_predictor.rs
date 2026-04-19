use crate::block::Block;
use crate::prelude::{Delay, SimulationState};
use crate::signal::Signal;
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
    pub control_signal: T,
    pub measured_output: T,
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

impl<T> SmithPredictorInput<T>
where
    T: Zero + Copy + Mul<f64, Output = T> + Sub<Output = T>,
{
    pub fn from_signals(control_signal: Signal<T>, measured_output: Signal<T>) -> Signal<Self> {
        Signal {
            value: Self {
                control_signal: control_signal.value,
                measured_output: measured_output.value,
            },
            sim_state: control_signal.sim_state.merge(measured_output.sim_state),
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

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let predicted_output = self.process.block(input.control_signal, sim_state);
        let delayed_predicted_output = self.delay.block(input.measured_output, sim_state);

        let output_diff = input.measured_output - delayed_predicted_output;

        let output = predicted_output + output_diff;
        self.last_output = Some(output);
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

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let predicted_output = self.process.block(input.control_signal, sim_state);
        let delayed_predicted_output = self.delay.block(predicted_output, sim_state);

        let output_diff = input.measured_output - delayed_predicted_output;
        let output_diff_filtered = self.filter.block(output_diff, sim_state);

        let output = predicted_output + output_diff_filtered;
        self.last_output = Some(output);
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
