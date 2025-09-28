#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod block;
#[cfg(feature = "alloc")]
pub mod continuous;
#[cfg(feature = "alloc")]
mod discrete;
mod input;
mod metrics;
#[cfg(feature = "alloc")]
mod output;
#[cfg(feature = "alloc")]
pub mod poly;
mod signal;
mod time;

#[cfg(feature = "alloc")]
pub use crate::continuous::s_var::s;

pub mod prelude {
    pub use crate::block::delay::Delay;
    pub use crate::block::mimo::{AsMIMO, MIMO};
    pub use crate::block::observer::Observer;
    pub use crate::block::pid::PID;
    pub use crate::block::saturation::Saturation;
    pub use crate::block::siso::{AsSISO, SISO};
    #[cfg(feature = "alloc")]
    pub use crate::continuous::Tf;
    #[cfg(feature = "alloc")]
    pub use crate::continuous::ss::SS;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::solver::Solver;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::solver::StateEstimation;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::solver::euler::Euler;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::solver::runge_kutta::RK4;
    pub use crate::input::impulse::Impulse;
    pub use crate::input::ramp::Ramp;
    pub use crate::input::sawtooth::Sawtooth;
    pub use crate::input::sinusoid::Sinusoid;
    pub use crate::input::square::Square;
    pub use crate::input::step::Step;
    pub use crate::input::{AsInput, Input};
    #[cfg(feature = "alloc")]
    pub use crate::metrics::good_hart::GoodHart;
    pub use crate::metrics::iae::IAE;
    pub use crate::metrics::ise::ISE;
    pub use crate::metrics::itae::ITAE;
    pub use crate::metrics::{AsMetric, Metric};
    #[cfg(feature = "std")]
    pub use crate::output::plotter::{JoinAll, Joinable, Plotter, RTPlotter, Savable};
    #[cfg(feature = "alloc")]
    pub use crate::output::printer::Printer;
    #[cfg(feature = "alloc")]
    pub use crate::output::writer::Writter;
    pub use crate::output::{AsOutput, Output};
    pub use crate::signal::Signal;
    #[cfg(feature = "std")]
    pub use crate::time::RTTime;
    pub use crate::time::Time;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use core::time::Duration;

    #[test]
    fn test_no_std_support() {
        let time = Time::from(0.1).with_max_time(Duration::from_secs(1));
        let mut step = Step::default();
        let mut pid = PID::new(1.0, 0.1, 0.01);

        for dt in time {
            let r = step.output(dt);
            let y = pid.output(r);
            let _ = (r, y);
        }
    }
}
