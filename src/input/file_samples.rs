use crate::{
    block::Block,
    math::{float_point::FloatPoint, sample::Sample},
    prelude::SimulationState,
};
use core::time::Duration;
use std::{fs::File, path::PathBuf};

use nalgebra::{DMatrix, SMatrix};
use num_complex::Complex;

pub struct FileSamples<T> {
    path: PathBuf,
    reader: csv::Reader<File>,
    time_index: usize,
    value_index: usize,
    current_record: Option<Record<T>>,
    last_record: Option<Record<T>>,
}

#[derive(Clone)]
struct Record<T> {
    time: Duration,
    value: T,
}

pub trait FromCsvRecord: Sized {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self>;
}

impl FromCsvRecord for f32 {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        record.get(value_index).and_then(|s| s.parse::<f32>().ok())
    }
}

impl FromCsvRecord for f64 {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        record.get(value_index).and_then(|s| s.parse::<f64>().ok())
    }
}

impl FromCsvRecord for Complex<f32> {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        record
            .get(value_index)
            .and_then(|s| s.parse::<Complex<f32>>().ok())
    }
}

impl FromCsvRecord for Complex<f64> {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        record
            .get(value_index)
            .and_then(|s| s.parse::<Complex<f64>>().ok())
    }
}

impl FromCsvRecord for DMatrix<f32> {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        let mut values = std::vec::Vec::new();
        let mut i = value_index;
        while let Some(s) = record.get(i) {
            let v: f32 = s.parse().ok()?;
            values.push(v);
            i += 1;
        }
        if values.is_empty() {
            return None;
        }
        let n = values.len();
        Some(DMatrix::from_fn(n, 1, |i, _| values[i]))
    }
}

impl FromCsvRecord for DMatrix<f64> {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        let mut values = std::vec::Vec::new();
        let mut i = value_index;
        while let Some(s) = record.get(i) {
            let v: f64 = s.parse().ok()?;
            values.push(v);
            i += 1;
        }
        if values.is_empty() {
            return None;
        }
        let n = values.len();
        Some(DMatrix::from_fn(n, 1, |i, _| values[i]))
    }
}

impl<const R: usize, const C: usize> FromCsvRecord for SMatrix<f32, R, C> {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        let count = R * C;
        let mut values = std::vec::Vec::with_capacity(count);
        for i in 0..count {
            let s = record.get(value_index + i)?;
            let v: f32 = s.parse().ok()?;
            values.push(v);
        }
        Some(SMatrix::from_row_iterator(values))
    }
}

impl<const R: usize, const C: usize> FromCsvRecord for SMatrix<f64, R, C> {
    fn from_csv_record(record: &csv::StringRecord, value_index: usize) -> Option<Self> {
        let count = R * C;
        let mut values = std::vec::Vec::with_capacity(count);
        for i in 0..count {
            let s = record.get(value_index + i)?;
            let v: f64 = s.parse().ok()?;
            values.push(v);
        }
        Some(SMatrix::from_row_iterator(values))
    }
}

impl<T> FileSamples<T>
where
    T: FromCsvRecord,
{
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
            last_record: None,
        };

        obj.current_record = obj.next_record();

        Ok(obj)
    }

    fn next_record(&mut self) -> Option<Record<T>> {
        let records = self.reader.records();
        let Some(Ok(rec)) = records.into_iter().next() else {
            return None;
        };

        let time = rec
            .get(self.time_index)
            .and_then(|time| time.parse::<f64>().ok())?;
        let value = T::from_csv_record(&rec, self.value_index)?;

        let record = Record {
            time: Duration::from_secs_f64(time),
            value,
        };

        Some(record)
    }
}

impl<T> Clone for FileSamples<T>
where
    T: FromCsvRecord,
{
    fn clone(&self) -> Self {
        Self::from_csv(
            self.path.to_str().expect("BUG: path was valid when FileSamples was first created, but failed to convert to str on clone"),
            self.time_index,
            self.value_index,
        )
        .expect("BUG: FileSamples was valid before cloning, but failed to re-open CSV file during clone")
    }
}

