use crate::block::{AsBlock, Block};
use crate::signal::Signal;
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;

#[derive(Copy, Clone)]
struct InputBuffered {
    instant: Duration,
    signal: Signal,
}

/// A block that introduces a time delay to the input signal.
/// The output signal is a delayed version of the input signal, with linear interpolation
/// between input samples to ensure smooth transitions.
///
/// # Examples:
/// ```
/// use aule::prelude::*;
/// use core::time::Duration;
///
/// let mut delay = Delay::new(Duration::from_secs(2), Signal::from((Duration::from_secs(1), 0.0)));
/// let input_signal = Signal { dt: Duration::from_secs(1), value: 1.0 };
/// let mut output_signals = Vec::new();
/// for _ in 0..3 {
///     let output = delay.output(input_signal);
///     output_signals.push(output);
/// }
/// // Output signals will be:
/// // 1st input (t=1s): output = 0.0 (initial signal)
/// // 2nd input (t=2s): output = 0.0 (initial signal)
/// // 3rd input (t=3s): output = 1.0 (first input delayed by 2s)
/// assert_eq!(output_signals[0].value, 0.0);
/// assert_eq!(output_signals[1].value, 0.0);
/// assert_eq!(output_signals[2].value, 1.0);
/// ```
pub struct Delay {
    delay: Duration,
    current_time: Duration,
    initial_signal: Signal,
    input_buffer: Vec<InputBuffered>,
    last_output: Option<Signal>,
}

impl Delay {
    /// Creates a new `Delay` block with the specified delay duration and initial signal.
    /// The `initial_signal` is used as the output signal until the delay period has elapsed.
    ///
    /// # Arguments
    /// * `delay` - The duration of the delay to be introduced.
    /// * `initial_signal` - The signal to output before the delay period has elapsed.
    /// # Returns
    /// A new instance of the `Delay` block.
    ///
    /// # Panics
    /// This function will panic if the `delay` is zero.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use core::time::Duration;
    ///
    /// let delay = Delay::new(Duration::from_secs(2), Signal::from((Duration::from_secs(1), 0.0)));
    /// ```
    pub fn new(delay: Duration, initial_signal: Signal) -> Self {
        assert!(
            delay > Duration::ZERO,
            "Delay duration must be greater than zero"
        );

        Delay {
            delay,
            current_time: Duration::ZERO,
            initial_signal,
            input_buffer: vec![(delay, initial_signal).into()],
            last_output: None,
        }
    }

    fn drop_invalid_signals(&mut self, current_time: Duration) {
        if self.input_buffer.len() < 2 {
            return;
        }

        let mut _i0 = self.input_buffer[0].instant;
        let mut i1 = self.input_buffer[1].instant;

        while i1 < current_time {
            self.input_buffer.remove(0);

            if self.input_buffer.len() < 2 {
                return;
            }

            _i0 = self.input_buffer[0].instant;
            i1 = self.input_buffer[1].instant;
        }
    }

    fn find_input_signal(&self, time: Duration) -> Option<&InputBuffered> {
        self.input_buffer
            .iter()
            .position(|InputBuffered { instant, .. }| *instant == time)
            .map(|pos| &self.input_buffer[pos])
    }
}

impl Block for Delay {
    /// Processes the input signal and returns the delayed output signal.
    /// The output signal is determined based on the current time and the specified delay.
    /// If the current time is less than or equal to the delay, the initial signal is returned.
    /// Otherwise, the output is computed using linear interpolation between the two nearest
    /// input signals in the buffer.
    /// The input signal is also added to the buffer with its corresponding future instant.
    ///
    /// # Arguments
    /// * `input` - The input signal to be processed.
    /// # Returns
    /// The output signal after applying the delay.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use core::time::Duration;
    ///
    /// let mut delay = Delay::new(Duration::from_secs(2), Signal::from((Duration::from_secs(1), 0.0)));
    /// let input_signal = Signal { dt: Duration::from_secs(1), value: 1.0 };
    /// let output1 = delay.output(input_signal);
    /// let output2 = delay.output(input_signal);
    /// let output3 = delay.output(input_signal);
    /// assert_eq!(output1.value, 0.0); // Initial signal
    /// assert_eq!(output2.value, 0.0); // Still initial signal
    /// assert_eq!(output3.value, 1.0); // First input delayed by 2s
    /// ```
    fn output(&mut self, input: Signal) -> Signal {
        self.current_time += input.dt;

        self.input_buffer.push((self.current_time, input).into());

        let ref_time = if self.current_time > self.delay {
            self.current_time - self.delay
        } else {
            Duration::ZERO
        };

        let output = if self.current_time <= self.delay {
            self.initial_signal
        } else {
            self.drop_invalid_signals(ref_time);

            match self.input_buffer.len() {
                0 => unreachable!("input buffer should always have at least one element"),
                1 => return self.input_buffer[0].signal,
                _ => {
                    if let Some(input) = self.find_input_signal(ref_time) {
                        input.signal
                    } else {
                        let (input0, input1) = (self.input_buffer[0], self.input_buffer[1]);

                        let gamma = (ref_time - input0.instant).as_secs_f32()
                            / (input1.instant - input0.instant).as_secs_f32();
                        assert!(
                            (0.0 <= gamma) && (gamma <= 1.0),
                            "gamma must be in [0, 1], got {}",
                            gamma
                        );

                        Signal {
                            dt: input.dt,
                            value: (1.0 - gamma) * input0.signal.value
                                + gamma * input1.signal.value,
                        }
                    }
                }
            }
        };

        self.last_output = Some(output);

        output
    }

