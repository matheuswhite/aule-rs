use crate::{discrete::integration::Discretizable, prelude::Euler};

pub struct Tf {
    numerator: crate::continuous::poly::Polynomial,
    denominator: crate::continuous::poly::Polynomial,
}

impl Tf {
    pub fn new(numerator: &[f32], denominator: &[f32]) -> Self {
        Tf {
            numerator: crate::continuous::poly::Polynomial::new(numerator),
            denominator: crate::continuous::poly::Polynomial::new(denominator),
        }
    }
}

impl Discretizable<Euler> for Tf {
    fn discretize(self) -> Euler {
        Euler::new(
            self.numerator.inner().clone(),
            self.denominator.inner().clone(),
        )
    }
}
