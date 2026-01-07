use aule::prelude::*;
use aule::s;

struct Motor {
    kv: f64,
    km: f64,
    tau_l: f64,
    last_output: Option<f64>,
    eletrical: SS<Euler, f64>,
    mechanical: SS<Euler, f64>,
}

impl Motor {
    fn new(kv: f64, km: f64, tau_l: f64, la: f64, ra: f64, jm: f64, fm: f64) -> Self {
        Motor {
            kv,
            km,
            tau_l,
            last_output: None,
            eletrical: (1.0 / (la * s + ra)).to_ss_controllable(Euler),
            mechanical: (1.0 / (jm * s + fm)).to_ss_controllable(Euler),
        }
    }
}

impl Block for Motor {
    type Input = f64;
    type Output = f64;

    fn output(&mut self, input: Signal<Self::Input>) -> Signal<Self::Output> {
        let eletrical = (input - self.last_output) * self.kv * self.eletrical.as_block();
        let mechanical = (eletrical * self.km - self.tau_l) * self.mechanical.as_block();

        self.last_output = Some(mechanical.value);

        mechanical
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.last_output
    }
}

fn main() {
    println!("Cleaning up previous output files...");
    std::fs::remove_dir_all("output").ok();
    std::fs::create_dir_all("output").ok();

    println!("Running Open Loop Motor Simulation...");
    let plotter1 = open_loop_motor();
    println!("Running Closed Loop Motor Simulation...");
    let plotter2 = closed_loop_motor();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");

    (plotter1, plotter2).join_all();
}

fn open_loop_motor() -> Plotter<1, f64> {
    let time = Time::new(1e-3, 1.0);
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 1.0, 0.01, 0.01);
    let mut step = Step::default();
    let mut writer = Writter::new("output/open_loop_motor.csv", ["output"]);
    let mut plotter = Plotter::new("Open loop Motor".to_string());

    for dt in time {
        let input = dt * step.as_block();
        let output = input * motor.as_block() * writer.as_block();

        let _ = output * plotter.as_block();
    }

    plotter.display();
    let res = plotter
        .save("output/open_loop_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}

fn closed_loop_motor() -> Plotter<1, f64> {
    let time = Time::new(1e-3, 1.0);
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 1.0, 0.01, 0.01);
    let mut step = Step::default();
    let mut pid = PID::new(10.0, 0.1, 0.01);
    let mut writer = Writter::new("output/closed_loop_motor.csv", ["output"]);
    let mut plotter = Plotter::new("Closed Loop Motor".to_string());

    for dt in time {
        let input = dt * step.as_block();
        let error = input - motor.last_output();
        let control_signal = error * pid.as_block();
        let output = control_signal * motor.as_block() * writer.as_block();

        let _ = output * plotter.as_block();
    }

    plotter.display();
    let res = plotter
        .save("output/closed_loop_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}
