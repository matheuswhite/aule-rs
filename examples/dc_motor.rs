use aule::prelude::*;
use aule::s;
use std::f32::consts::PI;
use std::rc::Rc;
use std::sync::Mutex;

fn main() {
    println!("Cleaning up previous output files...");
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir_all("output");

    let plotter_ctx = PlotterContext::new();

    println!("Running RT DC Motor Simulation...");
    test_rt_dc_motor(&plotter_ctx);
    println!("Running DC Motor Simulation...");
    test_dc_motor(&plotter_ctx);

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");

    keep_alive(plotter_ctx);
}

fn test_rt_dc_motor(plotter_ctx: &Rc<Mutex<PlotterContext>>) {
    let k = 1.0;
    let a = 1.0;
    let time = RTTime::from((0.001, 10.0));

    let mut input = Sinusoid::new(1.0, 1.0 / (2.0 * PI), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut chart = Chart::new("output/dc_motor.svg");
    let mut plotter = RTPlotter::new("DC Motor Output", (0.0, 10.0), (-1.0, 1.0), &plotter_ctx);

    for dt in time {
        let signal = dt >> input.as_input();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block()
            >> writer.as_monitor();

        let _ = (signal, output) >> chart.as_monitor();
        let _ = output >> plotter.as_monitor();
    }

    chart.plot();
    println!("Running plotter...");
}

fn test_dc_motor(plotter_ctx: &Rc<Mutex<PlotterContext>>) {
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
    let mut chart = Chart::new("output/dc_motor.svg");
    let mut plotter = Plotter::new("DC Motor Output", (0.0, 10.0), (-1.0, 1.0), &plotter_ctx);

    for dt in time {
        let signal = dt >> input.as_input();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block()
            >> writer.as_monitor();

        let _ = (signal, output) >> chart.as_monitor();
        let _ = output >> plotter.as_monitor();
    }

    chart.plot();
    println!("PID error metrics: {}", pid.error_metrics());
    plotter.display();
}
