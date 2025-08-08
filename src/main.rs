use aule::prelude::*;
use std::{f32::consts::PI, thread::sleep, time::Duration};

struct Motor {
    kv: Gain,
    km: Gain,
    tau_l: f32,
    last_output: Option<Signal>,
    eletrical: SS<Euler>,
    mechanical: SS<Euler>,
}

impl Motor {
    fn new(kv: f32, km: f32, tau_l: f32, la: f32, ra: f32, jm: f32, fm: f32) -> Self {
        Motor {
            kv: Gain::new(kv),
            km: Gain::new(km),
            tau_l,
            last_output: None,
            eletrical: (1.0 / (la * s + ra)).into(),
            mechanical: (1.0 / (jm * s + fm)).into(),
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

type I = RK4;
const TIME_STEP: f32 = 0.001;

pub struct RlCircuit {
    r: f32,
    l: f32,
    last_output: Option<Signal>,
    integrator: SS<I>,
}

impl RlCircuit {
    pub fn new(r: f32, l: f32) -> Self {
        RlCircuit {
            r,
            l,
            last_output: None,
            integrator: (1.0 / (l * s + r)).into(),
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
    println!("Running DC Motor Simulation...");
    test_dc_motor();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");
}

fn open_loop_rl_circuit() {
    let time = Time::from((TIME_STEP, 0.2));
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::new();
    let mut writer = Writter::new("output/open_loop_rl_circuit.csv", ["output"]);
    let mut chart = Chart::new("output/open_loop_rl_circuit.svg");

    for dt in time {
        let input = dt >> step.as_input();
        let _ = input * rl_circuit.as_block() >> writer.as_monitor() >> chart.as_monitor();
    }

    chart.plot();
}

fn closed_loop_rl_circuit() {
    let time = Time::from((TIME_STEP, 0.2));

    let mut pid = PID::new(1.0, 0.0, 0.00);
    let mut rl_circuit = RlCircuit::new(5.0, 0.05);
    let mut step = Step::new();
    let mut writer = Writter::new("output/closed_loop_rl_circuit.csv", ["output"]);
    let mut chart = Chart::new("output/closed_loop_rl_circuit.svg");

    for dt in time {
        let input = dt >> step.as_input();
        let _ = (input - rl_circuit.last_output()) * pid.as_block() * rl_circuit.as_block()
            >> writer.as_monitor()
            >> chart.as_monitor();
    }

    chart.plot();
}

fn test_third_order_system() {
    let time = Time::from((TIME_STEP, 10.0));

    let mut step = Step::new();
    let mut pid = PID::new(25.0, 0.0, 0.00);
    let mut plant: SS<I> = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).into();
    let mut writer = Writter::new("output/third_order_system.csv", ["output"]);
    let mut chart = Chart::new("output/third_order_system.svg");

    for dt in time {
        let input = dt >> step.as_input();
        let output = (input - plant.last_output()) * pid.as_block() * plant.as_block()
            >> writer.as_monitor();

        let _ = (input, output) >> chart.as_monitor();
    }

    chart.plot();
}

fn test_dc_motor() {
    let plotter_ctx = PlotterContext::new();

    let k = 1.0;
    let a = 1.0;
    let time = Time::from((TIME_STEP, 10.0));

    let mut input = Sinusoid::new(1.0, 1.0 / (2.0 * PI), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut chart = Chart::new("output/dc_motor.svg");
    let mut plotter = Plotter::new("DC Motor Output", (0.0, 10.0), (-1.0, 1.0), &plotter_ctx);

    for dt in time {
        let signal = dt >> input.as_input();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block()
            >> writer.as_monitor();

        let _ = (signal, output) >> chart.as_monitor();
        let _ = output >> plotter.as_monitor();

        sleep(Duration::from_secs_f32(TIME_STEP));
    }

    chart.plot();
    println!("Running plotter...");

    // plotter.display();

    keep_alive(plotter_ctx);
}
