use std::vec::Vec;

use crate::{
    identification::{
        find_time_at_value,
        first_order::{FirstOrderIdentification, FirstOrderModel, FirstOrderModelError},
    },
    signal::Signal,
};

pub struct Smith1;

impl FirstOrderIdentification for Smith1 {
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
        let y283 = yf * 0.283 + y0;
        let y632 = yf * 0.632 + y0;

        let t1 = find_time_at_value(signal_cloned.clone().into_iter(), y283)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let t2 = find_time_at_value(signal_cloned.into_iter(), y632)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let tau = 1.5 * (t2 - t1);
        let theta = t2 - tau;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
