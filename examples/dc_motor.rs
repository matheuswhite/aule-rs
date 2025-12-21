use aule::prelude::*;
use aule::s;
use std::thread::sleep;
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

fn test_rt_dc_motor() -> RTPlotter<2, Continuous> {
    let k = 1.0;
    let a = 1.0;
    let time = Time::continuous(0.001, 10.0);

    let mut input = Sinusoid::new(1.0, Duration::from_secs_f32(1.0), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();
    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = RTPlotter::new("Real Time DC Motor".to_string());

    for dt in time {
        let signal = dt * input.as_block();
        let output = (signal - plant.last_output()) * pid.as_block() * plant.as_block();
        let output = output * writer.as_block();

        signal.zip(output).map(|(sig, o)| [sig, o]) * plotter.as_block() * IgnoreOutput;

        sleep(dt.delta.dt());
    }

    let res = plotter
        .save("output/rt_dc_motor.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter
}

fn test_dc_motor() -> Plotter<2, Continuous> {
    let k = 1.0;
    let a = 1.0;
    let time = Time::continuous(0.001, 10.0);

    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);

    let mut input = Sinusoid::new(1.0, Duration::from_secs_f32(1.0), 0.0);
    let mut pid = PID::new(10.0, 1.0, 0.1);
    let mut plant: SS<RK4> = ((k * s) / (s * s + a * k * s)).into();

    let mut writer = Writter::new("output/dc_motor.csv", ["output"]);
    let mut plotter = Plotter::new("DC Motor".to_string());

    for dt in time {
        let signal = dt * input.as_block();
        let error = signal - plant.last_output();
        let control_signal = error * pid.as_block();
        let output = control_signal * plant.as_block();

        error * iae.as_block() * ise.as_block() * itae.as_block() * IgnoreOutput;
        error.zip(control_signal) * good_hart.as_block() * IgnoreOutput;

        output * writer.as_block() * IgnoreOutput;
        signal.zip(output).map(|(sig, o)| [sig, o]) * plotter.as_block() * IgnoreOutput;
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

pub fn error_metrics<T>(
    iae: &IAE<T>,
    ise: &ISE<T>,
    itae: &ITAE<T>,
    good_hart: &GoodHart<T>,
) -> String
where
    T: TimeType,
{
    format!(
        "\n  IAE: {}\n  ISE: {}\n  ITAE: {}\n  Good Hart: {}",
        iae.value(),
        ise.value(),
        itae.value(),
        good_hart.value()
    )
}
