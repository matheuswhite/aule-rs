use nalgebra::{Complex, DMatrix, SMatrix};

pub trait Absolute: Sized {
    fn absolute(&self) -> Self;
}

impl Absolute for f32 {
    fn absolute(&self) -> Self {
        self.abs()
    }
}

impl Absolute for f64 {
    fn absolute(&self) -> Self {
        self.abs()
    }
}

impl Absolute for Complex<f32> {
    fn absolute(&self) -> Self {
        Complex {
            re: self.re.abs(),
            im: self.im.abs(),
        }
    }
}

impl Absolute for Complex<f64> {
    fn absolute(&self) -> Self {
        Complex {
            re: self.re.abs(),
            im: self.im.abs(),
        }
    }
}

impl Absolute for DMatrix<f32> {
    fn absolute(&self) -> Self {
        self.map(|x| x.abs())
    }
}

impl Absolute for DMatrix<f64> {
    fn absolute(&self) -> Self {
        self.map(|x| x.abs())
    }
}

impl<const R: usize, const C: usize> Absolute for SMatrix<f32, R, C> {
    fn absolute(&self) -> Self {
        self.map(|x| x.abs())
    }
}

impl<const R: usize, const C: usize> Absolute for SMatrix<f64, R, C> {
    fn absolute(&self) -> Self {
        self.map(|x| x.abs())
    }
}

impl Absolute for DMatrix<Complex<f32>> {
    fn absolute(&self) -> Self {
        self.map(|x| x.absolute())
    }
}

impl Absolute for DMatrix<Complex<f64>> {
    fn absolute(&self) -> Self {
        self.map(|x| x.absolute())
    }
}

impl<const R: usize, const C: usize> Absolute for SMatrix<Complex<f32>, R, C> {
    fn absolute(&self) -> Self {
        self.map(|x| x.absolute())
    }
}

impl<const R: usize, const C: usize> Absolute for SMatrix<Complex<f64>, R, C> {
    fn absolute(&self) -> Self {
        self.map(|x| x.absolute())
    }
}
