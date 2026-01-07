use std::marker::PhantomData;

use aule::prelude::*;
use aule::tier3::mpc::{CostFunction, MPC, Optimizer};
use ndarray::arr2;

fn main() {
    let time = Time::new(1e-2, 3.0);
    let mut reference = Step::default();
    let mut plant = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).to_ss_controllable(Euler);

    let mut controller = MPC::new(
        SimulatingAnnealing::<10>::new(45.0),
        Observer::new(
            arr2(&[[0.0, 1.0], [-6.0, -11.0]]),
            [0.0, 1.0],
            [1.0, 0.0],
            0.0,
            [0.0, 0.0],
        ),
        IAECost::default(),
    );

    let mut plotter = Plotter::new("MPC Example".to_string());

    for dt in time {
        let ref_signal = dt * reference.as_block();
        let last_output = plant.last_output();

        let controller_input = (ref_signal, dt.map(|_| last_output.unwrap_or(0.0))).pack();
        let control_signal = controller_input * controller.as_block();
        let output = control_signal * plant.as_block();

        let mpc_model_input = (control_signal, output).pack();
        controller.model_mut().output(mpc_model_input);

        plotter.output([ref_signal, output].pack());
    }

    plotter.display();
    plotter.join();
}

struct SimulatingAnnealing<const N: usize> {
    control_sequence: [f64; N],
    control_limit: f64,
}

impl<const N: usize> SimulatingAnnealing<N> {
    fn random_float(min: f64, max: f64) -> f64 {
        rand::random::<f64>() * (max - min) + min
    }

    fn new(control_limit: f64) -> Self {
        SimulatingAnnealing {
            control_sequence: [0.0; N].map(|_| Self::random_float(0.0, control_limit)),
            control_limit,
        }
    }

    fn gen_new_control_sequence(
        &self,
        control_sequence: Signal<[f64; N]>,
        temperature: f64,
    ) -> Signal<[f64; N]> {
        let mut new_sequence = control_sequence;
        let index = rand::random::<u32>() as usize % N;
        let signal = &mut new_sequence.value[index];
        *signal += Self::random_float(-temperature / 100.0, temperature / 100.0);
        *signal = signal.clamp(-self.control_limit, self.control_limit);

        new_sequence
    }
}

impl<const N: usize> Optimizer<IAECost<N>, Observer<Euler, 2, f64>> for SimulatingAnnealing<N> {
    type Reference = f64;
    type MeasuredOutput = f64;
    type ManipulatedVariable = [f64; N];

    fn solve(
        &mut self,
        reference: Signal<f64>,
        measured_output: Signal<f64>,
        model: &mut Observer<Euler, 2, f64>,
    ) -> Signal<[f64; N]> {
        let mut control_sequence = reference.map(|_| self.control_sequence);
        let mut temperature = 1000.0;
        let temperature_final = 1.0;
        let cooling_rate = 0.95;

        while temperature > temperature_final {
            let new_sequence = self.gen_new_control_sequence(control_sequence, temperature);
            let new_energy = IAECost::cost(model, reference, measured_output, new_sequence);
            let old_energy = IAECost::cost(model, reference, measured_output, control_sequence);

            let delta_energy = new_energy - old_energy;

            if delta_energy < 0.0 || rand::random::<f64>() < (-delta_energy / temperature).exp() {
                control_sequence = new_sequence;
            }

            temperature *= cooling_rate;
        }

        control_sequence
    }
}

#[derive(Default)]
struct IAECost<const HORIZON: usize> {
    _marker: PhantomData<[(); HORIZON]>,
}

impl<const HORIZON: usize> CostFunction for IAECost<HORIZON> {
    type Model = Observer<Euler, 2, f64>;
    type Reference = f64;
    type MeasuredOutput = f64;
    type ManipulatedVariable = [f64; HORIZON];

    fn cost(
        model: &mut Observer<Euler, 2, f64>,
        reference: Signal<f64>,
        measured_output: Signal<f64>,
        manipulated_variable: Signal<[f64; HORIZON]>,
    ) -> f64 {
        model.save_state();
        let mut iae = IAE::default();

        for i in 0..HORIZON {
            let control_input = manipulated_variable.map(|u| u[i]);
            let estimator_input = (control_input, measured_output).pack();
            let estimated_output = model.output(estimator_input).unpack().0;
            let error = reference - estimated_output;
            iae.output(error);
        }

        model.restore_state();
        iae.value()
    }
}
