use std::time::Duration;

use crate::monitor::{AsMonitor, Monitor};
use crate::signal::Signal;
use charming::ImageRenderer;
use charming::{Chart as CharmingChart, component::Axis, element::AxisType, series::Line};

pub struct Chart {
    title: String,
    data: Vec<Signal>,
    sim_time: Duration,
}

impl Chart {
    pub fn new(title: &str) -> Self {
        Chart {
            title: title.to_string(),
            data: vec![],
            sim_time: Duration::default(),
        }
    }

    pub fn plot(&self) {
        let t = self
            .data
            .iter()
            .map(|s| s.dt.as_secs_f32().to_string())
            .collect::<Vec<_>>();
        let y = self.data.iter().map(|s| s.value).collect::<Vec<_>>();

        let chart = CharmingChart::new()
            .x_axis(Axis::new().type_(AxisType::Category).data(t))
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(y));

        let mut renderer = ImageRenderer::new(600, 450);
        renderer
            .save(&chart, &self.title)
            .expect("Failed to save chart");
    }
}

impl Monitor for Chart {
    fn show(&mut self, inputs: Signal) {
        self.sim_time += inputs.dt;
        let signal = Signal {
            dt: self.sim_time,
            value: inputs.value,
        };
        self.data.push(signal);
    }
}

impl AsMonitor for Chart {}
