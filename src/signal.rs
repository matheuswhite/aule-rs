use crate::block::mimo::MIMO;
#[cfg(feature = "alloc")]
use crate::output::Output;
use crate::{block::siso::SISO, error::ErrorMetric};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::{Add, Div, Mul, Neg, Shr, Sub};
use core::time::Duration;

/// The `Signal` struct represents a signal with a value and a time step.
/// It is used to encapsulate the data that flows through blocks in a block-based system.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// let signal = Signal {
///     value: 1.0,
///     dt: Duration::from_secs(1),
/// };
/// assert_eq!(signal.value, 1.0);
/// assert_eq!(signal.dt, Duration::from_secs(1));
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Signal {
    pub value: f32,
    pub dt: Duration,
}

impl From<Duration> for Signal {
    /// Creates a `Signal` from a `Duration`, initializing the value to 0.0.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let dt = Duration::from_secs(1);
    /// let signal: Signal = dt.into();
    /// assert_eq!(signal.value, 0.0);
    /// assert_eq!(signal.dt, dt);
    /// ```
    fn from(dt: Duration) -> Self {
        Signal { value: 0.0, dt }
    }
}

impl From<(f32, Duration)> for Signal {
    /// Creates a `Signal` from a tuple of value and duration.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal: Signal = (1.0, Duration::from_secs(1)).into();
    /// assert_eq!(signal.value, 1.0);
    /// assert_eq!(signal.dt, Duration::from_secs(1));
    /// ```
    fn from((value, dt): (f32, Duration)) -> Self {
        Signal { value, dt }
    }
}

impl From<(Duration, f32)> for Signal {
    /// Creates a `Signal` from a tuple of duration and value.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal: Signal = (Duration::from_secs(1), 1.0).into();
    /// assert_eq!(signal.value, 1.0);
    /// assert_eq!(signal.dt, Duration::from_secs(1));
    /// ```
    fn from((dt, value): (Duration, f32)) -> Self {
        Signal { value, dt }
    }
}

impl From<(f32, f32)> for Signal {
    /// Creates a `Signal` from a tuple of value and delta time in seconds.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal: Signal = (1.0, 1.0).into();
    /// assert_eq!(signal.value, 1.0);
    /// assert_eq!(signal.dt, Duration::from_secs_f32(1.0));
    /// ```
    fn from((value, dt): (f32, f32)) -> Self {
        Signal {
            value,
            dt: Duration::from_secs_f32(dt),
        }
    }
}

impl Neg for Signal {
    type Output = Self;

    /// Negates the signal's value.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let negated_signal = -signal;
    /// assert_eq!(negated_signal.value, -1.0);
    /// assert_eq!(negated_signal.dt, Duration::from_secs(1));
    /// ```
    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            dt: self.dt,
        }
    }
}

impl Sub for Signal {
    type Output = Self;

    /// Subtracts another signal from this signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal1 = Signal { value: 5.0, dt: Duration::from_secs(1) };
    /// let signal2 = Signal { value: 3.0, dt: Duration::from_secs(1) };
    /// let result = signal1 - signal2;
    /// assert_eq!(result.value, 2.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Sub<f32> for Signal {
    type Output = Self;

    /// Subtracts a scalar from the signal's value.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal = Signal { value: 5.0, dt: Duration::from_secs(1) };
    /// let result = signal - 3.0;
    /// assert_eq!(result.value, 2.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn sub(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value - rhs,
            dt: self.dt,
        }
    }
}

impl Sub<Option<Signal>> for Signal {
    type Output = Self;

    /// Subtracts an optional signal from this signal.
    /// If the optional signal is `None`, it returns the original signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal1 = Signal { value: 5.0, dt: Duration::from_secs(1) };
    /// let signal2 = Some(Signal { value: 3.0, dt: Duration::from_secs(1) });
    /// let result = signal1 - signal2;
    /// assert_eq!(result.value, 2.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    ///
    /// let signal3 = None;
    /// let result_none = signal1 - signal3;
    /// assert_eq!(result_none.value, 5.0);
    /// assert_eq!(result_none.dt, Duration::from_secs(1));
    /// ```
    fn sub(self, rhs: Option<Signal>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl Add for Signal {
    type Output = Self;

    /// Adds another signal to this signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal1 = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let signal2 = Signal { value: 3.0, dt: Duration::from_secs(1) };
    /// let result = signal1 + signal2;
    /// assert_eq!(result.value, 5.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Add<f32> for Signal {
    type Output = Self;

    /// Adds a scalar to the signal's value.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let result = signal + 3.0;
    /// assert_eq!(result.value, 5.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn add(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value + rhs,
            dt: self.dt,
        }
    }
}

