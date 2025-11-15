use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;

#[derive(Clone, Debug)]
pub struct Delay<TT>
where
    TT: TimeType,
{
    delay: Duration,
    initial_value: f32,
    input_buffer: Vec<Signal<f32, TT>>,
    last_output: Option<f32>,
}

impl<TT> Delay<TT>
where
    TT: TimeType,
{
    pub fn new(delay: Duration) -> Self {
        assert!(
            delay > Duration::ZERO,
            "Delay duration must be greater than zero"
        );

        Delay {
            delay,
            initial_value: 0.0,
            input_buffer: vec![],
            last_output: None,
        }
    }

    pub fn with_initial_signal(mut self, initial_signal: Signal<f32, TT>) -> Self {
        self.initial_value = initial_signal.value;

        if self.input_buffer.is_empty() {
            self.input_buffer.push(initial_signal);
        } else {
            self.input_buffer[0] = initial_signal;
        }
        self
    }

    fn drop_invalid_inputs(&mut self, current_time: Duration) {
        while self.input_buffer.len() >= 2 {
            let second = &self.input_buffer[1];

            if current_time < second.delta.sim_time() {
                break;
            }

            self.input_buffer.remove(0);
        }
    }
}

impl<TT> Block for Delay<TT>
where
    TT: TimeType,
{
    type Input = f32;
    type Output = f32;
    type TimeType = TT;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let current_time = input.delta.sim_time();

        if self.input_buffer.is_empty() {
            let mut initial_signal = input.replace(0.0);
            initial_signal.delta.reset_dt();
            initial_signal.delta.reset_sim_time();
            initial_signal.delta += (self.delay, self.delay);

            self.input_buffer.push(initial_signal);
        }

        let mut input_delayed = input;
        input_delayed.delta += self.delay;
        self.input_buffer.push(input_delayed);

        /* # Current time before delay */
        if current_time < self.delay {
            let output = input.replace(self.initial_value);
            self.last_output = Some(output.value);
            return output;
        }

        /* # Current time after delay */
        self.drop_invalid_inputs(current_time);

        let mut first_input = &self.input_buffer[0];
        let mut second_input = self.input_buffer.get(1).unwrap_or(&input);

        if current_time < first_input.delta.sim_time() {
            second_input = first_input;
            first_input = &input;
        }

        let gama = if first_input.delta.sim_time().as_secs_f32()
            != second_input.delta.sim_time().as_secs_f32()
        {
            let num = current_time.as_secs_f32() - first_input.delta.sim_time().as_secs_f32();
            let dem = second_input.delta.sim_time().as_secs_f32()
                - first_input.delta.sim_time().as_secs_f32();
            num / dem
        } else {
            0.0
        };
        assert!(
            (0.0..=1.0).contains(&gama),
            "gama must be in [0, 1], got {}",
            gama
        );

        let output_delta = first_input.delta.merge(second_input.delta);
        let output_value = first_input.value * (1.0 - gama) + second_input.value * gama;
        let output = Signal {
            value: output_value,
            delta: output_delta,
        };
        self.last_output = Some(output.value);
        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        self.input_buffer.clear();
        self.last_output = None;
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::{prelude::*, time::Continuous};
    use alloc::vec::Vec;
    use core::time::Duration;

    #[test]
    fn test_delay_happy_way() {
        let time = Time::continuous(1.0, 3.0);
        let mut delay = Delay::new(Duration::from_secs(2));

        let mut output_signals = Vec::new();
        for dt in time {
            let input_signal = dt.map(|_| 1.0);
            let output = delay.output(input_signal);
            output_signals.push(output);
        }

        // Output signals will be:
        // 1st input (t=1s): output = 0.0 (initial signal)
        // 2nd input (t=2s): output = 0.0 (initial signal)
        // 3rd input (t=3s): output = 1.0 (first input delayed by 2s)
        assert_eq!(output_signals[0].value, 0.0);
        assert_eq!(output_signals[1].value, 0.0);
        assert_eq!(output_signals[2].value, 1.0);
    }

    #[test]
    #[should_panic(expected = "Delay duration must be greater than zero")]
    fn test_delay_zero_duration() {
        let _delay = Delay::<Continuous>::new(Duration::from_secs(0));
    }

    #[test]
    fn test_delay_half_dt() {
        let mut time = Time::continuous(1.0, 6.0);
        let mut delay = Delay::new(Duration::from_secs(2));

        let mut input_signals = Vec::new();
        for i in 0..3 {
            let dt = time.next().unwrap();
            input_signals.push(dt.map(|_| (i + 1) as f32));
        }
        time.set_dt(0.5);
        for i in 0..6 {
            let dt = time.next().unwrap();
            input_signals.push(dt.map(|_| 3.0 + (i + 1) as f32));
        }
        let mut output_signals = Vec::new();
        for input in input_signals {
            let output = delay.output(input);
            output_signals.push(output);
        }
        // Output signals will be:
        // 1st input (t=1s): output = 0.0 (initial signal)
        // 2nd input (t=2s): output = 0.0 (initial signal)
        // 3rd input (t=3s): output = 1.0 (first input delayed by 2s)
        // 4th input (t=3.5s): output = 1.5 (interpolated between 1.0 and 2.0)
        // 5th input (t=4s): output = 2.0 (second input delayed by 2s)
        // 6th input (t=4.5s): output = 2.5 (interpolated between 2.0 and 3.0)
        // 7th input (t=5s): output = 3.0 (third input delayed by 2s)
        // 8th input (t=5.5s): output = 4.0 (forth input delayed by 2s)
        // 9th input (t=6s): output = 5.0 (fifth input delayed by 2s)
        assert_eq!(output_signals[0].value, 0.0);
        assert_eq!(output_signals[1].value, 0.0);
        assert_eq!(output_signals[2].value, 1.0);
        assert_eq!(output_signals[3].value, 1.5);
        assert_eq!(output_signals[4].value, 2.0);
        assert_eq!(output_signals[5].value, 2.5);
        assert_eq!(output_signals[6].value, 3.0);
        assert_eq!(output_signals[7].value, 4.0);
        assert_eq!(output_signals[8].value, 5.0);
    }

    #[test]
    fn test_delay_large_dt() {
        let mut time = Time::continuous(5.0, 10.0);
        let mut delay = Delay::new(Duration::from_secs(2));
        let mut input_signals = Vec::new();

        let large_input_signal = time.next().unwrap().map(|_| 1.0);
        input_signals.push(large_input_signal);

        time.set_dt(1.0);
        let input_signal = time.next().unwrap().map(|_| 1.0);
        input_signals.push(input_signal);
        let input_signal = time.next().unwrap().map(|_| 1.0);
        input_signals.push(input_signal);

        time.set_dt(0.5);
        for i in 0..6 {
            let half_input_signal = time.next().unwrap().map(|_| 2.0 + i as f32);
            input_signals.push(half_input_signal);
        }

        let mut output_signals = Vec::new();
        for input in input_signals {
            let output = delay.output(input);
            output_signals.push(output);
        }
        // Output signals will be:
        // 1st input (t=5s): output = 0.6 (interpolated between 0.0 and 1.0)
        // 2nd input (t=6s): output = 0.8 (interpolated between 0.0 and 1.0)
        // 3rd input (t=7s): output = 1.0 (first input delayed by 2s)
        // 4th input (t=7.5s): output = 1.0 (interpolated between 1.0 and 1.0)
        // 5th input (t=8s): output = 1.0 (second input delayed by 2s)
        // 6th input (t=8.5s): output = 1.0 (interpolated between 1.0 and 1.0)
        // 7th input (t=9s): output = 1.0 (third input delayed by 2s)
        // 8th input (t=9.5s): output = 2.0 (forth input delayed by 2s)
        // 9th input (t=10s): output = 3.0 (fifth input delayed by 2s)
        assert_eq!(output_signals[0].value, 0.6);
        assert_eq!(output_signals[1].value, 0.8);
        assert_eq!(output_signals[2].value, 1.0);
        assert_eq!(output_signals[3].value, 1.0);
        assert_eq!(output_signals[4].value, 1.0);
        assert_eq!(output_signals[5].value, 1.0);
        assert_eq!(output_signals[6].value, 1.0);
        assert_eq!(output_signals[7].value, 2.0);
        assert_eq!(output_signals[8].value, 3.0);
    }

    #[test]
    fn test_delay_large_dt_in_middle() {
        let mut time = Time::continuous(5.0, 11.0);
        let mut delay = Delay::new(Duration::from_secs(2));

        let large_input_signal =
            |v, time: &mut Time<Continuous>| time.next().unwrap().map(|_| v as f32);

        time.set_dt(1.0);
        let input_signal = |v, time: &mut Time<Continuous>| time.next().unwrap().map(|_| v as f32);

        time.set_dt(0.5);
        let half_signal = |v, time: &mut Time<Continuous>| time.next().unwrap().map(|_| v as f32);

        let mut output_signals = Vec::new();
        for i in 0..3 {
            let input = input_signal(i + 1, &mut time);
            output_signals.push(delay.output(input));
        }
        output_signals.push(delay.output(large_input_signal(4, &mut time)));
        for i in 0..6 {
            let input = half_signal(5 + i, &mut time);
            output_signals.push(delay.output(input));
        }
        // Output signals will be:
        // 1st input (t=1s): output = 0.0 (initial signal)
        // 2nd input (t=2s): output = 0.0 (initial signal)
        // 3rd input (t=3s): output = 1.0 (first input delayed by 2s)
        // 4th input (t=8s): output = 3.6 (interpolated between 3.0 and 4.0)
        // 5th input (t=8.5s): output = 3.7 (interpolated between 3.0 and 4.0)
        // 6th input (t=9s): output = 3.8 (interpolated between 3.0 and 4.0)
        // 7th input (t=9.5s): output = 3.9 (interpolated between 3.0 and 4.0)
        // 8th input (t=10s): output = 4.0 (forth input delayed by 2s)
        // 9th input (t=10.5s): output = 5.0 (fifth input delayed by 2s)
        // 10th input (t=11s): output = 6.0 (sixth input delayed by 2s)
    }
}
