use aule::prelude::*;
use aule::s;

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

fn main() {
    println!("Cleaning up previous output files...");
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir_all("output");

    let plotter_ctx = PlotterContext::new();

    println!("Running Open Loop Motor Simulation...");
    open_loop_motor();
    println!("Running Closed Loop Motor Simulation...");
    closed_loop_motor();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");

    keep_alive(plotter_ctx);
}

fn open_loop_motor() {
    let time = Time::from((0.001, 0.2));
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 1.0, 0.01, 0.01);
    let mut step = Step::new();
    let mut writer = Writter::new("output/open_loop_motor.csv", ["output"]);
    let mut chart = Chart::new("output/open_loop_motor.svg");

    for dt in time {
        let input = dt >> step.as_input();
        let output = input * motor.as_block() >> writer.as_monitor();

        let _ = (input, output) >> chart.as_monitor();
    }

    chart.plot();
}

fn closed_loop_motor() {
    let time = Time::from((0.001, 0.2));
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 1.0, 0.01, 0.01);
    let mut step = Step::new();
    let mut pid = PID::new(10.0, 0.1, 0.01);
    let mut writer = Writter::new("output/closed_loop_motor.csv", ["output"]);
    let mut chart = Chart::new("output/closed_loop_motor.svg");

    for dt in time {
        let input = dt >> step.as_input();
        let error = input - motor.last_output().unwrap_or_default();
        let control_signal = error * pid.as_block();
        let output = control_signal * motor.as_block() >> writer.as_monitor();

        let _ = (input, output) >> chart.as_monitor();
    }

    chart.plot();
}
