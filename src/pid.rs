use crate::block::{AsBlock, Block, Signal};

pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    last_input: f32,
    last_integral: f32,
    last_output: Option<Signal>,
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PID {
            kp,
            ki,
            kd,
            last_input: 0.0,
            last_integral: 0.0,
            last_output: None,
        }
    }
}

impl Block for PID {
    fn output(&mut self, input: Signal) -> Signal {
        let proportional = input.value;
        let integral = self.last_integral + input.value * input.dt.as_secs_f32();
        let derivative = (input.value - self.last_input) / input.dt.as_secs_f32();

        let output = self.kp * proportional + self.ki * integral + self.kd * derivative;
        let output = Signal {
            value: output,
            dt: input.dt,
        };

        self.last_output = Some(output);
        self.last_input = input.value;
        self.last_integral = integral;

        output
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for PID {}
