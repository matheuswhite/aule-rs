use crate::{block::Block, signal::Signal, tier1::filter::Filter};
use core::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

pub struct LowPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    cutoff_freq: f64,
    alpha: f64,
    prev_output: Option<Signal<T>>,
    dt: Duration,
}

impl<T> LowPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(cutoff_freq: f64, dt: Duration) -> Self {
        let ts = dt.as_secs_f64();
        let tau = 1.0 / (2.0 * core::f64::consts::PI * cutoff_freq);

        #[cfg(feature = "std")]
        let alpha = 1.0 - (-ts / tau).exp();
        #[cfg(not(feature = "std"))]
        let alpha = ts / (tau + ts);

        Self {
            cutoff_freq,
            alpha,
            prev_output: None,
            dt,
        }
    }

    pub fn cutoff_freq(&self) -> f64 {
        self.cutoff_freq
    }

    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}

impl<T> Block for LowPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Input = T;
    type Output = T;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let prev_value = self.prev_output.as_ref().map_or_else(
            || input.value.clone() - input.value.clone(),
            |prev| prev.value.clone(),
        );

        let filtered =
            input.map(|sig| prev_value.clone() + (sig - prev_value.clone()) * self.alpha);
        self.prev_output = Some(filtered.clone());
        filtered
    }

    fn reset(&mut self) {
        self.prev_output = None;
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.prev_output.clone().map(|signal| signal.value)
    }
}

impl<T> Filter for LowPass<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::LowPass;
    use crate::prelude::*;
    use alloc::vec::Vec;
    use core::f64::consts::PI;
    use core::time::Duration;

    fn run_filter<B>(mut block: B, samples: &[f64], dt: f32) -> Vec<f64>
    where
        B: Block<Input = f64, Output = f64>,
    {
        Time::new(dt, dt * samples.len() as f32)
            .zip(samples.iter().copied())
            .map(|(delta, value)| block.output(delta.map(|_| value)).value)
            .collect()
    }

    fn sine_samples(freq_hz: f64, dt: f64, count: usize) -> Vec<f64> {
        (0..count)
            .map(|idx| libm::sin(2.0 * PI * freq_hz * (idx as f64 + 1.0) * dt))
            .collect()
    }

    fn rms(values: &[f64]) -> f64 {
        let energy = values.iter().map(|value| value * value).sum::<f64>() / values.len() as f64;
        energy.sqrt()
    }

    #[test]
    fn test_low_pass_uses_null_initial_condition() {
        let delta = Time::new(0.1, 0.1).next().unwrap();
        let mut filter = LowPass::new(1.0, Duration::from_secs_f32(0.1));
        let tau = 1.0 / (2.0 * PI);
        let ts = Duration::from_secs_f32(0.1).as_secs_f64();
        let alpha = 1.0 - (-ts / tau).exp();

        let output = filter.output(delta.map(|_| 1.0));

        assert!((output.value - alpha).abs() < 1e-9);
        assert!((filter.last_output().unwrap() - alpha).abs() < 1e-9);
    }

    #[test]
    fn test_low_pass_step_response_matches_exact_discretization() {
        let samples = [0.0, 0.0, 1.0, 1.0, 1.0];
        let outputs = run_filter(
            LowPass::new(1.0, Duration::from_secs_f32(0.1)),
            &samples,
            0.1,
        );
        let tau = 1.0 / (2.0 * PI);
        let ts = Duration::from_secs_f32(0.1).as_secs_f64();
        let alpha = 1.0 - (-ts / tau).exp();
        let expected = [
            0.0,
            0.0,
            alpha,
            1.0 - (1.0 - alpha).powi(2),
            1.0 - (1.0 - alpha).powi(3),
        ];

        for (output, expected) in outputs.iter().zip(expected) {
            assert!((output - expected).abs() < 1e-9);
        }
    }

    #[test]
    fn test_low_pass_preserves_low_frequency_more_than_high_frequency() {
        let dt = 0.01;
        let count = 1000;
        let low_freq = sine_samples(0.2, dt, count);
        let high_freq = sine_samples(4.0, dt, count);

        let low_output = run_filter(
            LowPass::new(1.0, Duration::from_secs_f64(dt)),
            &low_freq,
            dt as f32,
        );
        let high_output = run_filter(
            LowPass::new(1.0, Duration::from_secs_f64(dt)),
            &high_freq,
            dt as f32,
        );

        let low_rms = rms(&low_output[200..]);
        let high_rms = rms(&high_output[200..]);

        assert!(low_rms > high_rms * 2.0);
    }
}
