use aule::prelude::*;
use aule::s;

pub struct RlCircuit {
    last_output: Option<Signal>,
    integrator: SS<RK4>,
}

impl RlCircuit {
    pub fn new(r: f32, l: f32) -> Self {
        RlCircuit {
            last_output: None,
            integrator: (1.0 / (l * s + r)).into(),
        }
    }
}

impl SISO for RlCircuit {
    fn output(&mut self, input: Signal) -> Signal {
        let output = input * self.integrator.as_siso();

        self.last_output = Some(output);

        output
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsSISO for RlCircuit {}

fn main() {
    println!("Cleaning up previous output files...");
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir_all("output");

    println!("Running Open Loop RL Circuit Simulation...");
    open_loop_rl_circuit();
    println!("Running Closed Loop RL Circuit Simulation...");
    closed_loop_rl_circuit();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");
}

fn open_loop_rl_circuit() {
    let time = Time::from((0.001, 0.2));
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::default();
    let mut writer = Writter::new("output/open_loop_rl_circuit.csv", ["output"]);

    for dt in time {
        let input = dt >> step.as_input();
        let _ = input * rl_circuit.as_siso() >> writer.as_output();
    }
}

fn closed_loop_rl_circuit() {
    let time = Time::from((0.001, 0.2));

    let mut pid = PID::new(1.0, 0.0, 0.00);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::default();
    let mut writer = Writter::new("output/closed_loop_rl_circuit.csv", ["output"]);

    for dt in time {
        let input = dt >> step.as_input();
        let _ = (input - rl_circuit.last_output()) * pid.as_siso() * rl_circuit.as_siso()
            >> writer.as_output();
    }
}
