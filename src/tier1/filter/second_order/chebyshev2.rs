use crate::{
    block::Block,
    math::{
        float_point::{AsFloatPoint, FloatPoint},
        sample::Sample,
    },
    prelude::{Biquad, Filter, SimulationState},
};
use core::time::Duration;

pub struct Chebyshev2<T>
where
    T: Sample,
{
    cutoff_freq: T::Alpha,
    ripple_db: T::Alpha,
    biquad: Biquad<T>,
    dt: Duration,
}

impl<T> Chebyshev2<T>
where
    T: Sample,
{
    fn base_parameters(
        cutoff_freq: T::Alpha,
        ripple_db: T::Alpha,
        dt: Duration,
    ) -> (T::Alpha, T::Alpha, T::Alpha, T::Alpha) {
        let ts = T::Alpha::from_duration(dt);
        let ten: T::Alpha = 10.0.as_fp();
        let half: T::Alpha = 0.5.as_fp();

        let epsilon = (ten.power(ripple_db / 10.0.as_fp()) - 1.0.as_fp()).square_root();
        let gamma: T::Alpha = half * (T::Alpha::one() / epsilon).arc_sin_h();

        let (sinh_g, cosh_g) = (gamma.sin_h(), gamma.cos_h());

        // Type I pole locations (angle theta = pi/4)
        let sigma_1 = sinh_g / T::Alpha::sqrt_2();
        let omega_1 = cosh_g / T::Alpha::sqrt_2();
        let r1_sq = sigma_1 * sigma_1 + omega_1 * omega_1;

        // Type II: invert poles
        let sigma_2 = sigma_1 / r1_sq;
        let r2_sq = T::Alpha::one() / r1_sq;

        // Zeros at ±j/cos(π/4) = ±j√2 (normalized)
        let omega_z_sq = 2.0.as_fp();

        let k = (T::Alpha::pi() * cutoff_freq * ts).tangent();

        (sigma_2, r2_sq, omega_z_sq, k)
    }

    pub fn low_pass(cutoff_freq: T::Alpha, ripple_db: T::Alpha, dt: Duration) -> Self {
        let (sigma_2, r2_sq, omega_z_sq, k) = Self::base_parameters(cutoff_freq, ripple_db, dt);
        let two: T::Alpha = 2.0.as_fp();

        // Denominator: H(s) denominator s² + 2σ₂s + r₂²
        let a0 = r2_sq * k * k + two * sigma_2 * k + 1.0.as_fp();

        // Numerator with zeros: s² + ω_z²
        // Gain normalized for unity DC gain: multiply by r₂²/ω_z²
        let gain = r2_sq / omega_z_sq;
        let n0 = T::Alpha::one() + omega_z_sq * k * k;
        let n1 = two * (omega_z_sq * k * k - 1.0.as_fp());

        let b0 = gain * n0 / a0;
        let b1 = gain * n1 / a0;
        let b2 = b0;
        let a1 = two * (r2_sq * k * k - 1.0.as_fp()) / a0;
        let a2 = (r2_sq * k * k - two * sigma_2 * k + 1.0.as_fp()) / a0;

        Self {
            cutoff_freq,
            ripple_db,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn high_pass(cutoff_freq: T::Alpha, ripple_db: T::Alpha, dt: Duration) -> Self {
        let (sigma_2, r2_sq, omega_z_sq, k) = Self::base_parameters(cutoff_freq, ripple_db, dt);
        let two: T::Alpha = 2.0.as_fp();

        // HP denominator (LP→HP: s → 1/s)
        let a0 = r2_sq + two * sigma_2 * k + k * k;

        // HP numerator with zeros
        let gain = r2_sq / omega_z_sq;
        let n0 = omega_z_sq + k * k;
        let n1 = two * (k * k - omega_z_sq);

        let b0 = gain * n0 / a0;
        let b1 = gain * n1 / a0;
        let b2 = b0;
        let a1 = two * (k * k - r2_sq) / a0;
        let a2 = (r2_sq - two * sigma_2 * k + k * k) / a0;

        Self {
            cutoff_freq,
            ripple_db,
            biquad: Biquad::new(b0, b1, b2, a1, a2, dt),
            dt,
        }
    }

    pub fn cutoff_freq(&self) -> T::Alpha {
        self.cutoff_freq
    }

    pub fn center_freq(&self) -> T::Alpha {
        self.cutoff_freq
    }

    pub fn ripple_db(&self) -> T::Alpha {
        self.ripple_db
    }

    pub fn biquad_coefficients(&self) -> (T::Alpha, T::Alpha, T::Alpha, T::Alpha, T::Alpha) {
        self.biquad.coefficients()
    }
}

impl<T> Block for Chebyshev2<T>
where
    T: Sample,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        self.biquad.block(input, sim_state)
    }

    fn reset(&mut self) {
        self.biquad.reset();
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.biquad.last_output()
    }
}

impl<T> Filter for Chebyshev2<T>
where
    T: Sample,
{
    type SignalValue = T;

    fn dt(&self) -> Duration {
        self.dt
    }
}
