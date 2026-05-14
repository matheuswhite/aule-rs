pub trait FromF64 {
    fn from_f64(v: f64) -> Self;
}

impl FromF64 for f32 {
    fn from_f64(v: f64) -> Self {
        v as f32
    }
}

impl FromF64 for f64 {
    fn from_f64(v: f64) -> Self {
        v
    }
}
