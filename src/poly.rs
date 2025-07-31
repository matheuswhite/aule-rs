use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify() {
        let p = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
        assert_eq!(p.coeff(), &[1.0, 2.0]);
    }

    #[test]
    fn test_simplify_zero() {
        let p = Polynomial::new(&[0.0]);
        let expected: &[f32] = &[];
        assert_eq!(p.coeff(), expected);
    }

    #[test]
    fn test_add() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[0.0, 4.0, 5.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 6.0, 8.0]);
    }

    #[test]
    fn test_add_diff_degree() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[4.0, 5.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 6.0, 8.0]);
    }

    #[test]
    fn test_empty_add() {
        let p1 = Polynomial::empty();
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 2.0]);
    }

    #[test]
    fn test_add_result_empty() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[-1.0, -2.0, -3.0]);
        let result = p1 + p2;
        let expected: &[f32] = &[];
        assert_eq!(result.coeff(), expected);
    }

    #[test]
    fn test_sub() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[0.0, 4.0, 5.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[1.0, -2.0, -2.0]);
    }

    #[test]
    fn test_sub_diff_degree() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[4.0, 5.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[1.0, -2.0, -2.0]);
    }

    #[test]
    fn test_empty_sub() {
        let p1 = Polynomial::empty();
        let p2 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[-1.0, -2.0, -3.0]);
    }

    #[test]
    fn test_empty_result_sub() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let result = p1 - p2;
        let expected: &[f32] = &[];
        assert_eq!(result.coeff(), expected);
    }

    #[test]
    fn test_mul() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = Polynomial::new(&[3.0, 4.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[3.0, 10.0, 8.0]);
    }

    #[test]
    fn test_mul_diff_degree() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = Polynomial::new(&[4.0, 5.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[4.0, 13.0, 22.0, 15.0]);
    }

    #[test]
    fn test_mul_single_term() {
        let p1 = Polynomial::new(&[1.0, 0.0]);
        let p2 = Polynomial::new(&[1.0, 1.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[1.0, 1.0, 0.0]);
    }

    #[test]
    fn test_empty_mul() {
        let p1 = Polynomial::empty();
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 * p2;
        let expected: &[f32] = &[];
        assert_eq!(result.coeff(), expected);
    }
}
