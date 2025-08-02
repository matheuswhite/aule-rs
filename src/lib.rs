mod block;
pub mod continuous;
mod discrete;
mod input;
mod monitor;
pub mod poly;
mod signal;
mod time;

pub mod prelude {
    pub use crate::block::gain::Gain;
    pub use crate::block::pid::PID;
    pub use crate::block::{AsBlock, Block};
    pub use crate::continuous::Tf;
    pub use crate::continuous::s_var::s;
    pub use crate::discrete::integration::euler::Euler;
    pub use crate::discrete::integration::{Discretizable, Integrator};
    pub use crate::input::impulse::Impulse;
    pub use crate::input::ramp::Ramp;
    pub use crate::input::setpoint::Setpoint;
    pub use crate::input::sinusoid::Sinusoid;
    pub use crate::input::step::Step;
    pub use crate::input::{AsInput, Input};
    #[cfg(feature = "graphics")]
    pub use crate::monitor::chart::Chart;
    #[cfg(feature = "graphics")]
    pub use crate::monitor::plotter::{Plotter, PlotterContext, keep_alive};
    pub use crate::monitor::printer::Printer;
    pub use crate::monitor::writer::Writter;
    pub use crate::monitor::{AsMonitor, Monitor};
    pub use crate::signal::Signal;
    pub use crate::time::Time;
    use ndarray::{Array, Dim};

    pub type Matrix<T> = Array<T, Dim<[usize; 2]>>;
}
