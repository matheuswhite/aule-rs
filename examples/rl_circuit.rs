use aule::prelude::*;
use aule::s;

pub struct RlCircuit {
    last_output: Option<f64>,
    integrator: SS<RK4, f64>,
}

impl RlCircuit {
    pub fn new(r: f64, l: f64) -> Self {
        RlCircuit {
            last_output: None,
            integrator: (1.0 / (l * s + r)).to_ss_controllable(RK4),
        }
    }
}

impl Block for RlCircuit {
    type Input = f64;
    type Output = f64;

    fn block(&mut self, input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let output = self.integrator.block(input, sim_state);

        self.last_output = Some(output);

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }
}

fn main() {
    println!("Cleaning up previous output files...");
    std::fs::remove_dir_all("output").ok();
    std::fs::create_dir_all("output").ok();

    println!("Running Open Loop RL Circuit Simulation...");
    open_loop_rl_circuit();
    println!("Running Closed Loop RL Circuit Simulation...");
    closed_loop_rl_circuit();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");
}

fn open_loop_rl_circuit() {
    let simulation = Simulation::new(0.001, 0.2);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::default();
    let mut writer = Writter::new("output/open_loop_rl_circuit.csv", ["output"]);

    for sim_state in simulation {
        let input = sim_state * step.as_block();
        let _ = input * rl_circuit.as_block() * writer.as_block();
    }
}

fn closed_loop_rl_circuit() {
    let simulation = Simulation::new(0.001, 0.2);

    let mut pid = PID::new(1.0, 0.0, 0.00);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::default();
    let mut writer = Writter::new("output/closed_loop_rl_circuit.csv", ["output"]);

    for sim_state in simulation {
        let input = sim_state * step.as_block();
        let _ = (input - rl_circuit.last_output())
            * pid.as_block()
            * rl_circuit.as_block()
            * writer.as_block();
    }
}
