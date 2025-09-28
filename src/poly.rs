use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};
use ndarray::Array2;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coeff: Vec<f32>,
}

impl Polynomial {
    pub fn new(coeff: &[f32]) -> Self {
        let output = Polynomial {
            coeff: coeff.to_vec(),
        };

        output.simplify()
    }

    pub fn empty() -> Self {
        Polynomial::new(&[])
    }

    fn simplify(self) -> Self {
        let mut coeff = self.coeff;
        while !coeff.is_empty() && coeff.first() == Some(&0.0) {
            coeff.remove(0);
        }
        Polynomial { coeff }
    }

    pub fn pow(self, exp: usize) -> Self {
        match exp {
            0 => Polynomial::new(&[1.0]),
            1 => self,
            _ => {
                let mut result = self.clone();
                for _ in 1..exp {
                    result = result * self.clone();
                }
                result
            }
        }
    }

    pub fn degree(&self) -> isize {
        self.coeff.len() as isize - 1
    }

    fn positive_degree(&self) -> usize {
        self.coeff.len() - 1
    }

    pub fn coeff(&self) -> &[f32] {
        &self.coeff
    }

    pub fn lead_coeff(&self) -> f32 {
        self.coeff.get(0).copied().unwrap_or(0.0)
    }

    pub fn companion_matrix(self) -> Array2<f32> {
        if self.degree() < 1 {
            return Array2::<f32>::default((0, 0));
        }

        let n = self.positive_degree();
        let mut lines = Vec::new();

        for i in 0..(n - 1) {
            let one_col = i + 1;
            let mut line = vec![0.0; n];
            line[one_col] = 1.0;
            lines.extend(line);
        }

        lines.extend(
            self.coeff[1..]
                .iter()
                .rev()
                .map(|&c| -c)
                .collect::<Vec<_>>(),
        );

        Array2::from_shape_vec((n, n), lines).unwrap()
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        if self.degree() < 0 {
            return rhs;
        }

        if rhs.degree() < 0 {
            return self;
        }

        let max_degree = self.positive_degree().max(rhs.positive_degree());
        let mut coeff = vec![0.0; max_degree + 1];
        let mut self_coeff = self.coeff.iter().rev();
        let mut rhs_coeff = rhs.coeff.iter().rev();

        for i in 0..=max_degree {
            let self_c = self_coeff.next().copied();
            let rhs_c = rhs_coeff.next().copied();

            if let (Some(self_c), Some(rhs_c)) = (self_c, rhs_c) {
                coeff[max_degree - i] = self_c + rhs_c;
            } else if let Some(self_c) = self_c {
                coeff[max_degree - i] = self_c;
            } else if let Some(rhs_c) = rhs_c {
                coeff[max_degree - i] = rhs_c;
            } else {
                unreachable!("Both coefficients should not be None");
            }
        }

        Polynomial { coeff }.simplify()
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Self::Output {
        if self.degree() < 0 {
            return -rhs;
        }

        if rhs.degree() < 0 {
            return self;
        }

        let max_degree = self.positive_degree().max(rhs.positive_degree());
        let mut coeff = vec![0.0; max_degree + 1];
        let mut self_coeff = self.coeff.iter().rev();
        let mut rhs_coeff = rhs.coeff.iter().rev();

        for i in 0..=max_degree {
            let self_c = self_coeff.next().copied();
            let rhs_c = rhs_coeff.next().copied();

            if let (Some(self_c), Some(rhs_c)) = (self_c, rhs_c) {
                coeff[max_degree - i] = self_c - rhs_c;
            } else if let Some(self_c) = self_c {
                coeff[max_degree - i] = self_c;
            } else if let Some(rhs_c) = rhs_c {
                coeff[max_degree - i] = -rhs_c;
            } else {
                unreachable!("Both coefficients should not be None");
            }
        }

        Polynomial { coeff }.simplify()
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        if self.degree() < 0 || rhs.degree() < 0 {
            return Polynomial::empty();
        }

        let mut coeff = vec![0.0; self.positive_degree() + rhs.positive_degree() + 1];

        for (i, &a) in self.coeff.iter().enumerate() {
            for (j, &b) in rhs.coeff.iter().enumerate() {
                coeff[i + j] += a * b;
            }
        }

        let output = Polynomial { coeff };

        output.simplify()
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Polynomial {
            coeff: self.coeff.iter().map(|&c| -c).collect(),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let degree = self.degree();
        let string = self
            .coeff()
            .iter()
            .enumerate()
            .map(|(i, &coeff)| {
                let i = degree - i as isize;
                if i == 0 {
                    format!("{}", coeff)
                } else if i == 1 {
                    format!("{}*x", coeff)
                } else {
                    format!("{}*x^{}", coeff, i)
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");

        write!(f, "{}", string)
    }
}
