use crate::{block::Block, signal::Signal, signal::Unpack};
use core::marker::PhantomData;

pub struct MPC<O, M, CF, Ref, MO, MV, const N: usize>
where
    CF: CostFunction<Model = M, Reference = Ref, MeasuredOutput = MO, ManipulatedVariable = [MV; N]>,
    O: Optimizer<CF, M, Reference = Ref, MeasuredOutput = MO, ManipulatedVariable = [MV; N]>,
    Ref: Copy,
    MO: Copy,
    MV: Copy,
{
    optimizer: O,
    model: M,
    _marker: PhantomData<CF>,
}

pub trait Optimizer<CF, M>
where
    CF: CostFunction<
            Model = M,
            Reference = Self::Reference,
            MeasuredOutput = Self::MeasuredOutput,
            ManipulatedVariable = Self::ManipulatedVariable,
        >,
{
    type Reference: Copy;
    type MeasuredOutput: Copy;
    type ManipulatedVariable: Copy;

    fn solve(
        &mut self,
        reference: Signal<Self::Reference>,
        measured_output: Signal<Self::MeasuredOutput>,
        model: &mut M,
    ) -> Signal<Self::ManipulatedVariable>;
}

pub trait CostFunction {
    type Model;
    type Reference;
    type MeasuredOutput;
    type ManipulatedVariable;

    fn cost(
        model: &mut Self::Model,
        reference: Signal<Self::Reference>,
        measured_output: Signal<Self::MeasuredOutput>,
        manipulated_variable: Signal<Self::ManipulatedVariable>,
    ) -> f64;
}

impl<O, M, CF, Ref, MO, MV, const N: usize> MPC<O, M, CF, Ref, MO, MV, N>
where
    O: Optimizer<CF, M, Reference = Ref, MeasuredOutput = MO, ManipulatedVariable = [MV; N]>,
    CF: CostFunction<Model = M, Reference = Ref, MeasuredOutput = MO, ManipulatedVariable = [MV; N]>,
    Ref: Copy,
    MO: Copy,
    MV: Copy,
{
    pub fn new(optimizer: O, model: M, _cost_function: CF) -> Self {
        MPC {
            optimizer,
            model,
            _marker: PhantomData,
        }
    }

    pub fn model_mut(&mut self) -> &mut M {
        &mut self.model
    }
}

impl<O, M, CF, Ref, MO, MV, const N: usize> Block for MPC<O, M, CF, Ref, MO, MV, N>
where
    O: Optimizer<CF, M, Reference = Ref, MeasuredOutput = MO, ManipulatedVariable = [MV; N]>,
    CF: CostFunction<Model = M, Reference = Ref, MeasuredOutput = MO, ManipulatedVariable = [MV; N]>,
    Ref: Copy,
    MO: Copy,
    MV: Copy,
{
    type Input = (Ref, MO);
    type Output = MV;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let (reference, measured_output) = input.unpack();

        let control_sequence = self
            .optimizer
            .solve(reference, measured_output, &mut self.model);

        control_sequence.map(|u| u[0])
    }
}