impl Add<Option<Signal>> for Signal {
    type Output = Self;

    /// Adds an optional signal to this signal.
    /// If the optional signal is `None`, it returns the original signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal1 = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let signal2 = Some(Signal { value: 3.0, dt: Duration::from_secs(1) });
    /// let result = signal1 + signal2;
    /// assert_eq!(result.value, 5.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    ///
    /// let signal3 = None;
    /// let result_none = signal1 + signal3;
    /// assert_eq!(result_none.value, 2.0);
    /// assert_eq!(result_none.dt, Duration::from_secs(1));
    /// ```
    fn add(self, rhs: Option<Signal>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl Div for Signal {
    type Output = Self;

    /// Divides this signal by another signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal1 = Signal { value: 6.0, dt: Duration::from_secs(1) };
    /// let signal2 = Signal { value: 3.0, dt: Duration::from_secs(1) };
    /// let result = signal1 / signal2;
    /// assert_eq!(result.value, 2.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Div<f32> for Signal {
    type Output = Self;

    /// Divides the signal's value by a scalar.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal = Signal { value: 6.0, dt: Duration::from_secs(1) };
    /// let result = signal / 3.0;
    /// assert_eq!(result.value, 2.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn div(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value / rhs,
            dt: self.dt,
        }
    }
}

impl Mul for Signal {
    type Output = Self;

    /// Multiplies two signals together.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal1 = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let signal2 = Signal { value: 3.0, dt: Duration::from_secs(1) };
    /// let result = signal1 * signal2;
    /// assert_eq!(result.value, 6.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Mul<f32> for Signal {
    type Output = Self;

    /// Multiplies the signal's value by a scalar.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// let signal = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let result = signal * 3.0;
    /// assert_eq!(result.value, 6.0);
    /// assert_eq!(result.dt, Duration::from_secs(1));
    /// ```
    fn mul(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value * rhs,
            dt: self.dt,
        }
    }
}

impl Mul<&mut dyn SISO> for Signal {
    type Output = Signal;

    /// Multiplies the signal with a mutable reference to a block, producing an output signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyBlock;
    ///
    /// impl SISO for MyBlock {
    ///     fn output(&mut self, input: Signal) -> Signal {
    ///         Signal {
    ///             value: input.value * 2.0, // Example processing
    ///             dt: input.dt,
    ///         }
    ///     }
    ///
    ///    fn last_output(&self) -> Option<Signal> {
    ///        None // Example implementation, could return the last processed signal
    ///   }
    /// }
    ///
    /// impl AsSISO for MyBlock {}
    ///
    /// let mut block = MyBlock;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = input_signal * block.as_siso();
    /// assert_eq!(output_signal.value, 2.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn mul(self, block: &mut dyn SISO) -> Self::Output {
        block.output(self)
    }
}

impl<const I: usize> Mul<&mut dyn MIMO> for [Signal; I] {
    type Output = Vec<Signal>;

    /// Multiplies an array of input signals with a mutable reference to a MIMO block, producing an array of output signals.
    /// This allows for processing multiple input signals through the MIMO block.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///   fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///     [Signal {
    ///       value: input[0].value + 1.0, // Example processing for first output
    ///       dt: input[0].dt,
    ///     }, Signal {
    ///       value: input[1].value * 2.0, // Example processing for second output
    ///       dt: input[1].dt,
    ///     }].to_vec()
    ///   }
    ///
    ///   fn last_output(&self) -> Option<Vec<Signal>> {
    ///     None // Example implementation, could return the last processed signals
    ///   }
    ///
    ///   fn dimensions(&self) -> (usize, usize) {
    ///       (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///   }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
    /// let output_signals = input_signals * block.as_mimo();
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output(self.to_vec())
    }
}

impl Mul<&mut dyn MIMO> for &[Signal] {
    type Output = Vec<Signal>;

