use crate::{block::Block, prelude::SimulationState};
use core::{f32::consts::PI, time::Duration};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Sinusoid<T>
where
    T: Float,
{
    amplitude: T,
    period: Duration,
    phase: T,
}

impl<T> Sinusoid<T>
where
    T: Float,
{
    pub fn new(amplitude: T, period: Duration, phase: T) -> Self {
        Sinusoid {
            amplitude,
            period,
            phase,
        }
    }

    /// Wraps the angle between 0 and 2pi
    fn wrap_angle(&self, angle: T) -> T {
        let two_pi = T::from(2.0 * PI).unwrap();
        let mut wrapped = angle % two_pi;
        if wrapped < T::zero() {
            wrapped = wrapped + two_pi;
        }
        wrapped
    }

    fn power_series(&self, angle: T) -> T {
        angle - (angle * angle * angle) / T::from(6.0).unwrap()
            + (angle * angle * angle * angle * angle) / T::from(120.0).unwrap()
            - (angle * angle * angle * angle * angle * angle * angle) / T::from(5040.0).unwrap()
    }

    fn approx_sine(&self, angle: T) -> T {
        let wrapped = self.wrap_angle(angle);
        let pi = T::from(PI).unwrap();
        let half_pi = pi / T::from(2.0).unwrap();
        let three_half_pi = pi + half_pi;
        let two_pi = pi + pi;

        if wrapped <= half_pi {
            self.power_series(wrapped)
        } else if wrapped <= pi {
            self.power_series(pi - wrapped)
        } else if wrapped <= three_half_pi {
            -self.power_series(wrapped - pi)
        } else {
            -self.power_series(two_pi - wrapped)
        }
    }
}

impl<T> Default for Sinusoid<T>
where
    T: Float,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            phase: T::zero(),
        }
    }
}

impl<T> Block for Sinusoid<T>
where
    T: Float,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let t = sim_state.sim_time().as_secs_f32();
        let t = T::from(t).unwrap();
        let period = T::from(self.period.as_secs_f32()).unwrap();

        if period.is_zero() {
            return self.amplitude * self.approx_sine(self.phase);
        }

        let angle = T::from(2.0 * PI).unwrap() * t / period + self.phase;

        self.amplitude * self.approx_sine(angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::Simulation;

    fn output_at(sinusoid: &mut Sinusoid<f32>, time: f32) -> f32 {
        let sim_state = Simulation::new(time, time).next().unwrap();
        sinusoid.block((), sim_state)
    }

    #[test]
    fn uses_amplitude_and_period() {
        let mut sinusoid = Sinusoid::new(2.0, Duration::from_secs_f32(2.0), 0.0);

        let output = output_at(&mut sinusoid, 0.5);

        assert!((output - 2.0).abs() < 5e-4);
    }

    #[test]
    fn uses_phase() {
        let mut sinusoid = Sinusoid::new(1.0, Duration::from_secs_f32(2.0 * PI), PI / 2.0);

        let output = output_at(&mut sinusoid, PI);

        assert!((output + 1.0).abs() < 2e-4);
    }

    #[test]
    fn stays_continuous_around_pi() {
        let mut sinusoid = Sinusoid::<f32>::default();

        let before_pi = output_at(&mut sinusoid, PI - 0.01);
        let after_pi = output_at(&mut sinusoid, PI + 0.01);

        assert!(before_pi > 0.0);
        assert!(after_pi < 0.0);
        assert!((after_pi - before_pi).abs() < 0.05);
    }
}
