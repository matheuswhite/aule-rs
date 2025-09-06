use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::Mul;

/// The `MIMO` trait defines the interface for processing multiple input and output signals in a block-based system.
/// It provides methods to output processed signals and retrieve the last output signals.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyMIMOBlock;
///
/// impl MIMO for MyMIMOBlock {
///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
///        // Process the input signals and return new signals
///        [Signal {
///            value: input[0].value + 1.0, // Example processing for first output
///            dt: input[0].dt,
///        }, Signal {
///            value: input[1].value * 2.0, // Example processing for second output
///            dt: input[1].dt,
///        }].to_vec()
///    }
///
///    fn last_output(&self) -> Option<Vec<Signal>> {
///        None // Example implementation, could return the last processed signals
///    }
///
///    fn dimensions(&self) -> (usize, usize) {
///        (2, 2) // Example dimensions: 2 inputs, 2 outputs
///    }
/// }
///
/// let mut block = MyMIMOBlock;
/// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }].to_vec();
/// let output_signals = block.output(input_signals);
/// assert_eq!(output_signals[0].value, 2.0);
/// assert_eq!(output_signals[1].value, 4.0);
/// ```
pub trait MIMO {
    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal>;
    fn last_output(&self) -> Option<Vec<Signal>>;
    fn dimensions(&self) -> (usize, usize); // (inputs, outputs)
}

/// The `AsMIMO` trait provides a way to treat any type that implements the `MIMO` trait as a dynamic block.
/// It allows for dynamic dispatch of the `output` and `last_output` methods.
/// This is useful for scenarios where you want to work with blocks without knowing their concrete types at compile time.
///
/// # Examples
/// ```
/// use aule::prelude::*;
/// use std::time::Duration;
///
/// struct MyMIMOBlock;
///
/// impl MIMO for MyMIMOBlock {
///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
///        [Signal {
///            value: input[0].value + 1.0, // Example processing for first output
///            dt: input[0].dt,
///        }, Signal {
///            value: input[1].value * 2.0, // Example processing for second output
///            dt: input[1].dt,
///        }].to_vec()
///    }
///
///    fn last_output(&self) -> Option<Vec<Signal>> {
///        None // Example implementation, could return the last processed signals
///    }
///
///    fn dimensions(&self) -> (usize, usize) {
///        (2, 2) // Example dimensions: 2 inputs, 2 outputs
///    }
/// }
///
/// impl AsMIMO for MyMIMOBlock {}
///
/// let mut block = MyMIMOBlock;
/// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }].to_vec();
/// let output_signals = input_signals * block.as_mimo();
/// assert_eq!(output_signals[0].value, 2.0);
/// assert_eq!(output_signals[1].value, 4.0);
/// ```
pub trait AsMIMO: Sized + MIMO + 'static {
    fn as_mimo(&mut self) -> &mut dyn MIMO {
        self
    }
}

impl<const I: usize> Mul<[Signal; I]> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    /// Multiplies the MIMO block by an array of input signals, producing an array of output signals.
    /// This allows for a more intuitive syntax when working with MIMO blocks.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }].to_vec()
    ///    }
    ///
    ///    fn last_output(&self) -> Option<Vec<Signal>> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    ///
    ///    fn dimensions(&self) -> (usize, usize) {
    ///        (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///    }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }].to_vec();
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: [Signal; I]) -> Self::Output {
        self.output(rhs.to_vec())
    }
}

impl Mul<&[Signal]> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    /// Multiplies a mutable reference to a MIMO block with a slice of input signals, producing an array of output signals.
    /// This allows for dynamic input sizes while still leveraging the MIMO block's processing capabilities.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }].to_vec()
    ///    }
    ///
    ///    fn last_output(&self) -> Option<Vec<Signal>> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    ///
    ///    fn dimensions(&self) -> (usize, usize) {
    ///        (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///    }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = vec![Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }].to_vec();
    /// let output_signals = block.as_mimo() * &input_signals[..];
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: &[Signal]) -> Self::Output {
        self.output(rhs.to_vec())
    }
}

impl Mul<Signal> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    /// Multiplies a mutable reference to a MIMO block with a single input signal, producing an array of output signals.
    /// This is a convenience implementation for MIMO blocks that accept a single input signal.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[0].value * 2.0, // Example processing for second output
    ///            dt: input[0].dt,
    ///        }].to_vec()
    ///    }
    ///
    ///    fn last_output(&self) -> Option<Vec<Signal>> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    ///
    ///    fn dimensions(&self) -> (usize, usize) {
    ///        (1, 2) // Example dimensions: 1 input, 2 outputs
    ///    }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signals = block.as_mimo() * input_signal;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 2.0);
    /// ```
    fn mul(self, rhs: Signal) -> Self::Output {
        self.output([rhs].to_vec())
    }
}

impl Mul<(Signal, Signal)> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    /// Multiplies a mutable reference to a MIMO block with a tuple of two input signals, producing an array of output signals.
    /// This is a convenience implementation for MIMO blocks that accept two input signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }].to_vec()
    ///    }
    ///
    ///    fn last_output(&self) -> Option<Vec<Signal>> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    ///
    ///    fn dimensions(&self) -> (usize, usize) {
    ///        (2, 2) // Example dimensions: 2 inputs, 2
    ///    }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = (Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) });
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: (Signal, Signal)) -> Self::Output {
        self.output([rhs.0, rhs.1].to_vec())
    }
}

impl Mul<(Signal, Signal, Signal)> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    /// Multiplies a mutable reference to a MIMO block with a tuple of three input signals, producing an array of output signals.
    /// This is a convenience implementation for MIMO blocks that accept three input signals.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0 + input[2].value, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }].to_vec()
    ///    }
    ///
    ///    fn last_output(&self) -> Option<Vec<Signal>> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    ///
    ///    fn dimensions(&self) -> (usize, usize) {
    ///        (3, 2) // Example dimensions: 3 inputs, 2 outputs
    ///    }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = (Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }, Signal { value: 3.0, dt: Duration::from_secs(1) });
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 7.0);
    /// ```
    fn mul(self, rhs: (Signal, Signal, Signal)) -> Self::Output {
        self.output([rhs.0, rhs.1, rhs.2].to_vec())
    }
}

#[cfg(feature = "alloc")]
impl Mul<Vec<Signal>> for &mut dyn MIMO {
    type Output = Vec<Signal>;

    /// Multiplies a mutable reference to a MIMO block with a vector of input signals, producing an array of output signals.
    /// This allows for dynamic input sizes while still leveraging the MIMO block's processing capabilities.
    /// This is similar to the slice implementation but works directly with `Vec<Signal>`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    /// use std::time::Duration;
    ///
    /// struct MyMIMOBlock;
    ///
    /// impl MIMO for MyMIMOBlock {
    ///    fn output(&mut self, input: Vec<Signal>) -> Vec<Signal> {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }].to_vec()
    ///    }
    ///
    ///    fn last_output(&self) -> Option<Vec<Signal>> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    ///
    ///    fn dimensions(&self) -> (usize, usize) {
    ///        (2, 2) // Example dimensions: 2 inputs, 2 outputs
    ///    }
    /// }
    ///
    /// impl AsMIMO for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals: Vec<Signal> = vec![Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: Vec<Signal>) -> Self::Output {
        self.output(rhs)
    }
}
