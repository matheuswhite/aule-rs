use crate::block::{AsMonitor, Monitor, Signal};

pub struct Printer {
    title: String,
    unit: String,
}

impl Printer {
    pub fn new(title: &str, unit: &str) -> Self {
        Printer {
            title: title.to_string(),
            unit: unit.to_string(),
        }
    }
}

impl Monitor for Printer {
    fn show(&mut self, input: Signal) {
        println!("[{}] {} {}", self.title, input.value, self.unit);
    }
}

impl AsMonitor for Printer {}
