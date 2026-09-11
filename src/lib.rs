#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[forbid(unsafe_code)]
mod block;
#[forbid(unsafe_code)]
#[cfg(feature = "alloc")]
pub mod continuous;
#[forbid(unsafe_code)]
#[cfg(feature = "alloc")]
mod discrete;
#[forbid(unsafe_code)]
#[cfg(feature = "std")]
mod identification;
#[forbid(unsafe_code)]
mod input;
#[forbid(unsafe_code)]
pub mod math;
#[forbid(unsafe_code)]
mod metrics;
#[forbid(unsafe_code)]
#[cfg(feature = "std")]
mod output;
#[forbid(unsafe_code)]
mod signal;
#[forbid(unsafe_code)]
mod simulation;
mod tier1;
#[forbid(unsafe_code)]
pub mod tier2;
#[forbid(unsafe_code)]
pub mod tier3;

#[cfg(feature = "alloc")]
pub use crate::continuous::s_var::s;
#[cfg(feature = "alloc")]
pub use crate::discrete::z_inv_var::z_inv;
#[cfg(feature = "alloc")]
pub use crate::discrete::z_var::z;

pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use nalgebra::{DMatrix, dmatrix};

    pub use crate::block::Block;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::Tf;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::solver::Solver;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::solver::StateEstimation;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::solver::euler::Euler;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::solver::runge_kutta::RK4;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::ss::SS;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::ss::DSS;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::tf::DTf;
    #[cfg(feature = "std")]
    pub use crate::identification::first_order::{
        FirstOrderIdentification, FirstOrderModel, FirstOrderModelError, hagglund::Hagglund,
        smith::Smith1, sundaresan_krishnaswamy::SundaresanKrishnaswamy,
        ziegler_nichols::ZieglerNichols,
    };
    #[cfg(feature = "std")]
    pub use crate::identification::second_order::{
        SecondOrderIdentification, SecondOrderModel, SecondOrderModelError, mollenkamp::Mollenkamp,
        smith::Smith2,
    };
    #[cfg(feature = "std")]
    pub use crate::input::file_samples::FileSamples;
    pub use crate::input::impulse::Impulse;
    pub use crate::input::ramp::Ramp;
    pub use crate::input::sawtooth::Sawtooth;
    pub use crate::input::sinusoid::Sinusoid;
    pub use crate::input::square::Square;
    pub use crate::input::step::Step;
    pub use crate::math::line_equation::LineEquation;
    #[cfg(feature = "alloc")]
    pub use crate::metrics::good_hart::GoodHart;
    pub use crate::metrics::iae::IAE;
    pub use crate::metrics::ise::ISE;
    pub use crate::metrics::itae::ITAE;
    #[cfg(feature = "std")]
    pub use crate::output::plotter::{
        JoinAll, Joinable, LegendPosition, Plotter, PlotterDynamic, RTPlotter, Savable,
    };
    #[cfg(feature = "std")]
    pub use crate::output::printer::Printer;
    #[cfg(feature = "std")]
    pub use crate::output::writer::Writter;
    pub use crate::signal::{AsSignal, Pack, Signal, Unpack};
    pub use crate::simulation::{EndlessSimulation, Simulation, SimulationState};
    #[cfg(all(feature = "alloc", feature = "swd"))]
    pub use crate::tier1::bridge::{BridgeSwdDown, BridgeSwdUp, RemoteSwd, SwdConnection};
    #[cfg(feature = "alloc")]
    pub use crate::tier1::delay::Delay;
    pub use crate::tier1::filter::{
        Filter,
        first_order::{high_pass::HighPass, low_pass::LowPass},
        second_order::{
            band_pass::BandPass, band_stop::BandStop, bessel::Bessel, biquad::Biquad,
            butterworth::Butterworth, chebyshev1::Chebyshev1, chebyshev2::Chebyshev2,
        },
    };
    #[cfg(feature = "alloc")]
    pub use crate::tier1::observer::Observer;
    pub use crate::tier1::pid::PID;
    pub use crate::tier1::saturation::Saturation;
    #[cfg(target_has_atomic = "8")]
    pub use crate::tier1::sync::billboard::{Billboard, Poster, Viewer};
    pub use crate::tier1::sync::conveyor::{
        BlockingFeeder, BlockingPicker, Conveyor, Feeder, Picker,
    };
    pub use crate::tier1::sync::jackpot::{Claimer, Jackpot, Staker};
    pub use crate::tier1::sync::mirror::{Mirror, MirrorInput, MirrorOutput};
    pub use crate::tier1::sync::sync_policy::{AtomicPolicy, CriticalSectionPolicy};
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_no_std_support() {
        let simulation = Simulation::new(0.1, 1.0);
        let mut step = Step::default();
        let mut pid = PID::new(1.0, 0.1, 0.01);

        for sim_state in simulation {
            let r = sim_state * step.as_block();
            let _y = pid.output(r);
        }
    }
}
