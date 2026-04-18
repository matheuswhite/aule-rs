use std::vec::Vec;

use crate::{
    identification::{
        find_time_at_value,
        second_order::{SecondOrderIdentification, SecondOrderModel, SecondOrderModelError},
    },
    signal::Signal,
};

pub struct Smith2;

impl Smith2 {
    const ZETA: [f64; 9] = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.5, 2.0, 3.0];
    const T20_TAU: [f64; 9] = [
        0.800, 0.952, 1.032, 1.102, 1.175, 1.257, 1.500, 1.800, 2.400,
    ];
    #[allow(clippy::approx_constant)]
    const T60_TAU: [f64; 9] = [3.13, 3.14, 2.87, 2.50, 2.23, 2.02, 2.16, 2.42, 3.00];
    const RATIO: [f64; 9] = [
        0.255, 0.303, 0.360, 0.441, 0.528, 0.621, 0.693, 0.745, 0.800,
    ];

    fn table_indexes(ratio: f64) -> (usize, usize) {
        match ratio {
            0.255 => (0, 0),
            x if x < 0.303 => (0, 1),
            0.303 => (1, 1),
            x if x < 0.360 => (1, 2),
            0.360 => (2, 2),
            x if x < 0.441 => (2, 3),
            0.441 => (3, 3),
            x if x < 0.528 => (3, 4),
            0.528 => (4, 4),
            x if x < 0.621 => (4, 5),
            0.621 => (5, 5),
            x if x < 0.693 => (5, 6),
            0.693 => (6, 6),
            x if x < 0.745 => (6, 7),
            0.745 => (7, 7),
            x if x < 0.800 => (7, 8),
            0.800 => (8, 8),
            _ => panic!("Invalid ratio: {}", ratio),
        }
    }

    fn zeta(ratio: f64) -> f64 {
        let (i0, i1) = Self::table_indexes(ratio);
        let alpha = (ratio - Self::RATIO[i0]) / (Self::RATIO[i1] - Self::RATIO[i0]);
        Self::ZETA[i0] + alpha * (Self::ZETA[i1] - Self::ZETA[i0])
    }

    fn t20_tau(ratio: f64) -> f64 {
        let (i0, i1) = Self::table_indexes(ratio);
        let alpha = (ratio - Self::RATIO[i0]) / (Self::RATIO[i1] - Self::RATIO[i0]);
        Self::T20_TAU[i0] + alpha * (Self::T20_TAU[i1] - Self::T20_TAU[i0])
    }

    fn t60_tau(ratio: f64) -> f64 {
        let (i0, i1) = Self::table_indexes(ratio);
        let alpha = (ratio - Self::RATIO[i0]) / (Self::RATIO[i1] - Self::RATIO[i0]);
        Self::T60_TAU[i0] + alpha * (Self::T60_TAU[i1] - Self::T60_TAU[i0])
    }
}

impl SecondOrderIdentification for Smith2 {
    fn from_step_response(
        &self,
        signals: Vec<Signal<f64>>,
    ) -> Result<super::SecondOrderModel, super::SecondOrderModelError> {
        let signals_cloned = signals.clone();
        let mut signals = signals.into_iter().peekable();

        let y0 = signals
            .peek()
            .ok_or(SecondOrderModelError::NotEnoughSamples)?
            .value;
        let yf = signals
            .last()
            .ok_or(SecondOrderModelError::NotEnoughSamples)?
            .value;

        let y20 = 0.2 * yf + y0;
        let y60 = 0.6 * yf + y0;

        let t20 = find_time_at_value(signals_cloned.clone().into_iter(), y20)
            .ok_or(SecondOrderModelError::TimeNotfound)?;
        let t60 = find_time_at_value(signals_cloned.clone().into_iter(), y60)
            .ok_or(SecondOrderModelError::TimeNotfound)?;

        let y_theta = 0.02 * yf + y0;
        let theta = find_time_at_value(signals_cloned.into_iter(), y_theta)
            .ok_or(SecondOrderModelError::TimeNotfound)?;
        let t20 = t20 - theta;
        let t60 = t60 - theta;

        let ratio = t20 / t60;

        if ratio < 0.255 || ratio > 0.800 {
            return Err(SecondOrderModelError::ParameterOutOfRange {
                parameter: ratio,
                min: 0.255,
                max: 0.800,
            });
        }

        let zeta = Self::zeta(ratio);
        let t20_tau = Self::t20_tau(ratio);
        let t60_tau = Self::t60_tau(ratio);

        let tau1 = t20 / t20_tau;
        let tau2 = t60 / t60_tau;
        let tau = (tau1 + tau2) / 2.0;
        let omega_n = 1.0 / tau;

        Ok(SecondOrderModel {
            k: yf,
            theta,
            zeta,
            omega_n,
        })
    }
}
