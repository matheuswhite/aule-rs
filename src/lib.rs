mod block;
pub mod chart;
mod continuous;
mod discrete;
mod gain;
mod pid;
mod plotter;
mod poly;
mod printer;
mod setpoint;
mod step;
pub mod writer;

pub mod prelude {
    pub use crate::block::{AsBlock, AsMonitor, Block, Input, Monitor, Signal};
    pub use crate::continuous::Tf;
    pub use crate::continuous::s_var::s;
    pub use crate::discrete::integration::euler::Euler;
    pub use crate::discrete::integration::{Discretizable, Integrator};
    pub use crate::gain::Gain;
    pub use crate::pid::PID;
    pub use crate::plotter::{Plotter, PlotterContext, keep_alive};
    pub use crate::printer::Printer;
    pub use crate::setpoint::Setpoint;
    pub use crate::step::Step;
    use ndarray::{Array, Dim};

    pub type Matrix<T> = Array<T, Dim<[usize; 2]>>;
}
