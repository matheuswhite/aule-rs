use aule::prelude::*;
use aule::s;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Cleaning up previous output files...");
    std::fs::remove_dir_all("output").ok();
    std::fs::create_dir_all("output").ok();

    println!("Running RT DC Motor Simulation...");
    let rt_plotter = test_rt_dc_motor();
    println!("Running DC Motor Simulation...");
    let plotter = test_dc_motor();

    println!("All simulations completed successfully!");
    println!("Check the 'output' directory for results.");

    (rt_plotter, plotter).join_all();
}

fn test_rt_dc_motor() -> RTPlotter<2, f64> {
    let k = 1.0f64;
    let a = 1.0f64;
    let time = Time::new(0.001, 10.0);

    let mut input = Sinusoid::new(1.0f64, Duration::from_secs_f32(1.0), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant = ((k * s) / (s * s + a * k * s)).to_ss_controllable(RK4);
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = RTPlotter::new("Real Time DC Motor".to_string());

    for dt in time {
        let signal = dt * input.as_block();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block();
        let output = output * writer.as_block();

        let _ = [signal, output].pack() * plotter.as_block();

        sleep(dt.delta.dt());
    }

    let res = plotter
        .save("output/rt_dc_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}

fn test_dc_motor() -> Plotter<2, f64> {
    let k = 1.0f64;
    let a = 1.0f64;
    let time = Time::new(0.001, 10.0);

    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);

    let mut input = Sinusoid::new(1.0, Duration::from_secs_f32(1.0), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant = ((k * s) / (s * s + a * k * s)).to_ss_controllable(RK4);

    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = Plotter::new("DC Motor".to_string());

    for dt in time {
        let signal = dt * input.as_block();
        let error = signal - plant.last_output();
        let control_signal = error * pid.as_block();
        let output = control_signal * plant.as_block();

        let _ = error * iae.as_block() * ise.as_block() * itae.as_block();
        let _ = (error, control_signal).pack() * good_hart.as_block();

        let _ = output * writer.as_block();
        let _ = [signal, output].pack() * plotter.as_block();
    }

    println!(
        "PID error metrics: {}",
        error_metrics(&iae, &ise, &itae, &good_hart)
    );
    plotter.display();
    let res = plotter
        .save("output/dc_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}

pub fn error_metrics(
    iae: &IAE<f64>,
    ise: &ISE<f64>,
    itae: &ITAE<f64>,
    good_hart: &GoodHart<f64>,
) -> String {
    format!(
        "\n  IAE: {}\n  ISE: {}\n  ITAE: {}\n  Good Hart: {}",
        iae.value(),
        ise.value(),
        itae.value(),
        good_hart.value()
    )
}
