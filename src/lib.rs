#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod block;
#[cfg(feature = "alloc")]
pub mod continuous;
#[cfg(feature = "alloc")]
mod discrete;
mod input;
mod metrics;
#[cfg(feature = "std")]
mod output;
#[cfg(feature = "alloc")]
pub mod poly;
mod signal;
mod tier1;
pub mod tier2;
pub mod tier3;
mod time;

#[cfg(feature = "alloc")]
pub use crate::continuous::s_var::s;
#[cfg(feature = "alloc")]
pub use crate::discrete::z_inv_var::z_inv;
#[cfg(feature = "alloc")]
pub use crate::discrete::z_var::z;

pub mod prelude {
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
    pub use crate::input::impulse::Impulse;
    pub use crate::input::ramp::Ramp;
    pub use crate::input::sawtooth::Sawtooth;
    pub use crate::input::sinusoid::Sinusoid;
    pub use crate::input::square::Square;
    pub use crate::input::step::Step;
    #[cfg(feature = "alloc")]
    pub use crate::metrics::good_hart::GoodHart;
    pub use crate::metrics::iae::IAE;
    pub use crate::metrics::ise::ISE;
    pub use crate::metrics::itae::ITAE;
    #[cfg(feature = "std")]
    pub use crate::output::plotter::{JoinAll, Joinable, Plotter, RTPlotter, Savable};
    #[cfg(feature = "std")]
    pub use crate::output::printer::Printer;
    #[cfg(feature = "std")]
    pub use crate::output::writer::Writter;
    pub use crate::signal::{Pack, Signal, Unpack};
    #[cfg(all(feature = "alloc", feature = "swd"))]
    pub use crate::tier1::bridge::{BridgeSwdDown, BridgeSwdUp, RemoteSwd, SwdConnection};
    #[cfg(feature = "alloc")]
    pub use crate::tier1::delay::Delay;
    #[cfg(feature = "alloc")]
    pub use crate::tier1::observer::Observer;
    pub use crate::tier1::pid::PID;
    pub use crate::tier1::saturation::Saturation;
    pub use crate::time::{Delta, EndlessTime, Time};
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_no_std_support() {
        let time = Time::new(0.1, 1.0);
        let mut step = Step::default();
        let mut pid = PID::new(1.0, 0.1, 0.01);

        for dt in time {
            let r = step.output(dt);
            let _y = pid.output(r);
        }
    }
}
