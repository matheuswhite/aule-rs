use crate::{
    block::Block,
    prelude::{Filter, SimulationState},
};
use core::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

pub struct Biquad<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    prev_input: [Option<T>; 2],
    prev_output: [Option<T>; 2],
    dt: Duration,
}

impl<T> Biquad<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(b0: f64, b1: f64, b2: f64, a1: f64, a2: f64, dt: Duration) -> Self {
        Self {
            b0,
            b1,
            b2,
            a1,
            a2,
            prev_input: [None, None],
            prev_output: [None, None],
            dt,
        }
    }

    pub fn coefficients(&self) -> (f64, f64, f64, f64, f64) {
        (self.b0, self.b1, self.b2, self.a1, self.a2)
    }
}

impl<T> Block for Biquad<T>
where
    T: Clone + Mul<f64, Output = T> + Add<Output = T> + Sub<Output = T>,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let prev_in_1 = self.prev_input[0].as_ref();
        let prev_in_2 = self.prev_input[1].as_ref();
        let prev_out_1 = self.prev_output[0].as_ref();
        let prev_out_2 = self.prev_output[1].as_ref();

        let prev_in_value_1 = prev_in_1
            .cloned()
            .unwrap_or_else(|| input.clone() - input.clone());
        let prev_in_value_2 = prev_in_2
            .cloned()
            .unwrap_or_else(|| input.clone() - input.clone());
        let prev_out_value_1 = prev_out_1
            .cloned()
            .unwrap_or_else(|| input.clone() - input.clone());
        let prev_out_value_2 = prev_out_2
            .cloned()
            .unwrap_or_else(|| input.clone() - input.clone());

        let input_clone = input.clone();
        let filtered =
            input * self.b0 + prev_in_value_1.clone() * self.b1 + prev_in_value_2.clone() * self.b2
                - prev_out_value_1.clone() * self.a1
                - prev_out_value_2.clone() * self.a2;

        self.prev_input[1] = self.prev_input[0].take();
        self.prev_input[0] = Some(input_clone);
        self.prev_output[1] = self.prev_output[0].take();
        self.prev_output[0] = Some(filtered.clone());

        filtered
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.prev_output[0].clone()
    }

    fn reset(&mut self) {
        self.prev_input = [None, None];
        self.prev_output = [None, None];
    }
}

impl<T> Filter for Biquad<T>
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
    use super::Biquad;
    use crate::prelude::*;
    use alloc::vec::Vec;
    use core::f64::consts::PI;
    use core::time::Duration;

    fn run_filter<B>(mut block: B, samples: &[f64], dt: f32) -> Vec<f64>
    where
        B: Block<Input = f64, Output = f64>,
    {
        Simulation::new(dt, dt * samples.len() as f32)
            .zip(samples.iter().copied())
            .map(|(sim_state, value)| block.block(value, sim_state))
            .collect()
    }

    fn sine_samples(freq_hz: f64, dt: f64, count: usize) -> Vec<f64> {
        (0..count)
            .map(|idx| libm::sin(2.0 * PI * freq_hz * (idx as f64 + 1.0) * dt))
            .collect()
    }

    fn rms(values: &[f64]) -> f64 {
        let energy = values.iter().map(|v| v * v).sum::<f64>() / values.len() as f64;
        libm::sqrt(energy)
    }

    /// Butterworth 2nd-order LP biquad coefficients for a given fc and dt.
    fn butterworth_lp(fc: f64, dt: f64) -> Biquad<f64> {
        let k = libm::tan(PI * fc * dt);
        let d = libm::sqrt(2.0);
        let a0 = k * k + d * k + 1.0;
        let b0 = k * k / a0;
        let b1 = 2.0 * k * k / a0;
        let b2 = k * k / a0;
        let a1 = 2.0 * (k * k - 1.0) / a0;
        let a2 = (k * k - d * k + 1.0) / a0;
        Biquad::new(b0, b1, b2, a1, a2, Duration::from_secs_f64(dt))
    }

    /// Condição inicial nula: com b0=0 e estado zerado, o primeiro sample deve ser 0.
    /// Usa um filtro de atraso puro (b0=0, b1=1) para evidenciar que prev_input
    /// começa como zero.
    #[test]
    fn test_biquad_uses_null_initial_condition() {
        let dt = 0.01_f64;
        let mut filter = Biquad::new(0.0, 1.0, 0.0, 0.0, 0.0, Duration::from_secs_f64(dt));
        let sim_state = Simulation::new(dt as f32, dt as f32).next().unwrap();

        let output = filter.block(1.0, sim_state);

        assert!((output - 0.0).abs() < 1e-9);
        assert!((filter.last_output().unwrap() - 0.0).abs() < 1e-9);
    }

    /// Resposta ao degrau: Butterworth LP de ganho DC = 1 deve convergir para 1.0.
    #[test]
    fn test_biquad_step_response_converges_to_dc_gain() {
        let dt = 0.01_f64;
        let n = 2000;
        let samples: Vec<f64> = (0..n).map(|_| 1.0).collect();
        let outputs = run_filter(butterworth_lp(1.0, dt), &samples, dt as f32);

        assert!((outputs[n - 1] - 1.0).abs() < 1e-3);
    }

    /// Resposta em frequência: sinal na faixa de passagem é preservado, fora é atenuado.
    /// Butterworth LP fc=1Hz: 0.2Hz (passa) vs 4.0Hz (rejeita).
    #[test]
    fn test_biquad_lp_preserves_low_frequency_more_than_high_frequency() {
        let dt = 0.01;
        let count = 1000;
        let low_freq = sine_samples(0.2, dt, count);
        let high_freq = sine_samples(4.0, dt, count);

        let low_output = run_filter(butterworth_lp(1.0, dt), &low_freq, dt as f32);
        let high_output = run_filter(butterworth_lp(1.0, dt), &high_freq, dt as f32);

        let low_rms = rms(&low_output[200..]);
        let high_rms = rms(&high_output[200..]);

        assert!(low_rms > high_rms * 2.0);
    }
}
