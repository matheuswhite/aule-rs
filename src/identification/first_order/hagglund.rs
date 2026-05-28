use crate::{
    identification::first_order::{
        FirstOrderIdentification, FirstOrderModel, FirstOrderModelError,
    },
    math::float_point::{AsFloatPoint, FloatPoint},
    prelude::LineEquation,
    signal::Signal,
};
use std::vec::Vec;

pub struct Hagglund;

impl<T> FirstOrderIdentification<T> for Hagglund
where
    T: FloatPoint,
{
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<FirstOrderModel<T>, FirstOrderModelError<T>> {
        let line_eq = LineEquation::from_signals_with_maximum_slope(signals.clone().into_iter())
            .map_err(|err| match err {
                crate::math::line_equation::LineEquationError::NotEnoughSignals => {
                    FirstOrderModelError::NotEnoughSamples
                }
            })?;

        let mut signals = signals.into_iter().peekable();
        let y0 = signals
            .peek()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let y632 = yf * 0.632.as_fp() + y0;

        let t1 = line_eq.time_at(y0);
        let t2 = line_eq.time_at(y632);
        let theta = t1;
        let tau = t2 - t1;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
