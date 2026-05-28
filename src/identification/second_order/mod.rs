use crate::{
    math::float_point::{AsFloatPoint, FloatPoint},
    prelude::{Delay, Tf},
    signal::Signal,
};
use core::fmt::Display;
use std::vec::Vec;

pub mod mollenkamp;
pub mod smith;

#[derive(Debug, Clone, PartialEq)]
pub struct SecondOrderModel<T>
where
    T: FloatPoint,
{
    pub k: T,
    pub theta: T,
    pub zeta: T,
    pub omega_n: T,
}

#[derive(Debug)]
pub enum SecondOrderModelError<T>
where
    T: FloatPoint,
{
    NegativeTheta(T),
    NotEnoughSamples,
    TimeNotfound,
    ParameterOutOfRange { parameter: T, min: T, max: T },
}

impl<T> TryFrom<SecondOrderModel<T>> for (Tf<T>, Delay<T>)
where
    T: FloatPoint + 'static,
{
    type Error = SecondOrderModelError<T>;

    fn try_from(value: SecondOrderModel<T>) -> Result<Self, Self::Error> {
        if value.theta.is_sign_negative() {
            return Err(SecondOrderModelError::NegativeTheta(value.theta));
        }

        let two: T = 2.0.as_fp();

        let omega_n2 = value.omega_n.power(2.0.as_fp());
        let tf = Tf::new(
            &[value.k * omega_n2],
            &[1.0.as_fp(), two * value.zeta * value.omega_n, omega_n2],
        );
        let delay = Delay::<T>::new(value.theta.to_duration());

        Ok((tf, delay))
    }
}

impl<T> Display for SecondOrderModel<T>
where
    T: FloatPoint,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "K: {}, θ: {}, ζ: {}, ωn: {}",
            self.k, self.theta, self.zeta, self.omega_n
        )
    }
}

pub trait SecondOrderIdentification<T>
where
    T: FloatPoint,
{
    #[allow(clippy::wrong_self_convention)]
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<SecondOrderModel<T>, SecondOrderModelError<T>>;
}
