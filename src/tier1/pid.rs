use crate::block::Block;
use crate::signal::Signal;
use crate::time::TimeType;
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct PID<TT>
where
    TT: TimeType,
{
    kp: f32,
    ki: f32,
    kd: f32,
    last_input: f32,
    last_integral: f32,
    last_output: Option<f32>,
    anti_windup: Option<(f32, f32)>,
    _marker: PhantomData<TT>,
}

impl<TT> PID<TT>
where
    TT: TimeType + Default,
{
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PID {
            kp,
            ki,
            kd,
            last_input: 0.0,
            last_integral: 0.0,
            last_output: None,
            anti_windup: None,
            _marker: PhantomData,
        }
    }

    pub fn with_anti_windup(mut self, min: f32, max: f32) -> Self {
        self.anti_windup = Some((min, max));
        self
    }

    pub fn clear_integral(&mut self) {
        self.last_integral = 0.0;
    }

    pub fn integral(&self) -> f32 {
        self.last_integral
    }

    pub fn error(&self) -> f32 {
        self.last_input
    }

    pub fn kp_mut(&mut self) -> &mut f32 {
        &mut self.kp
    }

    pub fn ki_mut(&mut self) -> &mut f32 {
        &mut self.ki
    }

    pub fn kd_mut(&mut self) -> &mut f32 {
        &mut self.kd
    }
}

impl<TT> Block for PID<TT>
where
    TT: TimeType + 'static,
{
    type Input = f32;
    type Output = f32;
    type TimeType = TT;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let proportional = input.value;
        let integral = self.last_integral + input.value * input.delta.dt().as_secs_f32();
        let derivative = (input.value - self.last_input) / input.delta.dt().as_secs_f32();

        let output = self.kp * proportional + self.ki * integral + self.kd * derivative;
        let (output, integral) = if let Some((min, max)) = self.anti_windup {
            if output < min || output > max {
                (output.clamp(min, max), self.last_integral)
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
        self.last_input = 0.0;
        self.last_integral = 0.0;
        self.last_output = None;
    }
}
