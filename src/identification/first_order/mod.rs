use crate::{
    prelude::{Delay, Tf},
    signal::Signal,
};
use core::{fmt::Display, time::Duration};
use std::vec::Vec;

pub mod hagglund;
pub mod smith;
pub mod sundaresan_krishnaswamy;
pub mod ziegler_nichols;

#[derive(Debug, Clone, PartialEq)]
pub struct FirstOrderModel {
    pub k: f64,
    pub tau: f64,
    pub theta: f64,
}

#[derive(Debug)]
pub enum FirstOrderModelError {
    NegativeTheta(f64),
    NotEnoughSamples,
    TimeNotfound,
}

impl TryFrom<FirstOrderModel> for (Tf<f64>, Delay<f64>) {
    type Error = FirstOrderModelError;

    fn try_from(value: FirstOrderModel) -> Result<Self, Self::Error> {
        if value.theta.is_sign_negative() {
            return Err(FirstOrderModelError::NegativeTheta(value.theta));
        }

        let tf = Tf::new(&[value.k], &[value.tau, 1.0]);
        let delay = Delay::<f64>::new(Duration::from_secs_f64(value.theta));

        Ok((tf, delay))
    }
}

impl Display for FirstOrderModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K: {}, θ: {}, τ: {}", self.k, self.theta, self.tau)
    }
}

pub trait FirstOrderIdentification {
    #[allow(clippy::wrong_self_convention)]
    fn from_step_response(
        &self,
        signals: Vec<Signal<f64>>,
    ) -> Result<FirstOrderModel, FirstOrderModelError>;
}