    /// Multiplies a slice of input signals with a mutable reference to a MIMO block, producing an array of output signals.
    /// This allows for processing multiple input signals through the MIMO block.
    /// The slice must have a length equal to the number of inputs expected by the MIMO block.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///   fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///     [Signal {
    ///       value: input[0].value + 1.0, // Example processing for first output
    ///       dt: input[0].dt,
    ///     }, Signal {
    ///       value: input[1].value * 2.0, // Example processing for second output
    ///       dt: input[1].dt,
    ///     }].to_vec()
    ///   }
    ///
    ///   fn last_output(&self) -> Option<Vec<Signal>> {
    ///     None // Example implementation, could return the last processed signals
    ///   }
    ///
    ///   fn dimensions(&self) -> (usize, usize) {
    ///       (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///   }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
    /// let output_signals = &input_signals[..] * block.as_mimo();
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output(self.to_vec())
    }
}

impl Mul<&mut dyn MIMO> for Signal {
    type Output = Vec<Signal>;

    /// Multiplies a single input signal with a mutable reference to a MIMO block that accepts one input, producing an array of output signals.
    /// This allows for processing a single input signal through the MIMO block.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///   fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///     [Signal {
    ///       value: input[0].value + 1.0, // Example processing for first output
    ///       dt: input[0].dt,
    ///     }, Signal {
    ///       value: input[0].value * 2.0, // Example processing for second output
    ///       dt: input[0].dt,
    ///     }].to_vec()
    ///   }
    ///
    ///   fn last_output(&self) -> Option<Vec<Signal>> {
    ///     None // Example implementation, could return the last processed signals
    ///   }
    ///
    ///   fn dimensions(&self) -> (usize, usize) {
    ///       (1, 2) // Example dimensions: 1 input, 2 outputs
    ///   }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signals = input_signal * block.as_mimo();
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 2.0);
    /// ```
    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output([self].to_vec())
    }
}

impl Mul<&mut dyn MIMO> for (Signal, Signal) {
    type Output = Vec<Signal>;

    /// Multiplies a tuple of two input signals with a mutable reference to a MIMO block that accepts two inputs, producing an array of output signals.
    /// This allows for processing two input signals through the MIMO block.
    /// The tuple must contain exactly two signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///   fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///     [Signal {
    ///       value: input[0].value + 1.0, // Example processing for first output
    ///       dt: input[0].dt,
    ///     }, Signal {
    ///       value: input[1].value * 2.0, // Example processing for second output
    ///       dt: input[1].dt,
    ///     }].to_vec()
    ///   }
    ///
    ///   fn last_output(&self) -> Option<Vec<Signal>> {
    ///     None // Example implementation, could return the last processed signals
    ///   }
    ///
    ///   fn dimensions(&self) -> (usize, usize) {
    ///       (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///   }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = (Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) });
    /// let output_signals = input_signals * block.as_mimo();
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output([self.0, self.1].to_vec())
    }
}

impl Mul<&mut dyn MIMO> for (Signal, Signal, Signal) {
    type Output = Vec<Signal>;

    /// Multiplies a tuple of three input signals with a mutable reference to a MIMO block that accepts three inputs, producing an array of output signals.
    /// This allows for processing three input signals through the MIMO block.
    /// The tuple must contain exactly three signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///   fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///     [Signal {
    ///       value: input[0].value + 1.0, // Example processing for first output
    ///       dt: input[0].dt,
    ///     }, Signal {
    ///       value: input[1].value * 2.0 + input[2].value, // Example processing for second output
    ///       dt: input[1].dt,
    ///     }].to_vec()
    ///   }
    ///
    ///   fn last_output(&self) -> Option<Vec<Signal>> {
    ///     None // Example implementation, could return the last processed signals
    ///   }
    ///
    ///   fn dimensions(&self) -> (usize, usize) {
    ///       (3, 2) // Example dimensions: 3 inputs, 2
    ///   }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = (Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }, Signal { value: 3.0, dt: Duration::from_secs(1) });
    /// let output_signals = input_signals * block.as_mimo();
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 7.0);
    /// ```
    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output([self.0, self.1, self.2].to_vec())
    }
}

#[cfg(feature = "alloc")]
impl Mul<&mut dyn MIMO> for Vec<Signal> {
    type Output = Vec<Signal>;

