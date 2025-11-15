use crate::{
    block::Block,
    prelude::{Delay, Joinable, Plotter, Step},
    signal::{Pack, Signal},
    time::{Continuous, Time},
};
use core::{fmt::Display, ops::Mul, time::Duration};
use num_traits::{Float, FromPrimitive};
use std::{string::ToString, vec::Vec};

pub struct StepResponse<B, T>
where
    B: Block<Input = T, Output = T, TimeType = Continuous>,
    T: FromPrimitive + Float + Display + Mul<f64, Output = T>,
{
    time_start: Duration,
    time_end: Duration,
    time_step: Duration,
    without_plot: bool,
    tolerance: T,
    block: B,
}

#[derive(Debug)]
pub struct StepInfo<T> {
    peak: Signal<T, Continuous>,
    rise_time: Duration,
    settling_time: Duration,
    transient_time: Duration,
    y_final: Signal<T, Continuous>,
    overshoot: f64,
    undershoot: f64,
    y: Vec<Signal<T, Continuous>>,
}

impl<B, T> StepResponse<B, T>
where
    B: Block<Input = T, Output = T, TimeType = Continuous>,
    T: FromPrimitive + Float + Display + Mul<f64, Output = T>,
{
    pub fn new(block: B) -> Self {
        Self {
            time_start: Duration::default(),
            time_end: Duration::from_secs(100),
            time_step: Duration::from_secs_f32(1e-3),
            without_plot: false,
            tolerance: T::from_f64(1e-4).unwrap(),
            block,
        }
    }

    pub fn with_start_time(mut self, time: Duration) -> Self {
        self.time_start = time;
        self
    }

    pub fn with_end_time(mut self, time: Duration) -> Self {
        self.time_end = time;
        self
    }

    pub fn with_step_time(mut self, time: Duration) -> Self {
        self.time_step = time;
        self
    }

    pub fn without_plot(mut self) -> Self {
        self.without_plot = true;
        self
    }

    pub fn with_tolerance(mut self, tol: T) -> Self {
        self.tolerance = tol;
        self
    }

    pub fn run(&mut self) -> StepInfo<T> {
        let time = Time::continuous(self.time_step.as_secs_f32(), self.time_end.as_secs_f32());
        let mut step = Step::default();
        let delay_time = Duration::from_secs_f32(0.1);
        let mut delay = Delay::new(delay_time);

        let mut y = Vec::new();
        let mut inputs = Vec::new();
        let mut peak: Option<Signal<T, Continuous>> = None;
        let mut y_final = None;
        let mut reach_one = false;
        let mut minimum: Option<Signal<T, Continuous>> = None;

        self.block.reset();
        for dt in time {
            let input = step.output(dt);
            inputs.push(input);

            let output = self.block.output(input);
            let delayed_output = delay.output(output);

            if output.value >= T::one() {
                reach_one = true;
            }

            if reach_one && (minimum.is_none() || minimum.unwrap().value > output.value) {
                minimum = Some(output);
            }

            if dt.delta.sim_time() < self.time_start {
                continue;
            }

            if dt.delta.sim_time() > delay_time {
                let percentage = (T::one() - (output.value / delayed_output.value)).abs();
                if percentage < self.tolerance {
                    break;
                }
            }

            if peak.is_none() || peak.unwrap().value < output.value {
                peak = Some(output);
            }
            y_final = Some(output);
            y.push(output);

            if self.without_plot {
                continue;
            }
        }

        let overshoot = (peak.unwrap().value - T::one()) / T::one();
        let overshoot = overshoot.to_f64().unwrap();

        let undershoot = (T::one() - minimum.unwrap().value) / T::one();
        let undershoot = undershoot.to_f64().unwrap();

        let mut plotter = Plotter::new("Step Response".to_string());

        for (i, y) in inputs.iter().zip(y.iter()) {
            plotter.output([*i, *y].pack());
        }

        plotter.display();
        plotter.join();

        StepInfo {
            peak: peak.expect("No peak found. Please check the step response settings."),
            rise_time: Duration::default(),
            settling_time: Duration::default(),
            transient_time: Duration::default(),
            y_final: y_final
                .expect("No final value found. Please check the step response settings."),
            overshoot,
            undershoot,
            y,
        }
    }
}

impl<T> StepInfo<T> {
    pub fn y(&self) -> &[Signal<T, Continuous>] {
        &self.y
    }
}

impl<T> Display for StepInfo<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Step Response Info:\n\
            Peak: {:.4} at {:.4}s\n\
            Final Value: {:.4} at {:.4}s\n\
            Rise Time: {:.4} s\n\
            Settling Time: {:.4} s\n\
            Transient Time: {:.4} s\n\
            Overshoot: {:.2}%\n\
            Undershoot: {:.2}%",
            self.peak.value,
            self.peak.delta.sim_time().as_secs_f64(),
            self.y_final.value,
            self.y_final.delta.sim_time().as_secs_f64(),
            self.rise_time.as_secs_f64(),
            self.settling_time.as_secs_f64(),
            self.transient_time.as_secs_f64(),
            self.overshoot * 100.0,
            self.undershoot * 100.0
        )
    }
}
