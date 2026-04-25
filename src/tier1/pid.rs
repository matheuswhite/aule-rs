use crate::{block::Block, prelude::SimulationState};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct PID<T>
where
    T: Float,
{
    kp: T,
    ki: T,
    kd: T,
    last_input: T,
    last_integral: T,
    last_output: Option<T>,
    anti_windup: Option<(T, T)>,
}

impl<T> PID<T>
where
    T: Float,
{
    pub fn new(kp: T, ki: T, kd: T) -> Self {
        PID {
            kp,
            ki,
            kd,
            last_input: T::zero(),
            last_integral: T::zero(),
            last_output: None,
            anti_windup: None,
        }
    }

    pub fn with_anti_windup(mut self, min: T, max: T) -> Self {
        self.anti_windup = Some((min, max));
        self
    }

    pub fn clear_integral(&mut self) {
        self.last_integral = T::zero();
    }

    pub fn integral(&self) -> &T {
        &self.last_integral
    }

    pub fn error(&self) -> &T {
        &self.last_input
    }

    pub fn kp_mut(&mut self) -> &mut T {
        &mut self.kp
    }

    pub fn ki_mut(&mut self) -> &mut T {
        &mut self.ki
    }

    pub fn kd_mut(&mut self) -> &mut T {
        &mut self.kd
    }
}

impl<T> Block for PID<T>
where
    T: Float,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let dt = T::from(sim_state.dt().as_secs_f32()).unwrap();
        let proportional = input;
        let derivative = (input - self.last_input) / dt;
        let integral_candidate = self.last_integral + input * dt;
        let output_candidate =
            self.kp * proportional + self.ki * integral_candidate + self.kd * derivative;

        let (output, integral) = if let Some((min, max)) = self.anti_windup {
            let should_hold_integral = (output_candidate > max && input > T::zero())
                || (output_candidate < min && input < T::zero());

            let integral = if should_hold_integral {
                self.last_integral
            } else {
                integral_candidate
            };

            let output = (self.kp * proportional + self.ki * integral + self.kd * derivative)
                .clamp(min, max);

            (output, integral)
        } else {
            (output_candidate, integral_candidate)
        };

        self.last_output = Some(output);
        self.last_input = input;
        self.last_integral = integral;

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }

    fn reset(&mut self) {
        self.last_input = T::zero();
        self.last_integral = T::zero();
        self.last_output = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;

    fn sim_state(dt: f32) -> SimulationState {
        Simulation::new(dt, dt).next().unwrap()
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn anti_windup_holds_integral_when_upper_saturation_worsens() {
        let mut pid = PID::new(10.0, 1.0, 0.0).with_anti_windup(0.0, 1.0);
        pid.last_integral = 5.0;

        let output = pid.block(0.1, sim_state(1.0));

        assert_close(output, 1.0);
        assert_close(*pid.integral(), 5.0);
    }

    #[test]
    fn anti_windup_allows_upper_saturation_to_unwind() {
        let mut pid = PID::new(10.0, 1.0, 0.0).with_anti_windup(0.0, 1.0);
        pid.last_integral = 5.0;

        let output = pid.block(-0.1, sim_state(1.0));

        assert_close(output, 1.0);
        assert_close(*pid.integral(), 4.9);
    }

    #[test]
    fn anti_windup_holds_integral_when_lower_saturation_worsens() {
        let mut pid = PID::new(10.0, 1.0, 0.0).with_anti_windup(0.0, 1.0);
        pid.last_integral = -5.0;

        let output = pid.block(-0.1, sim_state(1.0));

        assert_close(output, 0.0);
        assert_close(*pid.integral(), -5.0);
    }

    #[test]
    fn anti_windup_allows_lower_saturation_to_unwind() {
        let mut pid = PID::new(10.0, 1.0, 0.0).with_anti_windup(0.0, 1.0);
        pid.last_integral = -5.0;

        let output = pid.block(0.1, sim_state(1.0));

        assert_close(output, 0.0);
        assert_close(*pid.integral(), -4.9);
    }

    #[test]
    fn anti_windup_still_clamps_output_when_proportional_term_saturates() {
        let mut pid = PID::new(200.0, 1.0, 0.0).with_anti_windup(0.0, 1.0);
        pid.last_integral = -100.0;

        let output = pid.block(1.0, sim_state(1.0));

        assert_close(output, 1.0);
        assert_close(*pid.integral(), -99.0);
    }
}
