pub trait RecipOfCount {
    fn recip_of_count(count: usize) -> Self;
}

impl RecipOfCount for f32 {
    fn recip_of_count(count: usize) -> Self {
        1.0 / count as f32
    }
}

impl RecipOfCount for f64 {
    fn recip_of_count(count: usize) -> Self {
        1.0 / count as f64
    }
}
