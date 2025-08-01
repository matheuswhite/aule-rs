use core::time::Duration;
use std::ops::{Mul, Shr, Sub};

/// The `Block` trait defines the interface for processing signals in a block-based system.
/// It provides methods to output a processed signal and retrieve the last output signal.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyBlock;
///
/// impl Block for MyBlock {
///    fn output(&mut self, input: Signal) -> Signal {
///        // Process the input signal and return a new signal
///        Signal {
///            value: input.value * 2.0, // Example processing
///           dt: input.dt,
///       }
///    }
///
///    fn last_output(&self) -> Option<Signal> {
///       None // Example implementation, could return the last processed signal
///    }
/// }
///
/// let mut block = MyBlock;
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// let output_signal = block.output(input_signal);
/// assert_eq!(output_signal.value, 2.0);
/// ```
pub trait Block {
    fn output(&mut self, input: Signal) -> Signal;
    fn last_output(&self) -> Option<Signal>;
}

/// The `Monitor` trait defines the interface for monitoring signals in a block-based system.
/// It provides a method to display or log the input signal.
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
///         // Display or log the input signal
///         println!("Monitoring signal: value = {}, dt = {:?}", inputs.value, inputs.dt);
///     }
/// }
///
/// let mut monitor = MyMonitor;
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// monitor.show(input_signal);
/// ```
pub trait Monitor {
    fn show(&mut self, inputs: Signal);
}

/// The `Input` trait defines the interface for input blocks in a block-based system.
/// It provides a method to output a signal based on a given time step.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyInput;
///
/// impl Input for MyInput {
///     fn output(&mut self, dt: Duration) -> Signal {
///         // Generate a signal based on the time step
///         Signal {
///             value: 1.0, // Example value
///             dt, // Use the provided duration
///         }
///     }
/// }
///
/// let mut input = MyInput;
/// let dt = Duration::from_secs(1);
/// let output_signal = input.output(dt);
/// assert_eq!(output_signal.value, 1.0);
/// assert_eq!(output_signal.dt, dt);
/// ```
pub trait Input {
    fn output(&mut self, dt: Duration) -> Signal;
}

/// The `AsBlock` trait provides a way to treat any type that implements the `Block` trait as a dynamic block.
/// It allows for dynamic dispatch of the `output` and `last_output` methods.
/// This is useful for scenarios where you want to work with blocks without knowing their concrete types at compile time.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyBlock;
///
/// impl Block for MyBlock {
///    fn output(&mut self, input: Signal) -> Signal {
///       Signal {
///           value: input.value * 2.0, // Example processing
///          dt: input.dt,
///      }
///   }
///
///   fn last_output(&self) -> Option<Signal> {
///      None // Example implementation, could return the last processed signal
///   }
/// }
///
/// impl AsBlock for MyBlock {}
///
/// let mut block = MyBlock;
/// let mut block: &mut dyn Block = block.as_block();
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// let output_signal = block.output(input_signal);
/// assert_eq!(output_signal.value, 2.0);
/// ```
pub trait AsBlock: Sized + Block + 'static {
    fn as_block(&mut self) -> &mut dyn Block {
        self
    }
}

/// The `AsMonitor` trait provides a way to treat any type that implements the `Monitor` trait as a dynamic monitor.
/// It allows for dynamic dispatch of the `show` method.
/// This is useful for scenarios where you want to work with monitors without knowing their concrete types at compile time.
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
/// let mut monitor: &mut dyn Monitor = monitor.as_monitor();
/// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
/// monitor.show(input_signal);
/// ```
pub trait AsMonitor: Sized + Monitor + 'static {
    fn as_monitor(&mut self) -> &mut dyn Monitor {
        self
    }
}

/// The `AsInput` trait provides a way to treat any type that implements the `Input` trait as a dynamic input.
/// It allows for dynamic dispatch of the `output` method.
/// This is useful for scenarios where you want to work with inputs without knowing their concrete types at compile time.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyInput;
///
/// impl Input for MyInput {
///     fn output(&mut self, dt: Duration) -> Signal {
///         Signal {
///             value: 1.0, // Example value
///             dt, // Use the provided duration
///         }
///     }
/// }
///
/// impl AsInput for MyInput {}
///
/// let mut input = MyInput;
/// let mut input: &mut dyn Input = input.as_input();
/// let dt = Duration::from_secs(1);
/// let output_signal = input.output(dt);
/// assert_eq!(output_signal.value, 1.0);
/// assert_eq!(output_signal.dt, dt);
/// ```
pub trait AsInput: Sized + Input + 'static {
    fn as_input(&mut self) -> &mut dyn Input {
        self
    }
}

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

impl Mul<Signal> for &mut Box<dyn Block> {
    type Output = Signal;

    /// Multiplies a mutable reference to a block with a signal, producing an output signal.
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
    /// let mut block: Box<dyn Block> = Box::new(MyBlock);
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signal = &mut block * input_signal;
    /// assert_eq!(output_signal.value, 2.0);
    /// assert_eq!(output_signal.dt, Duration::from_secs(1));
    /// ```
    fn mul(self, input: Signal) -> Self::Output {
        self.output(input)
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

impl Shr<&mut dyn Input> for Duration {
    type Output = Signal;

    /// Puts the duration into an input block, producing a signal based on the duration.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyInput;
    ///
    /// impl Input for MyInput {
    ///     fn output(&mut self, dt: Duration) -> Signal {
    ///         Signal {
    ///             value: 1.0, // Example value
    ///             dt, // Use the provided duration
    ///         }
    ///     }
    /// }
    ///
    /// impl AsInput for MyInput {}
    ///
    /// let mut input = MyInput;
    /// let dt = Duration::from_secs(1);
    /// let output_signal = dt >> input.as_input();
    /// assert_eq!(output_signal.value, 1.0);
    /// assert_eq!(output_signal.dt, dt);
    /// ```
    fn shr(self, rhs: &mut dyn Input) -> Self::Output {
        rhs.output(self)
    }
}
