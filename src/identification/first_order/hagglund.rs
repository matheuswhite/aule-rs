use std::vec::Vec;

use crate::{
    identification::first_order::{
        FirstOrderIdentification, FirstOrderModel, FirstOrderModelError,
    },
    prelude::LineEquation,
    signal::Signal,
};

pub struct Hagglund;

impl FirstOrderIdentification for Hagglund {
    fn from_step_response(
        &self,
        signals: Vec<Signal<f64>>,
    ) -> Result<FirstOrderModel, FirstOrderModelError> {
        let line_eq = LineEquation::from_signals_with_maximum_slope(signals.clone().into_iter())
            .map_err(|err| match err {
                crate::line_equation::LineEquationError::NotEnoughSignals => {
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
        let y632 = yf * 0.632 + y0;

        let t1 = line_eq.time_at(y0) as f64;
        let t2 = line_eq.time_at(y632) as f64;
        let theta = t1;
        let tau = t2 - t1;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
