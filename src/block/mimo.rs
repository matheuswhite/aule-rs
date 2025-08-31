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
/// impl MIMO<2, 2> for MyMIMOBlock {
///    fn output(&mut self, input: [Signal; 2]) -> [Signal; 2] {
///        // Process the input signals and return new signals
///        [Signal {
///            value: input[0].value + 1.0, // Example processing for first output
///            dt: input[0].dt,
///        }, Signal {
///            value: input[1].value * 2.0, // Example processing for second output
///            dt: input[1].dt,
///        }]
///    }
///
///    fn last_output(&self) -> Option<[Signal; 2]> {
///        None // Example implementation, could return the last processed signals
///    }
/// }
///
/// let mut block = MyMIMOBlock;
/// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
/// let output_signals = block.output(input_signals);
/// assert_eq!(output_signals[0].value, 2.0);
/// assert_eq!(output_signals[1].value, 4.0);
/// ```
pub trait MIMO<const I: usize, const O: usize> {
    fn output(&mut self, input: [Signal; I]) -> [Signal; O];
    fn last_output(&self) -> Option<[Signal; O]>;
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
/// impl MIMO<2, 2> for MyMIMOBlock {
///    fn output(&mut self, input: [Signal; 2]) -> [Signal; 2] {
///        [Signal {
///            value: input[0].value + 1.0, // Example processing for first output
///            dt: input[0].dt,
///        }, Signal {
///            value: input[1].value * 2.0, // Example processing for second output
///            dt: input[1].dt,
///        }]
///    }
///
///    fn last_output(&self) -> Option<[Signal; 2]> {
///        None // Example implementation, could return the last processed signals
///    }
/// }
///
/// impl AsMIMO<2, 2> for MyMIMOBlock {}
///
/// let mut block = MyMIMOBlock;
/// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
/// let output_signals = input_signals * block.as_mimo();
/// assert_eq!(output_signals[0].value, 2.0);
/// assert_eq!(output_signals[1].value, 4.0);
/// ```
pub trait AsMIMO<const I: usize, const O: usize>: Sized + MIMO<I, O> + 'static {
    fn as_mimo(&mut self) -> &mut dyn MIMO<I, O> {
        self
    }
}

impl<const I: usize, const O: usize> Mul<[Signal; I]> for &mut dyn MIMO<I, O> {
    type Output = [Signal; O];

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
    /// impl MIMO<2, 2> for MyMIMOBlock {
    ///    fn output(&mut self, input: [Signal; 2]) -> [Signal; 2] {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }]
    ///    }
    ///
    ///    fn last_output(&self) -> Option<[Signal; 2]> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    /// }
    ///
    /// impl AsMIMO<2, 2> for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = [Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: [Signal; I]) -> Self::Output {
        self.output(rhs)
    }
}

impl<const I: usize, const O: usize> Mul<&[Signal]> for &mut dyn MIMO<I, O> {
    type Output = [Signal; O];

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
    /// impl MIMO<2, 2> for MyMIMOBlock {
    ///    fn output(&mut self, input: [Signal; 2]) -> [Signal; 2] {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }]
    ///    }
    ///
    ///    fn last_output(&self) -> Option<[Signal; 2]> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    /// }
    ///
    /// impl AsMIMO<2, 2> for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = vec![Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
    /// let output_signals = block.as_mimo() * &input_signals[..];
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: &[Signal]) -> Self::Output {
        let mut input: [Signal; I] = [Signal::default(); I];
        input.copy_from_slice(&rhs[0..I]);
        self.output(input)
    }
}

impl<const O: usize> Mul<Signal> for &mut dyn MIMO<1, O> {
    type Output = [Signal; O];

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
    /// impl MIMO<1, 2> for MyMIMOBlock {
    ///    fn output(&mut self, input: [Signal; 1]) -> [Signal; 2] {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[0].value * 2.0, // Example processing for second output
    ///            dt: input[0].dt,
    ///        }]
    ///    }
    ///
    ///    fn last_output(&self) -> Option<[Signal; 2]> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    /// }
    ///
    /// impl AsMIMO<1, 2> for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signal = Signal { value: 1.0, dt: Duration::from_secs(1) };
    /// let output_signals = block.as_mimo() * input_signal;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 2.0);
    /// ```
    fn mul(self, rhs: Signal) -> Self::Output {
        let input: [Signal; 1] = [rhs];
        self.output(input)
    }
}

impl<const O: usize> Mul<(Signal, Signal)> for &mut dyn MIMO<2, O> {
    type Output = [Signal; O];

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
    /// impl MIMO<2, 2> for MyMIMOBlock {
    ///    fn output(&mut self, input: [Signal; 2]) -> [Signal; 2] {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }]
    ///    }
    ///
    ///    fn last_output(&self) -> Option<[Signal; 2]> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    /// }
    ///
    /// impl AsMIMO<2, 2> for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = (Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) });
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: (Signal, Signal)) -> Self::Output {
        let input: [Signal; 2] = [rhs.0, rhs.1];
        self.output(input)
    }
}

impl<const O: usize> Mul<(Signal, Signal, Signal)> for &mut dyn MIMO<3, O> {
    type Output = [Signal; O];

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
    /// impl MIMO<3, 2> for MyMIMOBlock {
    ///    fn output(&mut self, input: [Signal; 3]) -> [Signal; 2] {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0 + input[2].value, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }]
    ///    }
    ///
    ///    fn last_output(&self) -> Option<[Signal; 2]> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    /// }
    ///
    /// impl AsMIMO<3, 2> for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals = (Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }, Signal { value: 3.0, dt: Duration::from_secs(1) });
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 7.0);
    /// ```
    fn mul(self, rhs: (Signal, Signal, Signal)) -> Self::Output {
        let input: [Signal; 3] = [rhs.0, rhs.1, rhs.2];
        self.output(input)
    }
}

#[cfg(feature = "alloc")]
impl<const I: usize, const O: usize> Mul<Vec<Signal>> for &mut dyn MIMO<I, O> {
    type Output = [Signal; O];

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
    /// impl MIMO<2, 2> for MyMIMOBlock {
    ///    fn output(&mut self, input: [Signal; 2]) -> [Signal; 2] {
    ///        [Signal {
    ///            value: input[0].value + 1.0, // Example processing for first output
    ///            dt: input[0].dt,
    ///        }, Signal {
    ///            value: input[1].value * 2.0, // Example processing for second output
    ///            dt: input[1].dt,
    ///        }]
    ///    }
    ///
    ///    fn last_output(&self) -> Option<[Signal; 2]> {
    ///        None // Example implementation, could return the last processed signals
    ///    }
    /// }
    ///
    /// impl AsMIMO<2, 2> for MyMIMOBlock {}
    ///
    /// let mut block = MyMIMOBlock;
    /// let input_signals: Vec<Signal> = vec![Signal { value: 1.0, dt: Duration::from_secs(1) }, Signal { value: 2.0, dt: Duration::from_secs(1) }];
    /// let output_signals = block.as_mimo() * input_signals;
    /// assert_eq!(output_signals[0].value, 2.0);
    /// assert_eq!(output_signals[1].value, 4.0);
    /// ```
    fn mul(self, rhs: Vec<Signal>) -> Self::Output {
        let mut input: [Signal; I] = [Signal::default(); I];
        input.copy_from_slice(&rhs[0..I]);
        self.output(input)
    }
}
