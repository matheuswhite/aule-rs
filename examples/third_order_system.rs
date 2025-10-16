use aule::prelude::*;

fn main() {
    let time = Time::continuous(1e-3, 10.0);

    let mut step = Step::default();
    let mut pid = PID::new(40.0, 10.0, 10.00);
    let mut plant: SS<RK4> = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).into();
    let mut writer = Writter::new("output/third_order_system.csv", ["input", "output"]);
    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
    let mut plotter = Plotter::new("Third Order System".to_string(), 1.0, 0.25);

    for dt in time {
        let input = dt * step.as_block();
        let error =
            (input - plant.last_output()) * iae.as_block() * ise.as_block() * itae.as_block();
        let control_signal = error * pid.as_block();
        let output = control_signal * plant.as_block();

        let _ = error.zip(control_signal) * good_hart.as_block();
        let _ = input.zip(output).map(|(i, o)| [i, o]) * plotter.as_block() * writer.as_block();
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
