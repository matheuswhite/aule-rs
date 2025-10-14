use aule::prelude::*;

fn main() {
    let time = Time::from((1e-3, 10.0));

    let mut step = Step::default();
    let mut pid = PID::new(40.0, 10.0, 10.00);
    let mut plant: SS<RK4> = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).into();
    let mut writer = Writter::new("output/third_order_system.csv", ["output"]);
    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
    let mut plotter = Plotter::new("Third Order System".to_string(), 1.0, 0.25);

    for dt in time {
        let input = dt * step.as_mut();
        let error = (input - plant.last_output()) * iae.as_mut() * ise.as_mut() * itae.as_mut();
        let control_signal = error * pid.as_mut();
        let _ = merge!(error, control_signal) * good_hart.as_mut();
        let output = control_signal * plant.as_mut();

        let _ = merge!(input, output) * plotter.as_mut() * writer.as_mut();
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
