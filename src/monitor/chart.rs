use crate::monitor::{AsMonitor, Monitor};
use crate::signal::Signal;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use charming::ImageRenderer;
use charming::{Chart as CharmingChart, component::Axis, element::AxisType, series::Line};
use std::time::Duration;

pub struct Chart {
    title: String,
    datas: Vec<Vec<Signal>>,
    sim_time: Duration,
}

impl Chart {
    pub fn new(title: &str) -> Self {
        Chart {
            title: title.to_string(),
            datas: vec![],
            sim_time: Duration::default(),
        }
    }

    pub fn plot(&self) {
        let t = self.datas[0]
            .iter()
            .map(|s| s.dt.as_secs_f32().to_string())
            .collect::<Vec<_>>();

        let mut chart = CharmingChart::new()
            .x_axis(Axis::new().type_(AxisType::Category).data(t))
            .y_axis(Axis::new().type_(AxisType::Value));

        for y in &self.datas {
            let y = y.iter().map(|s| s.value).collect::<Vec<_>>();
            chart = chart.series(Line::new().data(y));
        }

        let mut renderer = ImageRenderer::new(600, 450);
        renderer
            .save(&chart, &self.title)
            .expect("Failed to save chart");
    }
}

impl Monitor for Chart {
    fn show(&mut self, inputs: Vec<Signal>) {
        self.sim_time += inputs[0].dt;

        if self.datas.len() < inputs.len() {
            self.datas.resize(inputs.len(), vec![]);
        }

        for (data, input) in self.datas.iter_mut().zip(inputs.iter()) {
            let signal = Signal {
                dt: self.sim_time,
                value: input.value,
            };

            data.push(signal.clone());
        }
    }
}

impl AsMonitor for Chart {}
