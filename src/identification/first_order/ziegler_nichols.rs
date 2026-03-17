use std::vec::Vec;

use crate::{
    identification::first_order::{
        FirstOrderIdentification, FirstOrderModel, FirstOrderModelError,
    },
    line_equation::LineEquationError,
    prelude::LineEquation,
    signal::Signal,
};

pub struct ZieglerNichols;

impl FirstOrderIdentification for ZieglerNichols {
    fn from_step_response(
        &self,
        signals: Vec<Signal<f64>>,
    ) -> Result<FirstOrderModel, FirstOrderModelError> {
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

        let t1 = line_eq.time_at(y0) as f64;
        let t3 = line_eq.time_at(yf) as f64;

        let theta = t1;
        let tau = t3 - t1;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
