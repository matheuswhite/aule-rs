use aule::prelude::*;
use aule::s;

struct Motor {
    kv: f32,
    km: f32,
    tau_l: f32,
    last_output: Option<Signal>,
    eletrical: SS<Euler>,
    mechanical: SS<Euler>,
}

impl Motor {
    fn new(kv: f32, km: f32, tau_l: f32, la: f32, ra: f32, jm: f32, fm: f32) -> Self {
        Motor {
            kv,
            km,
            tau_l,
            last_output: None,
            eletrical: (1.0 / (la * s + ra)).into(),
            mechanical: (1.0 / (jm * s + fm)).into(),
        }
    }
}

impl SISO for Motor {
    fn output(&mut self, input: Signal) -> Signal {
        let eletrical = (input - self.last_output) * self.kv * self.eletrical.as_siso();
        let mechanical = (eletrical * self.km - self.tau_l) * self.mechanical.as_siso();

        self.last_output = Some(mechanical);

        mechanical
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsSISO for Motor {}

fn main() {
    println!("Cleaning up previous output files...");
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir_all("output");

    println!("Running Open Loop Motor Simulation...");
    let plotter1 = open_loop_motor();
    println!("Running Closed Loop Motor Simulation...");
    let plotter2 = closed_loop_motor();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");

    (plotter1, plotter2).join_all();
}

fn open_loop_motor() -> Plotter {
    let time = Time::from((0.001, 0.2));
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 1.0, 0.01, 0.01);
    let mut step = Step::default();
    let mut writer = Writter::new("output/open_loop_motor.csv", ["output"]);
    let mut plotter = Plotter::new("Open loop Motor".to_string(), 0.05, 0.5);

    for dt in time {
        let input = dt >> step.as_input();
        let output = input * motor.as_siso() >> writer.as_output();

        let _ = output >> plotter.as_output();
    }

    plotter.display();
    let res = plotter
        .save("output/open_loop_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}

fn closed_loop_motor() -> Plotter {
    let time = Time::from((0.001, 0.2));
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 1.0, 0.01, 0.01);
    let mut step = Step::default();
    let mut pid = PID::new(10.0, 0.1, 0.01);
    let mut writer = Writter::new("output/closed_loop_motor.csv", ["output"]);
    let mut plotter = Plotter::new("Closed Loop Motor".to_string(), 0.05, 0.5);

    for dt in time {
        let input = dt >> step.as_input();
        let error = input - motor.last_output().unwrap_or_default();
        let control_signal = error * pid.as_siso();
        let output = control_signal * motor.as_siso() >> writer.as_output();

        let _ = output >> plotter.as_output();
    }

    plotter.display();
    let res = plotter
        .save("output/closed_loop_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}
