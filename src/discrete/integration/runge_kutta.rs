use crate::{
    block::{AsBlock, Block},
    prelude::Integrator,
};

pub struct RungeKutta;

impl Block for RungeKutta {
    fn output(&mut self, input: crate::prelude::Signal) -> crate::prelude::Signal {
        todo!()
    }

    fn last_output(&self) -> Option<crate::prelude::Signal> {
        todo!()
    }
}

impl Integrator for RungeKutta {}

impl AsBlock for RungeKutta {}
