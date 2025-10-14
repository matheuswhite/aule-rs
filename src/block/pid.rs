use crate::block::Block;
use crate::merge;
use crate::metrics::Metric;
#[cfg(feature = "alloc")]
use crate::prelude::GoodHart;
use crate::prelude::{IAE, ISE, ITAE};
use crate::signal::Signal;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    last_input: f32,
    last_integral: f32,
    last_output: Option<Signal<f32>>,
    iae: Option<IAE>,
    ise: Option<ISE>,
    itae: Option<ITAE>,
    #[cfg(feature = "alloc")]
    good_hart: Option<GoodHart>,
    anti_windup: Option<(f32, f32)>,
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
            iae: None,
            ise: None,
            itae: None,
            #[cfg(feature = "alloc")]
            good_hart: None,
            anti_windup: None,
        }
    }

    pub fn with_iae(mut self) -> Self {
        self.iae = Some(IAE::default());
        self
    }

    pub fn with_ise(mut self) -> Self {
        self.ise = Some(ISE::default());
        self
    }

    pub fn with_itae(mut self) -> Self {
        self.itae = Some(ITAE::default());
        self
    }

    #[cfg(feature = "alloc")]
    pub fn with_good_hart(mut self, alpha1: f32, alpha2: f32, alpha3: f32) -> Self {
        self.good_hart = Some(GoodHart::new(alpha1, alpha2, alpha3));
        self
    }

    pub fn with_anti_windup(mut self, min: f32, max: f32) -> Self {
        self.anti_windup = Some((min, max));
        self
    }

    #[cfg(feature = "alloc")]
    pub fn error_metrics(&self) -> String {
        format!(
            "\n  IAE: {}\n  ISE: {}\n  ITAE: {}\n  Good Hart: {}",
            self.iae
                .as_ref()
                .map_or("N/A".to_string(), |e| e.value().to_string()),
            self.ise
                .as_ref()
                .map_or("N/A".to_string(), |e| e.value().to_string()),
            self.itae
                .as_ref()
                .map_or("N/A".to_string(), |e| e.value().to_string()),
            self.good_hart
                .as_ref()
                .map_or("N/A".to_string(), |gh| gh.value().to_string())
        )
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

impl Block for PID {
    type Input = f32;
    type Output = f32;

    fn output(&mut self, input: Signal<f32>) -> Signal<f32> {
        if let Some(iae) = &mut self.iae {
            iae.update(input.clone());
        }

        if let Some(ise) = &mut self.ise {
            ise.update(input.clone());
        }

        if let Some(itae) = &mut self.itae {
            itae.update(input.clone());
        }

        let proportional = input.value;
        let integral = self.last_integral + input.value * input.dt.as_secs_f32();
        let derivative = (input.value - self.last_input) / input.dt.as_secs_f32();

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

        let output = Signal {
            value: output,
            dt: input.dt,
        };

        self.last_output = Some(output.clone());
        self.last_input = input.value;
        self.last_integral = integral;

        #[cfg(feature = "alloc")]
        if let Some(good_hart) = &mut self.good_hart {
            good_hart.update(merge!(input, output));
        }

        output
    }

    fn last_output(&self) -> Option<Signal<f32>> {
        self.last_output.clone()
    }
}
