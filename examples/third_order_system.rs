use aule::prelude::*;

fn main() {
    let time = Time::new(1e-3, 10.0);

    let mut step = Step::default();
    let mut pid = PID::new(40.0, 10.0, 10.00);
    let mut saturation = Saturation::new(0.0, 15.0);
    let mut plant = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).to_ss_controllable(RK4);
    let mut writer = Writter::new("output/third_order_system.csv", ["input", "output"]);
    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
    let mut plotter: Plotter<_, f64> = Plotter::new("Third Order System".to_string());

    for dt in time {
        let input = dt * step.as_block();
        let error = input - plant.last_output();
        iae.output(error);
        ise.output(error);
        itae.output(error);

        let control_signal = error * pid.as_block();
        let control_signal = control_signal * saturation.as_block();
        let output = control_signal * plant.as_block();

        let _ = (error, control_signal).pack() * good_hart.as_block();
        let _ = [input, output].pack() * plotter.as_block() * writer.as_block();
    }

    println!("IAE Value: {}", iae.value());
    println!("ISE Value: {}", ise.value());
    println!("ITAE Value: {}", itae.value());
    println!("GoodHart Value: {}", good_hart.value());

    plotter.display();
    let res = plotter
        .save("output/third_order_system.png")
        .expect("Failed to save plot");
    print!("{}", res);

    plotter.join();
}
