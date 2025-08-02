use core::time::Duration;
use std::ops::{Mul, Shr, Sub};

use crate::{block::Block, monitor::Monitor};

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
#[derive(Debug, Clone, Copy, Default)]
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
            dt: self.dt,
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

impl Mul<&mut Box<dyn Block>> for Signal {
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
    /// impl Block for MyBlock {
    ///     fn output(&mut self, input: Signal) -> Signal {
    ///         Signal {
    ///             value: input.value * 2.0, // Example processing
    ///             dt: input.dt,
    ///         }
    ///     }
    ///
    ///     fn last_output(&self) -> Option<Signal> {
    ///         None // Example implementation, could return the last processed signal
    ///     }
    /// }
    ///
    /// impl AsBlock for MyBlock {}
    ///
    /// let mut block = Box::new(MyBlock);
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = input_signal * block.as_block();
    /// assert_eq!(output_signal.value, 2.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn mul(self, block: &mut Box<dyn Block>) -> Self::Output {
        block.output(self)
    }
}

impl Mul<&mut dyn Block> for Signal {
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
    /// impl Block for MyBlock {
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
    /// impl AsBlock for MyBlock {}
    ///
    /// let mut block = MyBlock;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = input_signal * block.as_block();
    /// assert_eq!(output_signal.value, 2.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn mul(self, block: &mut dyn Block) -> Self::Output {
        block.output(self)
    }
}

impl Shr<&mut dyn Monitor> for Signal {
    type Output = Signal;

    /// Puts the signal to a monitor, allowing it to display or log the signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMonitor;
    ///
    /// impl Monitor for MyMonitor {
    ///     fn show(&mut self, inputs: Signal) {
    ///         println!("Monitoring signal: value = {}, dt = {:?}", inputs.value, inputs.dt);
    ///     }
    /// }
    ///
    /// impl AsMonitor for MyMonitor {}
    ///
    /// let mut monitor = MyMonitor;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = input_signal >> monitor.as_monitor();
    /// assert_eq!(output_signal.value, 1.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn shr(self, monitor: &mut dyn Monitor) -> Self::Output {
        monitor.show(self);
        self
    }
}
