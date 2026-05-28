use crate::{
    identification::{
        find_time_at_value,
        second_order::{SecondOrderIdentification, SecondOrderModel, SecondOrderModelError},
    },
    math::float_point::{AsFloatPoint, FloatPoint},
    signal::Signal,
};
use std::vec::Vec;

pub struct Mollenkamp;

impl<T> SecondOrderIdentification<T> for Mollenkamp
where
    T: FloatPoint,
{
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<SecondOrderModel<T>, SecondOrderModelError<T>> {
        let signals_cloned = signals.clone();
        let mut signals = signals.into_iter().peekable();

        let four_seven_five: T = 0.475.as_fp();
        let two_eight_eleven: T = 2.811.as_fp();
        let one_six_six: T = 1.66.as_fp();
        let oh_eight_oh_five: T = 0.0805.as_fp();
        let five_five_four_seven: T = 5.547.as_fp();
        let seven_oh_eight: T = 0.78.as_fp();
        let two_six: T = 2.6.as_fp();
        let nine_two_two: T = 0.922.as_fp();

        let y0 = signals
            .peek()
            .ok_or(SecondOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(SecondOrderModelError::NotEnoughSamples)?
            .value;
        let y15 = yf * 0.15.as_fp() + y0;
        let y45 = yf * 0.45.as_fp() + y0;
        let y75 = yf * 0.75.as_fp() + y0;

        let t1 = find_time_at_value(signals_cloned.clone().into_iter(), y15)
            .ok_or(SecondOrderModelError::TimeNotfound)?;
        let t2 = find_time_at_value(signals_cloned.clone().into_iter(), y45)
            .ok_or(SecondOrderModelError::TimeNotfound)?;
        let t3 = find_time_at_value(signals_cloned.into_iter(), y75)
            .ok_or(SecondOrderModelError::TimeNotfound)?;

        let x = (t2 - t1) / (t3 - t1);

        let zeta: T = (oh_eight_oh_five
            - five_five_four_seven * (four_seven_five - x).power(2.0.as_fp()))
            / (x - 0.356.as_fp());

        let f2 = if zeta < 1.0.as_fp() {
            seven_oh_eight * two_eight_eleven.power(zeta)
        } else {
            two_six * zeta - 0.60.as_fp()
        };

        let omega_n = f2 / (t3 - t1);

        let f3 = nine_two_two * one_six_six.power(zeta);

        let theta = t2 - (f3 / omega_n);

        Ok(SecondOrderModel {
            k: yf,
            theta,
            zeta,
            omega_n,
        })
    }
}
