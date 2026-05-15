pub trait FromUsize {
    fn from_usize(v: usize) -> Self;
}

impl FromUsize for f32 {
    fn from_usize(v: usize) -> Self {
        v as f32
    }
}

impl FromUsize for f64 {
    fn from_usize(v: usize) -> Self {
        v as f64
    }
}
