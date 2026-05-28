use crate::{
    identification::{
        find_time_at_value,
        first_order::{FirstOrderIdentification, FirstOrderModel, FirstOrderModelError},
    },
    math::float_point::{AsFloatPoint, FloatPoint},
    signal::Signal,
};
use std::vec::Vec;

pub struct SundaresanKrishnaswamy;

impl<T> FirstOrderIdentification<T> for SundaresanKrishnaswamy
where
    T: FloatPoint,
{
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<FirstOrderModel<T>, FirstOrderModelError<T>> {
        let signal_cloned = signals.clone();
        let mut signals = signals.into_iter().peekable();

        let sixty_seven_percent: T = 0.67.as_fp();
        let one_thirty_percent: T = 1.3.as_fp();
        let twenty_nine_percent: T = 0.29.as_fp();

        let y0 = signals
            .peek()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(FirstOrderModelError::NotEnoughSamples)?
            .value;
        let y353 = yf * 0.353.as_fp() + y0;
        let y853 = yf * 0.853.as_fp() + y0;

        let t1 = find_time_at_value(signal_cloned.clone().into_iter(), y353)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let t2 = find_time_at_value(signal_cloned.into_iter(), y853)
            .ok_or(FirstOrderModelError::TimeNotfound)?;
        let tau = sixty_seven_percent * (t2 - t1);
        let theta = one_thirty_percent * t1 - twenty_nine_percent * t2;
        let k = yf - y0;

        Ok(FirstOrderModel { k, tau, theta })
    }
}
