use aule::prelude::*;

fn main() {
    let time = Time::from((0.001, 10.0));

    let mut step = Step::new();
    let mut pid = PID::new(25.0, 0.0, 0.00);
    let mut plant: SS<RK4> = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).into();
    let mut writer = Writter::new("output/third_order_system.csv", ["output"]);
    let mut iae = IAE::new();
    let mut ise = ISE::new();
    let mut itae = ITAE::new();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
    let mut plotter = Plotter::new();

    for dt in time {
        let input = dt >> step.as_input();
        let error = (input - plant.last_output())
            >> iae.as_error_metric()
            >> ise.as_error_metric()
            >> itae.as_error_metric();
        let control_signal = error * pid.as_siso();
        good_hart.update([error, control_signal]);
        let output = control_signal * plant.as_siso() >> writer.as_monitor();

        let _ = output >> plotter.as_monitor();
    }

    println!("IAE Value: {}", iae.value());
    println!("ISE Value: {}", ise.value());
    println!("ITAE Value: {}", itae.value());
    println!("GoodHart Value: {}", good_hart.value());

    plotter.display();

    plotter.join();
}
