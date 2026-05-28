use crate::{
    identification::first_order::{
        FirstOrderIdentification, FirstOrderModel, FirstOrderModelError,
    },
    math::{float_point::FloatPoint, line_equation::LineEquationError},
    prelude::LineEquation,
    signal::Signal,
};
use std::vec::Vec;

pub struct ZieglerNichols;

impl<T> FirstOrderIdentification<T> for ZieglerNichols
where
    T: FloatPoint,
{
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<FirstOrderModel<T>, FirstOrderModelError<T>> {
        let line_eq = LineEquation::from_signals_with_maximum_slope(signals.clone().into_iter())
            .map_err(|err| match err {
                LineEquationError::NotEnoughSignals => FirstOrderModelError::NotEnoughSamples,
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

        let t1 = line_eq.time_at(y0);
        let t3 = line_eq.time_at(yf);

        let theta = t1;
        let tau = t3 - t1;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
