use crate::{block::Block, prelude::SimulationState};
use std::{fs::File, path::PathBuf};

pub struct FileSamples {
    path: PathBuf,
    reader: csv::Reader<File>,
    time_index: usize,
    value_index: usize,
    current_record: Option<Record>,
    last_record: Record,
}

#[derive(Clone)]
struct Record {
    time: f64,
    value: f64,
}

impl FileSamples {
    pub fn from_csv(
        path: impl AsRef<str>,
        time_index: usize,
        value_index: usize,
    ) -> Result<Self, csv::Error> {
        let mut obj = Self {
            path: PathBuf::from(path.as_ref()),
            reader: csv::Reader::from_path(path.as_ref())?,
            time_index,
            value_index,
            current_record: None,
            last_record: Record {
                time: 0.0,
                value: 0.0,
            },
        };

        obj.current_record = obj.next_record();

        Ok(obj)
    }

    fn next_record(&mut self) -> Option<Record> {
        let records = self.reader.records();
        let Some(Ok(rec)) = records.into_iter().next() else {
            return None;
        };

        let time = rec
            .get(self.time_index)
            .map(|time| time.parse::<f64>().ok())
            .flatten()?;
        let value = rec
            .get(self.value_index)
            .map(|value| value.parse::<f64>().ok())
            .flatten()?;

        let record = Record { time, value };

        Some(record)
    }
}

impl Clone for FileSamples {
    fn clone(&self) -> Self {
        Self::from_csv(
            self.path.to_str().expect("BUG: path was valid when FileSamples was first created, but failed to convert to str on clone"),
            self.time_index,
            self.value_index,
        )
        .expect("BUG: FileSamples was valid before cloning, but failed to re-open CSV file during clone")
    }
}

impl Block for FileSamples {
    type Input = ();
    type Output = Option<f64>;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let Some(record) = self.current_record.clone() else {
            return None;
        };

        if sim_state.sim_time().as_secs_f64() >= record.time {
            self.last_record = record;
            self.current_record = self.next_record();
        }

        let Some(record) = self.current_record.clone() else {
            return None;
        };

        let start = self.last_record.time;
        let end = record.time;

        if end == start {
            return Some(record.value);
        }

        let t = sim_state.sim_time().as_secs_f64();
        let alpha = (t - start) / (end - start);

        let value = self.last_record.value + alpha * (record.value - self.last_record.value);

        Some(value)
    }
}
