use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;
use core::marker::PhantomData;
use core::ops::{Div, Mul, Sub};
use num_traits::{Zero, clamp};

#[derive(Debug, Clone, PartialEq)]
pub struct PID<T, K>
where
    T: Zero
        + Copy
        + Mul<f64, Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<f64, Output = T>
        + PartialOrd,
    K: TimeType,
{
    kp: T,
    ki: T,
    kd: T,
    last_input: T,
    last_integral: T,
    last_output: Option<T>,
    anti_windup: Option<(T, T)>,
    _marker: PhantomData<K>,
}

impl<T, K> PID<T, K>
where
    T: Zero
        + Copy
        + Mul<f64, Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<f64, Output = T>
        + PartialOrd,
    K: TimeType + Default,
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
            _marker: PhantomData,
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

impl<T, K> Block for PID<T, K>
where
    T: Zero
        + Copy
        + Mul<f64, Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<f64, Output = T>
        + PartialOrd,
    K: TimeType + 'static,
{
    type Input = T;
    type Output = T;
    type TimeType = K;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let proportional = input.value;
        let integral = self.last_integral + input.value * input.delta.dt().as_secs_f64();
        let derivative = (input.value - self.last_input) / input.delta.dt().as_secs_f64();

        let output = self.kp * proportional + self.ki * integral + self.kd * derivative;
        let (output, integral) = if let Some((min, max)) = self.anti_windup {
            if output < min || output > max {
                (clamp(output, min, max), self.last_integral)
            } else {
                (output, integral)
            }
        } else {
            (output, integral)
        };

        let output = input.replace(output);

        self.last_output = Some(output.value);
        self.last_input = input.value;
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
