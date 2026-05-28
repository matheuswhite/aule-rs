use crate::{
    identification::{
        find_time_at_value,
        first_order::{FirstOrderIdentification, FirstOrderModel, FirstOrderModelError},
    },
    math::float_point::{AsFloatPoint, FloatPoint},
    signal::Signal,
};
use std::vec::Vec;

pub struct Smith1;

impl<T> FirstOrderIdentification<T> for Smith1
where
    T: FloatPoint,
{
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<FirstOrderModel<T>, FirstOrderModelError<T>> {
        let signal_cloned = signals.clone();
        let mut signals = signals.into_iter().peekable();

        let one_half: T = 1.5.as_fp();

        let y0 = signals
            .peek()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let y283 = yf * 0.283.as_fp() + y0;
        let y632 = yf * 0.632.as_fp() + y0;

        let t1 = find_time_at_value(signal_cloned.clone().into_iter(), y283)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let t2 = find_time_at_value(signal_cloned.into_iter(), y632)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let tau = one_half * (t2 - t1);
        let theta = t2 - tau;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
