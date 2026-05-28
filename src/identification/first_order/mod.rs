use crate::{
    math::float_point::FloatPoint,
    prelude::{Delay, Tf},
    signal::Signal,
};
use core::fmt::Display;
use std::vec::Vec;

pub mod hagglund;
pub mod smith;
pub mod sundaresan_krishnaswamy;
pub mod ziegler_nichols;

#[derive(Debug, Clone, PartialEq)]
pub struct FirstOrderModel<T>
where
    T: FloatPoint,
{
    pub k: T,
    pub tau: T,
    pub theta: T,
}

#[derive(Debug)]
pub enum FirstOrderModelError<T>
where
    T: FloatPoint,
{
    NegativeTheta(T),
    NotEnoughSamples,
    TimeNotfound,
}

impl<T> TryFrom<FirstOrderModel<T>> for (Tf<T>, Delay<T>)
where
    T: FloatPoint + 'static,
{
    type Error = FirstOrderModelError<T>;

    fn try_from(value: FirstOrderModel<T>) -> Result<Self, Self::Error> {
        if value.theta.is_sign_negative() {
            return Err(FirstOrderModelError::NegativeTheta(value.theta));
        }

        let tf = Tf::new(&[value.k], &[value.tau, T::one()]);
        let delay = Delay::<T>::new(value.theta.to_duration());

        Ok((tf, delay))
    }
}

impl<T> Display for FirstOrderModel<T>
where
    T: FloatPoint,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K: {}, θ: {}, τ: {}", self.k, self.theta, self.tau)
    }
}

pub trait FirstOrderIdentification<T>
where
    T: FloatPoint,
{
    #[allow(clippy::wrong_self_convention)]
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<FirstOrderModel<T>, FirstOrderModelError<T>>;
}
