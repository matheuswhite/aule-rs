use aule::prelude::*;
use aule::s;
use aule::tier1::smith_predictor::SmithPredictorFiltered;
use std::time::Duration;

fn main() {
    open_loop();
    pi_controller();
    smith_predictor();
}

fn open_loop() {
    let time = Time::continuous(1e-2, 400.0);
    let mut step = Step::default();
    let mut plant: SS<RK4> = (5.6 / (40.2 * s + 1.0)).into();
    let mut delay = Delay::new(Duration::from_secs_f32(93.9));
    let mut plotter = Plotter::new("[Long Dead Time] Open Loop".to_string(), 100.0, 1.0);

    for dt in time {
        let reference = dt * step.as_block();

        let plant_output = reference * plant.as_block();
        let delayed_output = plant_output * delay.as_block();

        let _ = delayed_output * plotter.as_block();
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
    let mut plant = kp.map(|_| SS::<RK4>::from(5.6 / (40.2 * s + 1.0)));
    let mut delay = kp.map(|_| Delay::new(Duration::from_secs_f32(93.9)));
    let mut plotter = Plotter::new(format!("[Long Dead Time] PI {:?}", kp), 100.0, 0.2);

    for dt in time {
        let reference = dt * step.as_block();
        let mut outputs = [(); 3].map(|_| dt.map(|_| 0.0));

        for i in 0..3 {
            let error = reference - delay[i].last_output();
            let control_signal = error * controller[i].as_block();

            let plant_output = control_signal * plant[i].as_block();
            outputs[i] = plant_output * delay[i].as_block();
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
        let _ = output * plotter.as_block();
    }

    plotter.display();
    plotter.join();
}

fn smith_predictor() {
    let time = Time::continuous(1e-2, 700.0);
    let mut step = Step::default();
    let mut plotter = Plotter::new(
        "[Long Dead Time] Smith Predictor vs Pure PI".to_string(),
        100.0,
        0.2,
    );
    let delay_value = Duration::from_secs_f32(93.9);

    let kp = 0.574;
    let ti = 40.2;
    let mut with_predictor = BlockCollection {
        controller: PID::new(kp, kp / ti, 0.0),
        plant: SS::<RK4>::from(5.6 / (40.2 * s + 1.0)),
        delay: Delay::new(delay_value),
        smith_predictor: Some(SmithPredictorFiltered::new(
            SS::<RK4>::from(5.6 / (40.2 * s + 1.0)),
            SS::<RK4>::from(1.0 / (2.0 * s + 1.0)),
            delay_value,
        )),
    };

    let kp = 0.0501;
    let ti = 47.3;
    let mut without_predictor = BlockCollection {
        controller: PID::new(kp, kp / ti, 0.0),
        plant: SS::<RK4>::from(5.6 / (40.2 * s + 1.0)),
        delay: Delay::new(delay_value),
        smith_predictor: None,
    };

    for dt in time {
        let reference = dt * step.as_block();

        let with_predictor_output = reference * with_predictor.as_block();
        let without_predictor_output = reference * without_predictor.as_block();

        let _ = with_predictor_output
            .zip(without_predictor_output)
            .map(|(with, without)| [with, without])
            * plotter.as_block();
    }

    plotter.display();
    plotter.join();
}

type SmithPredictorFilteredRK4 = SmithPredictorFiltered<SS<RK4>, SS<RK4>, Continuous>;

struct BlockCollection {
    controller: PID<Continuous>,
    plant: SS<RK4>,
    delay: Delay<Continuous>,
    smith_predictor: Option<SmithPredictorFilteredRK4>,
}

impl Block for BlockCollection {
    type Input = f32;
    type Output = f32;
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