impl<T> Block for FileSamples<T>
where
    T: FromCsvRecord + Sample,
{
    type Input = ();
    type Output = Option<T>;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let record = self.current_record.clone()?;

        if sim_state.sim_time() >= record.time {
            self.last_record = Some(record);
            self.current_record = self.next_record();
        }

        let record = self.current_record.clone()?;
        let Some(last_record) = self.last_record.as_ref() else {
            return Some(record.value);
        };

        let start = last_record.time.as_secs_f64();
        let end = record.time.as_secs_f64();

        if end == start {
            return Some(record.value);
        }

        let alpha = <T::Alpha as FloatPoint>::from_duration(sim_state.sim_time());

        Some(T::lerp(&last_record.value, &record.value, alpha))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use std::format;
    use std::io::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn write_tmp_csv(contents: &str) -> PathBuf {
        let id = TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let path = std::env::temp_dir().join(format!("aule_file_samples_{}_{}.csv", pid, id));
        let mut f = File::create(&path).expect("failed to create tmp csv");
        f.write_all(contents.as_bytes())
            .expect("failed to write tmp csv");
        path
    }

    fn make_state(sim_time_s: f64, dt_s: f64) -> SimulationState {
        let mut sim = Simulation::new(dt_s as f32, (sim_time_s + dt_s * 2.0) as f32);
        let initial = sim
            .next()
            .expect("simulation should yield at least one state");
        let delta = Duration::from_secs_f64(sim_time_s) - initial.sim_time();
        initial + delta
    }

    fn approx_eq_f64(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    fn approx_eq_f32(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() < tol
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_returns_first_value_before_first_sample() {
        let path = write_tmp_csv("time,value\n1.0,10.0\n2.0,20.0\n");
        let mut fs = FileSamples::<f64>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.5);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v, 10.0, 1e-9), "got {v}");
    }

    #[test]
    fn f64_interpolates_between_samples() {
        let path = write_tmp_csv("time,value\n0.0,0.0\n1.0,10.0\n2.0,20.0\n");
        let mut fs = FileSamples::<f64>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(1.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v, 15.0, 1e-6), "got {v}");
    }

    #[test]
    fn f64_midpoint_interpolation() {
        let path = write_tmp_csv("time,value\n0.0,0.0\n1.0,10.0\n");
        let mut fs = FileSamples::<f64>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v, 5.0, 1e-6), "got {v}");
    }

    #[test]
    fn f64_returns_none_past_last_sample() {
        let path = write_tmp_csv("time,value\n0.0,0.0\n1.0,10.0\n");
        let mut fs = FileSamples::<f64>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let dt = 0.5;
        let mut sim = Simulation::new(dt as f32, 3.0);
        let mut last_some = None;
        let mut last_result = None;
        while let Some(state) = sim.next() {
            let v = fs.block((), state);
            if v.is_some() {
                last_some = v;
            }
            last_result = Some(v);
        }
        assert!(last_some.is_some(), "expected at least one Some");
        assert_eq!(
            last_result,
            Some(None),
            "expected None after exhausting samples"
        );
    }

    #[test]
    fn f64_custom_column_indices() {
        let path = write_tmp_csv("v,t\n5.0,0.0\n15.0,1.0\n");
        let mut fs = FileSamples::<f64>::from_csv(path.to_str().unwrap(), 1, 0).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v, 10.0, 1e-6), "got {v}");
    }

    #[test]
    fn f64_clone_preserves_behavior() {
        let path = write_tmp_csv("time,value\n0.0,0.0\n1.0,10.0\n");
        let fs = FileSamples::<f64>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let mut clone = fs.clone();
        let state = make_state(0.5, 0.1);
        let v = clone.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v, 5.0, 1e-6), "got {v}");
    }

    // ───────────────────────────── f32 ─────────────────────────────

    #[test]
    fn f32_returns_first_value_before_first_sample() {
        let path = write_tmp_csv("time,value\n1.0,10.0\n2.0,20.0\n");
        let mut fs = FileSamples::<f32>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.5);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f32(v, 10.0, 1e-6), "got {v}");
    }

    #[test]
    fn f32_midpoint_interpolation() {
        let path = write_tmp_csv("time,value\n0.0,0.0\n1.0,10.0\n");
        let mut fs = FileSamples::<f32>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f32(v, 5.0, 1e-4), "got {v}");
    }

    #[test]
    fn f32_interpolates_between_samples() {
        let path = write_tmp_csv("time,value\n0.0,0.0\n1.0,10.0\n2.0,20.0\n");
        let mut fs = FileSamples::<f32>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(1.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f32(v, 15.0, 1e-4), "got {v}");
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_returns_first_value_before_first_sample() {
        let path = write_tmp_csv("time,value\n1.0,10+2i\n2.0,20+4i\n");
        let mut fs = FileSamples::<Complex<f64>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.5);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v.re, 10.0, 1e-9), "re: {}", v.re);
        assert!(approx_eq_f64(v.im, 2.0, 1e-9), "im: {}", v.im);
    }

    #[test]
    fn complex_f64_midpoint_interpolation() {
        let path = write_tmp_csv("time,value\n0.0,0+0i\n1.0,10+4i\n");
        let mut fs = FileSamples::<Complex<f64>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v.re, 5.0, 1e-6), "re: {}", v.re);
        assert!(approx_eq_f64(v.im, 2.0, 1e-6), "im: {}", v.im);
    }

    #[test]
    fn complex_f64_full_traversal() {
        let path = write_tmp_csv("time,value\n0.0,0+0i\n1.0,10+4i\n2.0,20+8i\n");
        let mut fs = FileSamples::<Complex<f64>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(1.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v.re, 15.0, 1e-6), "re: {}", v.re);
        assert!(approx_eq_f64(v.im, 6.0, 1e-6), "im: {}", v.im);
    }

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_midpoint_interpolation() {
        let path = write_tmp_csv("time,value\n0.0,0+0i\n1.0,10+4i\n");
        let mut fs = FileSamples::<Complex<f32>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f32(v.re, 5.0, 1e-4), "re: {}", v.re);
        assert!(approx_eq_f32(v.im, 2.0, 1e-4), "im: {}", v.im);
    }

    // ───────────────────────────── SMatrix<f64, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f64_column_vector_returns_first_value_before_first_sample() {
        let path = write_tmp_csv("time,a,b,c\n1.0,1.0,2.0,3.0\n2.0,4.0,5.0,6.0\n");
        let mut fs =
            FileSamples::<SMatrix<f64, 3, 1>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.5);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v[(0, 0)], 1.0, 1e-9));
        assert!(approx_eq_f64(v[(1, 0)], 2.0, 1e-9));
        assert!(approx_eq_f64(v[(2, 0)], 3.0, 1e-9));
    }

    #[test]
    fn smatrix_f64_column_vector_midpoint_interpolation() {
        let path = write_tmp_csv("time,a,b,c\n0.0,0.0,0.0,0.0\n1.0,2.0,4.0,6.0\n");
        let mut fs =
            FileSamples::<SMatrix<f64, 3, 1>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v[(0, 0)], 1.0, 1e-6), "{}", v[(0, 0)]);
        assert!(approx_eq_f64(v[(1, 0)], 2.0, 1e-6), "{}", v[(1, 0)]);
        assert!(approx_eq_f64(v[(2, 0)], 3.0, 1e-6), "{}", v[(2, 0)]);
    }

    #[test]
    fn smatrix_f64_square_matrix_midpoint_interpolation() {
        // CSV row-major: m00, m01, m10, m11
        let path =
            write_tmp_csv("time,m00,m01,m10,m11\n0.0,0.0,0.0,0.0,0.0\n1.0,2.0,4.0,6.0,8.0\n");
        let mut fs =
            FileSamples::<SMatrix<f64, 2, 2>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f64(v[(0, 0)], 1.0, 1e-6), "{}", v[(0, 0)]);
        assert!(approx_eq_f64(v[(0, 1)], 2.0, 1e-6), "{}", v[(0, 1)]);
        assert!(approx_eq_f64(v[(1, 0)], 3.0, 1e-6), "{}", v[(1, 0)]);
        assert!(approx_eq_f64(v[(1, 1)], 4.0, 1e-6), "{}", v[(1, 1)]);
    }

    // ───────────────────────────── SMatrix<f32, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f32_column_vector_midpoint_interpolation() {
        let path = write_tmp_csv("time,a,b,c\n0.0,0.0,0.0,0.0\n1.0,2.0,4.0,6.0\n");
        let mut fs =
            FileSamples::<SMatrix<f32, 3, 1>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f32(v[(0, 0)], 1.0, 1e-4), "{}", v[(0, 0)]);
        assert!(approx_eq_f32(v[(1, 0)], 2.0, 1e-4), "{}", v[(1, 0)]);
        assert!(approx_eq_f32(v[(2, 0)], 3.0, 1e-4), "{}", v[(2, 0)]);
    }

    #[test]
    fn smatrix_f32_square_matrix_midpoint_interpolation() {
        let path =
            write_tmp_csv("time,m00,m01,m10,m11\n0.0,0.0,0.0,0.0,0.0\n1.0,2.0,4.0,6.0,8.0\n");
        let mut fs =
            FileSamples::<SMatrix<f32, 2, 2>>::from_csv(path.to_str().unwrap(), 0, 1).unwrap();
        let state = make_state(0.5, 0.1);
        let v = fs.block((), state).expect("should produce a value");
        assert!(approx_eq_f32(v[(0, 0)], 1.0, 1e-4), "{}", v[(0, 0)]);
        assert!(approx_eq_f32(v[(0, 1)], 2.0, 1e-4), "{}", v[(0, 1)]);
        assert!(approx_eq_f32(v[(1, 0)], 3.0, 1e-4), "{}", v[(1, 0)]);
        assert!(approx_eq_f32(v[(1, 1)], 4.0, 1e-4), "{}", v[(1, 1)]);
    }
}
