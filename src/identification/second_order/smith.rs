use crate::{
    identification::{
        find_time_at_value,
        second_order::{SecondOrderIdentification, SecondOrderModel, SecondOrderModelError},
    },
    math::float_point::{AsFloatPoint, FloatPoint},
    signal::Signal,
};
use core::marker::PhantomData;
use std::vec::Vec;

#[derive(Default)]
pub struct Smith2<T>
where
    T: FloatPoint,
{
    _marker: PhantomData<T>,
}

impl<T> Smith2<T>
where
    T: FloatPoint,
{
    fn zeta_table() -> [T; 9] {
        [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.5, 2.0, 3.0].map(|f| f.as_fp())
    }

    fn t20_tau_table() -> [T; 9] {
        [
            0.800, 0.952, 1.032, 1.102, 1.175, 1.257, 1.500, 1.800, 2.400,
        ]
        .map(|f| f.as_fp())
    }

    fn t60_tau_table() -> [T; 9] {
        [3.13, 3.14, 2.87, 2.50, 2.23, 2.02, 2.16, 2.42, 3.00].map(|f| f.as_fp())
    }

    fn ratio_table() -> [T; 9] {
        [
            0.255, 0.303, 0.360, 0.441, 0.528, 0.621, 0.693, 0.745, 0.800,
        ]
        .map(|f| f.as_fp())
    }

    fn table_indexes(ratio: T) -> (usize, usize) {
        if ratio == 0.255.as_fp() {
            return (0, 0);
        } else if ratio < 0.303.as_fp() {
            return (0, 1);
        } else if ratio == 0.303.as_fp() {
            return (1, 1);
        } else if ratio < 0.360.as_fp() {
            return (1, 2);
        } else if ratio == 0.360.as_fp() {
            return (2, 2);
        } else if ratio < 0.441.as_fp() {
            return (2, 3);
        } else if ratio == 0.441.as_fp() {
            return (3, 3);
        } else if ratio < 0.528.as_fp() {
            return (3, 4);
        } else if ratio == 0.528.as_fp() {
            return (4, 4);
        } else if ratio < 0.621.as_fp() {
            return (4, 5);
        } else if ratio == 0.621.as_fp() {
            return (5, 5);
        } else if ratio < 0.693.as_fp() {
            return (5, 6);
        } else if ratio == 0.693.as_fp() {
            return (6, 6);
        } else if ratio < 0.745.as_fp() {
            return (6, 7);
        } else if ratio == 0.745.as_fp() {
            return (7, 7);
        } else if ratio < 0.800.as_fp() {
            return (7, 8);
        } else if ratio == 0.800.as_fp() {
            return (8, 8);
        } else {
            panic!("Invalid ratio: {}", ratio)
        }
    }

    fn zeta(ratio: T) -> T {
        let ratio_table = Self::ratio_table();
        let zeta_table = Self::zeta_table();

        let (i0, i1) = Self::table_indexes(ratio);
        let alpha = (ratio - ratio_table[i0]) / (ratio_table[i1] - ratio_table[i0]);
        zeta_table[i0] + alpha * (zeta_table[i1] - zeta_table[i0])
    }

    fn t20_tau(ratio: T) -> T {
        let ratio_table = Self::ratio_table();
        let t20_tau_table = Self::t20_tau_table();

        let (i0, i1) = Self::table_indexes(ratio);
        let alpha = (ratio - ratio_table[i0]) / (ratio_table[i1] - ratio_table[i0]);
        t20_tau_table[i0] + alpha * (t20_tau_table[i1] - t20_tau_table[i0])
    }

    fn t60_tau(ratio: T) -> T {
        let ratio_table = Self::ratio_table();
        let t60_tau_table = Self::t60_tau_table();

        let (i0, i1) = Self::table_indexes(ratio);
        let alpha = (ratio - ratio_table[i0]) / (ratio_table[i1] - ratio_table[i0]);
        t60_tau_table[i0] + alpha * (t60_tau_table[i1] - t60_tau_table[i0])
    }
}

impl<T> SecondOrderIdentification<T> for Smith2<T>
where
    T: FloatPoint,
{
    fn from_step_response(
        &self,
        signals: Vec<Signal<T>>,
    ) -> Result<SecondOrderModel<T>, SecondOrderModelError<T>> {
        let signals_cloned = signals.clone();
        let mut signals = signals.into_iter().peekable();

        let oh_oh_two: T = 0.02.as_fp();
        let oh_two: T = 0.2.as_fp();
        let two_five_five: T = 0.255.as_fp();
        let oh_six: T = 0.6.as_fp();

        let y0 = signals
            .peek()
            .ok_or(SecondOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(SecondOrderModelError::NotEnoughSamples)?
            .value;

        let y20 = oh_two * yf + y0;
        let y60 = oh_six * yf + y0;

        let t20 = find_time_at_value(signals_cloned.clone().into_iter(), y20)
            .ok_or(SecondOrderModelError::TimeNotfound)?;
        let t60 = find_time_at_value(signals_cloned.clone().into_iter(), y60)
            .ok_or(SecondOrderModelError::TimeNotfound)?;

        let y_theta = oh_oh_two * yf + y0;
        let theta = find_time_at_value(signals_cloned.into_iter(), y_theta)
            .ok_or(SecondOrderModelError::TimeNotfound)?;
        let t20 = t20 - theta;
        let t60 = t60 - theta;

        let ratio = t20 / t60;

        if !(two_five_five <= ratio && ratio <= 0.800.as_fp()) {
            return Err(SecondOrderModelError::ParameterOutOfRange {
                parameter: ratio,
                min: 0.255.as_fp(),
                max: 0.800.as_fp(),
            });
        }

        let zeta = Self::zeta(ratio);
        let t20_tau = Self::t20_tau(ratio);
        let t60_tau = Self::t60_tau(ratio);

        let tau1 = t20 / t20_tau;
        let tau2 = t60 / t60_tau;
        let tau = (tau1 + tau2) / 2.0.as_fp();
        let omega_n = T::one() / tau;

        Ok(SecondOrderModel {
            k: yf,
            theta,
            zeta,
            omega_n,
        })
    }
}
