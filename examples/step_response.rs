use aule::prelude::*;
use aule::tier3::StepResponse;

fn main() {
    let plant = ClosedLoop::default();

    let mut step_response = StepResponse::new(plant).with_tolerance(1e-5);
    let info = step_response.run();

    println!("{}", info);
}

struct ClosedLoop {
    plant: SS<RK4, f64>,
    pid: PID<f64, Continuous>,
}

impl Default for ClosedLoop {
    fn default() -> Self {
        Self {
            plant: Tf::new(&[1.0], &[1.0, 6.0, 11.0, 6.0]).to_ss_controllable(RK4),
            pid: PID::new(40.0, 10.0, 10.0),
        }
    }
}

impl Block for ClosedLoop {
    type Input = f64;
    type Output = f64;
    type TimeType = Continuous;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        let error = input - self.plant.last_output();
        let control_signal = error * self.pid.as_block();
        control_signal * self.plant.as_block()
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.plant.last_output()
    }
}
