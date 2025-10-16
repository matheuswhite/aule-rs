use aule::prelude::*;
use aule::s;
use std::time::Duration;

fn main() {
    println!("Cleaning up previous output files...");
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir_all("output");

    println!("Running RT DC Motor Simulation...");
    let rt_plotter = test_rt_dc_motor();
    println!("Running DC Motor Simulation...");
    let plotter = test_dc_motor();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");

    (rt_plotter, plotter).join_all();
}

fn test_rt_dc_motor() -> RTPlotter<2> {
    let k = 1.0;
    let a = 1.0;
    let time = Time::continuous(0.001, 10.0);

    let mut input = Sinusoid::new(1.0, Duration::from_secs_f32(1.0), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = RTPlotter::new("Real Time DC Motor".to_string(), 1.0, 1.0);

    for dt in time {
        let signal = dt * input.as_block();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block();
        let output = output * writer.as_block();

        let _ = signal.zip(output).map(|(sig, o)| [sig, o]) * plotter.as_block();
    }

    let res = plotter
        .save("output/rt_dc_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}

fn test_dc_motor() -> Plotter<2> {
    let k = 1.0;
    let a = 1.0;
    let time = Time::continuous(0.001, 10.0);

    let mut input = Sinusoid::new(1.0, Duration::from_secs_f32(1.0), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1)
        .with_iae()
        .with_ise()
        .with_itae()
        .with_good_hart(0.3, 0.3, 0.4);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = Plotter::new("DC Motor".to_string(), 1.0, 1.0);

    for dt in time {
        let signal = dt * input.as_block();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block();
        let output = output * writer.as_block();

        let _ = signal.zip(output).map(|(sig, o)| [sig, o]) * plotter.as_block();
    }

    println!("PID error metrics: {}", pid.error_metrics());
    plotter.display();
    let res = plotter
        .save("output/dc_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}
