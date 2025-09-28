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
mod error;
mod input;
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
    pub use crate::discrete::integration::Integrator;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::integration::StateEstimation;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::integration::euler::Euler;
    #[cfg(feature = "alloc")]
    pub use crate::discrete::integration::runge_kutta::RK4;
    #[cfg(feature = "alloc")]
    pub use crate::error::good_hart::GoodHart;
    pub use crate::error::iae::IAE;
    pub use crate::error::ise::ISE;
    pub use crate::error::itae::ITAE;
    pub use crate::error::{AsErrorMetric, ErrorMetric};
    pub use crate::input::impulse::Impulse;
    pub use crate::input::ramp::Ramp;
    pub use crate::input::setpoint::Setpoint;
    pub use crate::input::sinusoid::Sinusoid;
    pub use crate::input::step::Step;
    pub use crate::input::{AsInput, Input};
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
        let mut step = Step::new();
        let mut pid = PID::new(1.0, 0.1, 0.01);

        for dt in time {
            let r = step.output(dt);
            let y = pid.output(r);
            let _ = (r, y);
        }
    }
}