    /// Multiplies a vector of input signals with a mutable reference to a MIMO block, producing an array of output signals.
    /// This allows for processing multiple input signals through the MIMO block.
    /// The vector must have a length equal to the number of inputs expected by the MIMO block.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///   fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///     [Signal {
    ///       value: input[0].value + 1.0, // Example processing for first output
    ///       dt: input[0].dt,
    ///     }, Signal {
    ///       value: input[1].value * 2.0, // Example processing for second output
    ///       dt: input[1].dt,
    ///     }].to_vec()
    ///   }
    ///
    ///   fn last_output(&self) -> Option<Vec<Signal>> {
    ///     None // Example implementation, could return the last processed signals
    ///   }
    ///
    ///   fn dimensions(&self) -> (usize, usize) {
    ///       (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///   }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = Vec::from([Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }]);
    /// let output_signals = input_signals * block.as_mimo();
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output(self)
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for Signal {
    type Output = Signal;

    /// Puts the signal to a monitor, allowing it to display or log the signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyOutput;
    ///
    /// impl Output for MyOutput {
    ///     fn show(&mut self, inputs: &[Signal]) {
    ///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
    ///     }
    /// }
    ///
    /// impl AsOutput for MyOutput {}
    ///
    /// let mut monitor = MyOutput;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = input_signal >> monitor.as_output();
    /// assert_eq!(output_signal.value, 1.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&[self]);
        self
    }
}

impl Shr<&mut dyn ErrorMetric<1>> for Signal {
    type Output = Signal;

    /// Updates the error metric with the signal and returns the signal.
    /// This allows the error metric to accumulate or process the signal as needed.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyErrorMetric;
    ///
    /// impl ErrorMetric<1> for MyErrorMetric {
    ///     fn update(&mut self, input: [Signal; 1]) -> [Signal; 1] {
    ///         // Example implementation, could accumulate error
    ///         println!("Updating error metric with value: {}", input[0].value);
    ///         input
    ///     }
    ///
    ///     fn value(&self) -> f32 {
    ///         0.0 // Example implementation, could return accumulated error
    ///     }
    /// }
    ///
    /// impl AsErrorMetric<1> for MyErrorMetric {}
    ///
    /// let mut error_metric = MyErrorMetric;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = input_signal >> error_metric.as_error_metric();
    /// assert_eq!(output_signal.value, 1.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn shr(self, rhs: &mut dyn ErrorMetric<1>) -> Self::Output {
        let input = [self];
        let output = rhs.update(input);
        output[0]
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for (Signal, Signal) {
    type Output = (Signal, Signal);

    /// Puts a tuple of two signals to a monitor, allowing it to display or log the signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyOutput;
    ///
    /// impl Output for MyOutput {
    ///     fn show(&mut self, inputs: &[Signal]) {
    ///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
    ///     }
    /// }
    ///
    /// impl AsOutput for MyOutput {}
    ///
    /// let mut monitor = MyOutput;
    /// let input_signal1 = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let input_signal2 = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let output_signals = (input_signal1, input_signal2) >> monitor.as_output();
    /// assert_eq!(output_signals.0.value, 1.0);
    /// assert_eq!(output_signals.0.dt, Duration::from_secs(1));
    /// assert_eq!(output_signals.1.value, 2.0);
    /// assert_eq!(output_signals.1.dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&[self.0, self.1]);
        self
    }
}

impl Shr<&mut dyn ErrorMetric<2>> for (Signal, Signal) {
    type Output = (Signal, Signal);

