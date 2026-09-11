use crate::{
    block::Block,
    tier1::sync::sync_policy::{AtomicPolicy, SyncPolicy},
};
use core::marker::PhantomData;

pub struct Mirror<T, P = AtomicPolicy>
where
    P: SyncPolicy<T>,
{
    storage: P::Storage,
    _marker: PhantomData<T>,
}

impl<T, P> Mirror<T, P>
where
    P: SyncPolicy<T>,
{
    pub const fn new() -> Self {
        Self {
            storage: P::INIT,
            _marker: PhantomData,
        }
    }

    pub fn as_input(&'_ self) -> MirrorInput<'_, T, P> {
        MirrorInput { mirror: self }
    }

    pub fn as_output(&'_ self) -> MirrorOutput<'_, T, P> {
        MirrorOutput { mirror: self }
    }
}

pub struct MirrorInput<'a, T, P>
where
    P: SyncPolicy<T>,
{
    mirror: &'a Mirror<T, P>,
}
pub struct MirrorOutput<'a, T, P>
where
    P: SyncPolicy<T>,
{
    mirror: &'a Mirror<T, P>,
}

impl<'a, T, P> Block for MirrorInput<'a, T, P>
where
    P: SyncPolicy<T>,
{
    type Input = ();
    type Output = T;

    fn block(
        &mut self,
        _input: Self::Input,
        _sim_state: crate::prelude::SimulationState,
    ) -> Self::Output {
        P::get(&self.mirror.storage)
    }
}

impl<'a, T, P> Block for MirrorOutput<'a, T, P>
where
    P: SyncPolicy<T>,
{
    type Input = T;
    type Output = ();

    fn block(
        &mut self,
        input: Self::Input,
        _sim_state: crate::prelude::SimulationState,
    ) -> Self::Output {
        P::set(&self.mirror.storage, input);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block::Block, signal::AsSignal, simulation::EndlessSimulation, tier1::sync::mirror::Mirror,
    };

    static MIRROR: Mirror<f32> = Mirror::new();

    #[test]
    fn test_usage() {
        let sim = EndlessSimulation::new(1e-3);
        let mut mirror_in = MIRROR.as_input();
        let mut mirror_out = MIRROR.as_output();

        for sim_state in sim {
            let signal = 7.0.as_signal(sim_state);
            let _ = signal * mirror_out.as_block();
            let res = sim_state * mirror_in.as_block();

            assert_eq!(res.value, 7.0);
            break;
        }
    }
}
