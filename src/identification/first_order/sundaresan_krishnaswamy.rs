use std::vec::Vec;

use crate::{
    identification::{
        find_time_at_value,
        first_order::{FirstOrderIdentification, FirstOrderModel, FirstOrderModelError},
    },
    signal::Signal,
};

pub struct SundaresanKrishnaswamy;

impl FirstOrderIdentification for SundaresanKrishnaswamy {
    fn from_step_response(
        &self,
        signals: Vec<Signal<f64>>,
    ) -> Result<FirstOrderModel, FirstOrderModelError> {
        let signal_cloned = signals.clone();
        let mut signals = signals.into_iter().peekable();

        let y0 = signals
            .peek()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let y353 = yf * 0.353 + y0;
        let y853 = yf * 0.853 + y0;

        let t1 = find_time_at_value(signal_cloned.clone().into_iter(), y353)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let t2 = find_time_at_value(signal_cloned.into_iter(), y853)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let tau = 0.67 * (t2 - t1);
        let theta = 1.3 * t1 - 0.29 * t2;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
