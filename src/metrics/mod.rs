use crate::signal::Signal;

#[cfg(feature = "alloc")]
pub mod good_hart;
pub mod iae;
pub mod ise;
pub mod itae;

pub trait Metric<const N: usize> {
    fn update(&mut self, input: [Signal; N]) -> [Signal; N];
    fn value(&self) -> f32;
}

pub trait AsMetric<const N: usize>: Sized + Metric<N> + 'static {
    fn as_metric(&mut self) -> &mut dyn Metric<N> {
        self
    }
}