    /// Returns the last output signal produced by the block, if any.
    /// If no output has been produced yet, it returns `None`.
    ///
    /// # Returns
    /// An `Option<Signal>` containing the last output signal or `None` if no output has been produced.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use core::time::Duration;
    ///
    /// let mut delay = Delay::new(Duration::from_secs(2), Signal::from((Duration::from_secs(1), 0.0)));
    /// assert_eq!(delay.last_output(), None);
    /// let input_signal = Signal { dt: Duration::from_secs(1), value: 1.0 };
    /// let _ = delay.output(input_signal);
    /// assert_eq!(delay.last_output().unwrap().value, 0.0); // Initial signal
    /// ```
    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl From<(Duration, Signal)> for InputBuffered {
    /// Converts a tuple of (Duration, Signal) into an InputBuffered struct.
    ///
    /// # Arguments
    /// * `instant` - The time instant associated with the signal.
    /// * `signal` - The signal value.
    /// # Returns
    /// An instance of `InputBuffered` containing the provided instant and signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use core::time::Duration;
    ///
    /// let input_buffered: InputBuffered = (Duration::from_secs(1), Signal { dt: Duration::from_secs(1), value: 1.0 }).into();
    /// assert_eq!(input_buffered.instant, Duration::from_secs(1));
    /// assert_eq!(input_buffered.signal.value, 1.0);
    /// ```
    fn from((instant, signal): (Duration, Signal)) -> Self {
        Self { instant, signal }
    }
}

impl AsBlock for Delay {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use alloc::vec::Vec;
    use core::time::Duration;

    #[test]
    fn test_delay_happy_way() {
        let mut delay = Delay::new(
            Duration::from_secs(2),
            Signal::from((Duration::from_secs(1), 0.0)),
        );
        let input_signal = Signal {
            dt: Duration::from_secs(1),
            value: 1.0,
        };
        let mut output_signals = Vec::new();
        for _ in 0..3 {
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
        Delay::new(
            Duration::from_secs(0),
            Signal::from((Duration::from_secs(1), 0.0)),
        );
    }

    #[test]
    fn test_delay_half_dt() {
        let mut delay = Delay::new(
            Duration::from_secs(2),
            Signal::from((Duration::from_secs(1), 0.0)),
        );
        let mut input_signals = Vec::new();
        for i in 0..3 {
            input_signals.push(Signal {
                dt: Duration::from_millis(1000),
                value: (i + 1) as f32,
            });
        }
        for i in 0..6 {
            input_signals.push(Signal {
                dt: Duration::from_millis(500),
                value: 3.0 + (i + 1) as f32,
            });
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
        let mut delay = Delay::new(
            Duration::from_secs(2),
            Signal::from((Duration::from_secs(1), 0.0)),
        );
        let large_input_signal = Signal {
            dt: Duration::from_secs(5),
            value: 1.0,
        };
        let input_signal = Signal {
            dt: Duration::from_secs(1),
            value: 1.0,
        };
        let mut output_signals = Vec::new();
        output_signals.push(delay.output(large_input_signal));
        output_signals.push(delay.output(input_signal));
        output_signals.push(delay.output(input_signal));
        for i in 0..6 {
            let half_input_signal = Signal {
                dt: Duration::from_millis(500),
                value: 2.0 + i as f32,
            };
            output_signals.push(delay.output(half_input_signal));
        }
        // Output signals will be:
        // 1st input (t=5s): output = 0.33 (interpolated between 0.0 and 1.0)
        // 2nd input (t=6s): output = 0.66 (interpolated between 0.0 and 1.0)
        // 3rd input (t=7s): output = 1.0 (first input delayed by 2s)
        // 4th input (t=7.5s): output = 1.0 (interpolated between 1.0 and 1.0)
        // 5th input (t=8s): output = 1.0 (second input delayed by 2s)
        // 6th input (t=8.5s): output = 1.0 (interpolated between 1.0 and 1.0)
        // 7th input (t=9s): output = 1.0 (third input delayed by 2s)
        // 8th input (t=9.5s): output = 2.0 (forth input delayed by 2s)
        // 9th input (t=10s): output = 3.0 (fifth input delayed by 2s)
        assert!(
            (output_signals[0].value - 0.33).abs() < 0.01,
            "{}",
            output_signals[0].value
        );
        assert!(
            (output_signals[1].value - 0.66).abs() < 0.01,
            "{}",
            output_signals[1].value
        );
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
        let mut delay = Delay::new(
            Duration::from_secs(2),
            Signal::from((Duration::from_secs(1), 0.0)),
        );
        let large_input_signal = |v: i32| Signal {
            dt: Duration::from_secs(5),
            value: v as f32,
        };
        let input_signal = |v: i32| Signal {
            dt: Duration::from_secs(1),
            value: v as f32,
        };
        let half_signal = |v: i32| Signal {
            dt: Duration::from_millis(500),
            value: v as f32,
        };
        let mut output_signals = Vec::new();
        for i in 0..3 {
            let input = input_signal(i + 1);
            output_signals.push(delay.output(input));
        }
        output_signals.push(delay.output(large_input_signal(4)));
        for i in 0..6 {
            let input = half_signal(5 + i);
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
