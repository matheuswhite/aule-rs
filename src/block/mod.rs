use crate::signal::Signal;
#[cfg(feature = "alloc")]
use core::ops::Mul;

pub mod delay;
pub mod gain;
pub mod pid;
pub mod saturation;

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

#[cfg(feature = "alloc")]
impl Mul<Signal> for &mut alloc::boxed::Box<dyn Block> {
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
