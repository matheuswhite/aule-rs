use aule::prelude::*;

fn main() {
    let mut swd_conn = SwdConnection::new("nRF52833_xxAA", 0, 0x2000_0000, 128 * 1024);
    let time = Time::continuous(1e-3, 10.0);

    let mut step = Step::default();
    let mut remote_pid = swd_conn.new_remote_block("pid1").unwrap();
    let mut plant: SS<RK4> = Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).into();
    let mut iae = IAE::default();
    let mut ise = ISE::default();
    let mut itae = ITAE::default();
    let mut good_hart = GoodHart::new(0.3, 0.3, 0.4);
    let mut plotter = Plotter::new("Third Order System".to_string());

    for dt in time {
        let input = dt * step.as_block();
        let error =
            (input - plant.last_output()) * iae.as_block() * ise.as_block() * itae.as_block();

        let control_signal = remote_pid.output(error);
        let output = control_signal * plant.as_block();

        let _ = error.map(|e| (e, control_signal.value)) * good_hart.as_block();
        let _ = input.map(|i| [i, output.value]) * plotter.as_block();
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
