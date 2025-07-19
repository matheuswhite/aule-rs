use aule::prelude::*;
use std::time::Duration;

struct Motor {
    kv: Gain,
    km: Gain,
    tau_l: f32,
    last_output: Option<Signal>,
    eletrical: StateSpace,
    mechanical: StateSpace,
}

impl Motor {
    fn new(kv: f32, km: f32, tau_l: f32, la: f32, ra: f32, jm: f32, fm: f32) -> Self {
        Motor {
            kv: Gain::new(kv),
            km: Gain::new(km),
            tau_l,
            last_output: None,
            eletrical: (1.0 / (la * s + ra)).as_ss(),
            mechanical: (1.0 / (jm * s + fm)).as_ss(),
        }
    }
}

impl Block for Motor {
    fn output(&mut self, input: Signal) -> Signal {
        let eletrical = (input - self.last_output) * self.kv.as_block() * self.eletrical.as_block();
        let mechanical = (eletrical * self.km.as_block() - self.tau_l) * self.mechanical.as_block();

        mechanical
    }

    fn last_output(&self) -> Option<Signal> {
        self.last_output
    }
}

impl AsBlock for Motor {}

fn main() {
    let plotter_context = PlotterContext::new();

    let dt = Duration::from_secs_f32(0.1);

    let mut pid = PID::new(1.0, 0.1, 0.01);
    let mut motor = Motor::new(1.0, 1.0, 0.1, 0.01, 0.1, 0.1, 0.01);
    let step = Step::new(dt).with_max_time(Duration::from_secs(10));
    let mut step_response_graph = Plotter::new("Step response", "Motor Speed", &plotter_context);
    let mut impulse_response_graph =
        Plotter::new("Impulse response", "Motor Speed", &plotter_context);

    for input in step {
        let output = (input - motor.last_output()) * pid.as_block() * motor.as_block();
        let _ = output >> step_response_graph.as_monitor();
        let _ = output >> impulse_response_graph.as_monitor();
    }

    keep_alive(plotter_context);
}
