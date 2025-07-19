mod block;
mod gain;
mod pid;
mod plotter;
mod printer;
mod s;
mod setpoint;
mod ss;
mod step;
mod tf;

pub mod prelude {
    pub use crate::block::{AsBlock, AsMonitor, Block, Input, Monitor, Signal};
    pub use crate::gain::Gain;
    pub use crate::pid::PID;
    pub use crate::plotter::{Plotter, PlotterContext, keep_alive};
    pub use crate::printer::Printer;
    pub use crate::s::s;
    pub use crate::setpoint::Setpoint;
    pub use crate::ss::StateSpace;
    pub use crate::step::Step;
    pub use crate::tf::Tf;
}

// 2nd part
// impl ToVerilog for Motor {
//     fn to_verilog(&self) -> Verilog {
//         todo!()
//     }
// }

// let verilog_code = Verilog::empty_code() >> (setpoint - motor) >> pid >> motor;
