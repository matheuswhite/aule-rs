mod block;
#[cfg(feature = "graphics")]
pub mod chart;
pub mod continuous;
mod discrete;
mod gain;
mod input;
mod pid;
#[cfg(feature = "graphics")]
mod plotter;
pub mod poly;
mod printer;
mod time;
pub mod writer;

pub mod prelude {
    pub use crate::block::{AsBlock, AsMonitor, Block, Monitor, Signal};
    pub use crate::continuous::Tf;
    pub use crate::continuous::s_var::s;
    pub use crate::discrete::integration::euler::Euler;
    pub use crate::discrete::integration::{Discretizable, Integrator};
    pub use crate::gain::Gain;
    pub use crate::input::setpoint::Setpoint;
    pub use crate::input::step::Step;
    pub use crate::input::{AsInput, Input};
    pub use crate::pid::PID;
    #[cfg(feature = "graphics")]
    pub use crate::plotter::{Plotter, PlotterContext, keep_alive};
    pub use crate::printer::Printer;
    pub use crate::time::Time;
    use ndarray::{Array, Dim};

    pub type Matrix<T> = Array<T, Dim<[usize; 2]>>;
}
