use aule::{chart::Chart, prelude::*, writer::Writter};
use std::{rc::Rc, time::Duration};

struct Motor {
    kv: Gain,
    km: Gain,
    tau_l: f32,
    last_output: Option<Signal>,
    eletrical: Euler,
    mechanical: Euler,
}

impl Motor {
    fn new(kv: f32, km: f32, tau_l: f32, la: f32, ra: f32, jm: f32, fm: f32) -> Self {
        Motor {
            kv: Gain::new(kv),
            km: Gain::new(km),
            tau_l,
            last_output: None,
            eletrical: (1.0 / (la * s + ra)).discretize(),
            mechanical: (1.0 / (jm * s + fm)).discretize(),
        }
    }
}

impl Block for Motor {
    fn output(&mut self, input: Signal) -> Signal {
        let eletrical = (input - self.last_output) * self.kv.as_block() * self.eletrical.as_block();
        let mechanical = (eletrical * self.km.as_block() - self.tau_l) * self.mechanical.as_block();

        self.last_output = Some(mechanical);

        mechanical
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for Motor {}

pub struct RlCircuit {
    r: f32,
    l: f32,
    last_output: Option<Signal>,
    integrator: Euler,
}

impl RlCircuit {
    pub fn new(r: f32, l: f32) -> Self {
        RlCircuit {
            r,
            l,
            last_output: None,
            integrator: (1.0 / (l * s + r)).discretize(),
        }
    }
}

impl Block for RlCircuit {
    fn output(&mut self, input: Signal) -> Signal {
        let output = input * self.integrator.as_block();

        self.last_output = Some(output);

        output
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for RlCircuit {}

fn main() {
    println!("Cleaning up previous output files...");
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir_all("output");

    println!("Running Open Loop RL Circuit Simulation...");
    open_loop_rl_circuit();
    println!("Running Closed Loop RL Circuit Simulation...");
    closed_loop_rl_circuit();
    println!("Running Third Order System Simulation...");
    test_third_order_system();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");
}

fn open_loop_rl_circuit() {
    let dt = Duration::from_secs_f32(0.01);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let step = Step::new(dt).with_max_time(Duration::from_secs(10));
    let mut writer = Writter::new("output/open_loop_rl_circuit.csv", "output");
    let mut chart = Chart::new("output/open_loop_rl_circuit.svg");

    for input in step {
        let _ = input * rl_circuit.as_block() >> writer.as_monitor() >> chart.as_monitor();
    }

    chart.plot();
}

fn closed_loop_rl_circuit() {
    let dt = Duration::from_secs_f32(0.01);
    let mut pid = PID::new(1.0, 0.0, 0.00);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let step = Step::new(dt).with_max_time(Duration::from_secs(10));
    let mut writer = Writter::new("output/closed_loop_rl_circuit.csv", "output");
    let mut chart = Chart::new("output/closed_loop_rl_circuit.svg");

    for input in step {
        let _ = (input - rl_circuit.last_output()) * pid.as_block() * rl_circuit.as_block()
            >> writer.as_monitor()
            >> chart.as_monitor();
    }

    chart.plot();
}

fn test_third_order_system() {
    let dt = Duration::from_secs_f32(0.01);
    let step = Step::new(dt).with_max_time(Duration::from_secs(20));
    let mut pid = PID::new(25.0, 0.0, 0.00);
    let mut plant = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).discretize();
    let mut writer = Writter::new("output/third_order_system.csv", "output");
    let mut chart = Chart::new("output/third_order_system.svg");

    for input in step {
        let _ = (input - plant.last_output()) * pid.as_block() * plant.as_block()
            >> writer.as_monitor()
            >> chart.as_monitor();
    }

    chart.plot();
}