    /// Updates the error metric with a tuple of two signals and returns the tuple.
    /// This allows the error metric to accumulate or process both signals as needed.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyErrorMetric;
    ///
    /// impl ErrorMetric<2> for MyErrorMetric {
    ///     fn update(&mut self, input: [Signal; 2]) -> [Signal; 2] {
    ///         // Example implementation, could accumulate error
    ///         println!("Updating error metric with values: {}, {}", input[0].value, input[1].value);
    ///         input
    ///     }
    ///
    ///    fn value(&self) -> f32 {
    ///        0.0 // Example implementation, could return accumulated error
    ///   }
    /// }
    ///
    /// impl AsErrorMetric<2> for MyErrorMetric {}
    ///
    /// let mut error_metric = MyErrorMetric;
    ///
    /// let input_signal1 = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let input_signal2 = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let output_signals = (input_signal1, input_signal2) >> error_metric.as_error_metric();
    /// assert_eq!(output_signals.0.value, 1.0);
    /// assert_eq!(output_signals.0.dt, Duration::from_secs(1));
    /// assert_eq!(output_signals.1.value, 2.0);
    /// assert_eq!(output_signals.1.dt, Duration::from_secs(1));
    /// ```
    fn shr(self, rhs: &mut dyn ErrorMetric<2>) -> Self::Output {
        let input = [self.0, self.1];
        let output = rhs.update(input);
        (output[0], output[1])
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for (Signal, Signal, Signal) {
    type Output = (Signal, Signal, Signal);

    /// Puts a tuple of three signals to a monitor, allowing it to display or log the signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyOutput;
    ///
    /// impl Output for MyOutput {
    ///     fn show(&mut self, inputs: &[Signal]) {
    ///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
    ///     }
    /// }
    ///
    /// impl AsOutput for MyOutput {}
    ///
    /// let mut monitor = MyOutput;
    /// let input_signal1 = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let input_signal2 = Signal { value: 2.0, dt: Duration::from_secs(1) };
    /// let input_signal3 = Signal { value: 3.0, dt: Duration::from_secs(1) };
    /// let output_signals = (input_signal1, input_signal2, input_signal3) >> monitor.as_output();
    /// assert_eq!(output_signals.0.value, 1.0);
    /// assert_eq!(output_signals.0.dt, Duration::from_secs(1));
    /// assert_eq!(output_signals.1.value, 2.0);
    /// assert_eq!(output_signals.1.dt, Duration::from_secs(1));
    /// assert_eq!(output_signals.2.value, 3.0);
    /// assert_eq!(output_signals.2.dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&[self.0, self.1, self.2]);
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a> Shr<&mut dyn Output> for &'a [Signal] {
    type Output = &'a [Signal];

    /// Puts a slice of signals to a monitor, allowing it to display or log the signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyOutput;
    ///
    /// impl Output for MyOutput {
    ///     fn show(&mut self, inputs: &[Signal]) {
    ///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
    ///     }
    /// }
    ///
    /// impl AsOutput for MyOutput {}
    ///
    /// let mut monitor = MyOutput;
    /// let input_signals = [
    ///     Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///     Signal { value: 2.0, dt: Duration::from_secs(1) },
    /// ];
    /// let output_signals = input_signals >> monitor.as_output();
    /// assert_eq!(output_signals[0].value, 1.0);
    /// assert_eq!(output_signals[0].dt, Duration::from_secs(1));
    /// assert_eq!(output_signals[1].value, 2.0);
    /// assert_eq!(output_signals[1].dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(self);
        self
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> Shr<&mut dyn Output> for [Signal; N] {
    type Output = [Signal; N];

    /// Puts an array of signals to a monitor, allowing it to display or log the signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyOutput;
    ///
    /// impl Output for MyOutput {
    ///     fn show(&mut self, inputs: &[Signal]) {
    ///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
    ///     }
    /// }
    ///
    /// impl AsOutput for MyOutput {}
    ///
    /// let mut monitor = MyOutput;
    /// let input_signals = [
    ///     Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///     Signal { value: 2.0, dt: Duration::from_secs(1) },
    /// ];
    /// let output_signals = input_signals >> monitor.as_output();
    /// assert_eq!(output_signals[0].value, 1.0);
    /// assert_eq!(output_signals[0].dt, Duration::from_secs(1));
    /// assert_eq!(output_signals[1].value, 2.0);
    /// assert_eq!(output_signals[1].dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&self);
        self
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for Vec<Signal> {
    type Output = Vec<Signal>;

    /// Puts a vector of signals to a monitor, allowing it to display or log the signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyOutput;
    ///
    /// impl Output for MyOutput {
    ///     fn show(&mut self, inputs: &[Signal]) {
    ///         println!("Outputing signal: value = {}, dt = {:?}", inputs[0].value, inputs[0].dt);
    ///     }
    /// }
    ///
    /// impl AsOutput for MyOutput {}
    ///
    /// let mut monitor = MyOutput;
    /// let input_signals = vec![
    ///     Signal { value: 1.0, dt: Duration::from_secs(1) },
    ///     Signal { value: 2.0, dt: Duration::from_secs(1) },
    /// ];
    /// let output_signals = input_signals >> monitor.as_output();
    /// assert_eq!(output_signals[0].value, 1.0);
    /// assert_eq!(output_signals[0].dt, Duration::from_secs(1));
    /// assert_eq!(output_signals[1].value, 2.0);
    /// assert_eq!(output_signals[1].dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&self);
        self
    }
}
