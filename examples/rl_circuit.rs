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
            integrator: (1.0 / (l * s + r)).into(),
        }
    }
}

impl Block for RlCircuit {
    type Input = f64;
    type Output = f64;
    type TimeType = Continuous;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let output = input * self.integrator.as_block();

        self.last_output = Some(output.value);

        output
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }
}

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
    let time = Time::continuous(0.001, 0.2);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::default();
    let mut writer = Writter::new("output/open_loop_rl_circuit.csv", ["output"]);

    for dt in time {
        let input = dt * step.as_block();
        input * rl_circuit.as_block() * writer.as_block() * IgnoreOutput;
    }
}

fn closed_loop_rl_circuit() {
    let time = Time::continuous(0.001, 0.2);

    let mut pid = PID::new(1.0, 0.0, 0.00);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::default();
    let mut writer = Writter::new("output/closed_loop_rl_circuit.csv", ["output"]);

    for dt in time {
        let input = dt * step.as_block();
        (input - rl_circuit.last_output())
            * pid.as_block()
            * rl_circuit.as_block()
            * writer.as_block()
            * IgnoreOutput;
    }
}
