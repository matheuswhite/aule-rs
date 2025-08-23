use crate::signal::Signal;

#[cfg(feature = "alloc")]
pub mod good_hart;
pub mod iae;
pub mod ise;
pub mod itae;

pub trait ErrorMetric<const N: usize> {
    fn update(&mut self, input: [Signal; N]) -> [Signal; N];
    fn value(&self) -> f32;
}

pub trait AsErrorMetric<const N: usize>: Sized + ErrorMetric<N> + 'static {
    fn as_error_metric(&mut self) -> &mut dyn ErrorMetric<N> {
        self
    }
}
