use crate::{
    prelude::{Delay, Tf},
    signal::Signal,
};
use core::{fmt::Display, time::Duration};
use std::vec::Vec;

pub mod mollenkamp;
pub mod smith;

#[derive(Debug, Clone, PartialEq)]
pub struct SecondOrderModel {
    pub k: f64,
    pub theta: f64,
    pub zeta: f64,
    pub omega_n: f64,
}

#[derive(Debug)]
pub enum SecondOrderModelError {
    NegativeTheta(f64),
    NotEnoughSamples,
    TimeNotfound,
    ParameterOutOfRange { parameter: f64, min: f64, max: f64 },
}

impl TryFrom<SecondOrderModel> for (Tf<f64>, Delay<f64>) {
    type Error = SecondOrderModelError;

    fn try_from(value: SecondOrderModel) -> Result<Self, Self::Error> {
        if value.theta.is_sign_negative() {
            return Err(SecondOrderModelError::NegativeTheta(value.theta));
        }

        let omega_n2 = value.omega_n.powi(2);
        let tf = Tf::new(
            &[value.k * omega_n2],
            &[1.0, 2.0 * value.zeta * value.omega_n, omega_n2],
        );
        let delay = Delay::<f64>::new(Duration::from_secs_f64(value.theta));

        Ok((tf, delay))
    }
}

impl Display for SecondOrderModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "K: {}, θ: {}, ζ: {}, ωn: {}",
            self.k, self.theta, self.zeta, self.omega_n
        )
    }
}

pub trait SecondOrderIdentification {
    fn from_step_response(
        &self,
        signals: Vec<Signal<f64>>,
    ) -> Result<SecondOrderModel, SecondOrderModelError>;
}
