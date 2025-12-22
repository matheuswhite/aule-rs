use aule::prelude::*;
use aule::s;
use aule::tier2::SmithPredictorFiltered;
use std::time::Duration;

fn main() {
    open_loop();
    pi_controller();
    smith_predictor();
}

fn open_loop() {
    let time = Time::continuous(1e-2, 400.0);
    let mut step = Step::default();
    let mut plant = (5.6 / (40.2f64 * s + 1.0)).to_ss_controllable(RK4);
    let mut delay = Delay::new(Duration::from_secs_f32(93.9));
    let mut plotter = Plotter::new("[Long Dead Time] Open Loop".to_string());

    for dt in time {
        let reference = dt * step.as_block();

        let plant_output = reference * plant.as_block();
        let delayed_output = delay.output(plant_output);

        delayed_output * plotter.as_block() * IgnoreOutput;
    }

    plotter.display();
    plotter.join();
}

fn pi_controller() {
    let time = Time::continuous(1e-2, 2000.0);
    let mut step = Step::default();

    let kp = [0.06, 0.08, 0.1];
    let ti = 47.3;
    let mut controller = kp.map(|kp| PID::new(kp, kp / ti, 0.0));
    let mut plant = kp.map(|_| (5.6 / (40.2f64 * s + 1.0)).to_ss_controllable(RK4));
    let mut delay = kp.map(|_| Delay::new(Duration::from_secs_f32(93.9)));
    let mut plotter = Plotter::new(format!("[Long Dead Time] PI {:?}", kp));

    for dt in time {
        let reference = dt * step.as_block();
        let mut outputs = [(); 3].map(|_| dt.map(|_| 0.0));

        for i in 0..3 {
            let error = reference - delay[i].last_output();
            let control_signal = error * controller[i].as_block();

            let plant_output = control_signal * plant[i].as_block();
            outputs[i] = delay[i].output(plant_output);
        }

        let output =
            outputs
                .into_iter()
                .enumerate()
                .fold(dt.map(|_| [0.0; 3]), |mut acc, (i, v)| {
                    acc.value[i] = v.value;
                    acc.delta = acc.delta.merge(v.delta);
                    acc
                });
        plotter.output(output) * IgnoreOutput;
    }

    plotter.display();
    plotter.join();
}

fn smith_predictor() {
    let time = Time::continuous(1e-2, 700.0);
    let mut step = Step::default();
    let mut plotter = Plotter::new("[Long Dead Time] Smith Predictor vs Pure PI".to_string());
    let delay_value = Duration::from_secs_f32(93.9);

    let kp = 0.574;
    let ti = 40.2;
    let mut with_predictor = BlockCollection {
        controller: PID::new(kp, kp / ti, 0.0),
        plant: (5.6 / (40.2f64 * s + 1.0)).to_ss_controllable(RK4),
        delay: Delay::new(delay_value),
        smith_predictor: Some(SmithPredictorFiltered::new(
            (5.6 / (40.2f64 * s + 1.0)).to_ss_controllable(RK4),
            (1.0 / (2.0f64 * s + 1.0)).to_ss_controllable(RK4),
            delay_value,
        )),
    };

    let kp = 0.0501;
    let ti = 47.3;
    let mut without_predictor = BlockCollection {
        controller: PID::new(kp, kp / ti, 0.0),
        plant: (5.6 / (40.2f64 * s + 1.0)).to_ss_controllable(RK4),
        delay: Delay::new(delay_value),
        smith_predictor: None,
    };

    for dt in time {
        let reference = dt * step.as_block();

        let with_predictor_output = reference * with_predictor.as_block();
        let without_predictor_output = reference * without_predictor.as_block();

        let plotter_input = with_predictor_output
            .zip(without_predictor_output)
            .map(|(with, without)| [with, without]);

        plotter.output(plotter_input) * IgnoreOutput;
    }

    plotter.display();
    plotter.join();
}

type SmithPredictorFilteredRK4 =
    SmithPredictorFiltered<f64, SS<RK4, f64>, SS<RK4, f64>, Continuous>;

struct BlockCollection {
    controller: PID<f64, Continuous>,
    plant: SS<RK4, f64>,
    delay: Delay<f64, Continuous>,
    smith_predictor: Option<SmithPredictorFilteredRK4>,
}

impl Block for BlockCollection {
    type Input = f64;
    type Output = f64;
    type TimeType = Continuous;

    fn output(
        &mut self,
        input: Signal<Self::Input, Self::TimeType>,
    ) -> Signal<Self::Output, Self::TimeType> {
        if let Some(smith_predictor) = &mut self.smith_predictor {
            let preditor_last_output = smith_predictor.last_output();
            let error = input - preditor_last_output;
            let control_signal = error * self.controller.as_block();

            let plant_output = control_signal * self.plant.as_block();
            let delayed_output = plant_output * self.delay.as_block();
            let _predicted_output = control_signal.zip(delayed_output) * smith_predictor.as_block();

            delayed_output
        } else {
            let error = input - self.delay.last_output();
            let control_signal = error * self.controller.as_block();

            let plant_output = control_signal * self.plant.as_block();
            plant_output * self.delay.as_block()
        }
    }

    fn last_output(&self) -> Option<Self::Output> {
        self.delay.last_output()
    }
}
