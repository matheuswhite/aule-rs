use aule::prelude::*;
use aule::s;
use std::f32::consts::PI;

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

fn test_rt_dc_motor() -> RTPlotter {
    let k = 1.0;
    let a = 1.0;
    let time = RTTime::from((0.001, 10.0));

    let mut input = Sinusoid::new(1.0, 1.0 / (2.0 * PI), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = RTPlotter::new(1.0, 1.0);

    for dt in time {
        let signal = dt >> input.as_input();
        let output =
            (signal - plant.last_output()) * pid.as_siso() * plant.as_siso() >> writer.as_monitor();

        let _ = (signal, output) >> plotter.as_monitor();
    }

    plotter
}

fn test_dc_motor() -> Plotter {
    let k = 1.0;
    let a = 1.0;
    let time = Time::from((0.001, 10.0));

    let mut input = Sinusoid::new(1.0, 1.0 / (2.0 * PI), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1)
        .with_iae()
        .with_ise()
        .with_itae()
        .with_good_hart(0.3, 0.3, 0.4);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = Plotter::new(1.0, 1.0);

    for dt in time {
        let signal = dt >> input.as_input();
        let output =
            (signal - plant.last_output()) * pid.as_siso() * plant.as_siso() >> writer.as_monitor();

        let _ = (signal, output) >> plotter.as_monitor();
    }

    println!("PID error metrics: {}", pid.error_metrics());
    plotter.display();

    plotter
}
