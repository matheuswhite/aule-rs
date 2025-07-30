use crate::block::Block;

pub mod euler;
pub mod runge_kutta;

pub trait Discretizable<T> {
    fn discretize(self) -> T
    where
        T: Integrator;
}

pub trait Integrator: Block {}
