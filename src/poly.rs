use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};
use ndarray::Array2;

/// A polynomial represented by its coefficients.
/// The coefficients are stored in a vector, where the index represents the power of x.
/// For example, a polynomial `2*x^2 + 3*x + 4` would be represented as `[2.0, 3.0, 4.0]`.
/// The first element is the coefficient for `x^0`, the second for `x^1`, and so on.
///
/// The polynomial is simplified to remove leading zeros.
/// For example, `[0.0, 2.0, 3.0]` would be simplified to `[2.0, 3.0]`.
/// The degree of the polynomial is determined by the length of the coefficients vector minus one.
/// If the coefficients vector is empty, the polynomial is considered to have a degree of -1.
/// The lead coefficient is the first element of the coefficients vector.
/// If the coefficients vector is empty, the lead coefficient is 0.0.
/// The polynomial supports addition, subtraction, multiplication, and negation operations.
/// It also provides methods to get the degree, coefficients, and lead coefficient.
/// The polynomial can be raised to a power using the `pow` method.
/// The polynomial can be printed in a human-readable format using the `Display` trait.
///
/// # Example
/// ```
/// use aule::poly::Polynomial;
///
/// let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
/// let p2 = Polynomial::new(&[4.0, 5.0]);
/// let result = p1.clone() + p2.clone();
/// assert_eq!(result.coeff(), &[1.0, 6.0, 8.0]);
/// let result = p1.clone() - p2.clone();
/// assert_eq!(result.coeff(), &[1.0, -2.0, -2.0]);
/// let result = p1.clone() * p2;
/// assert_eq!(result.coeff(), &[4.0, 13.0, 22.0, 15.0]);
/// let result = p1.clone().pow(2);
/// assert_eq!(result.coeff(), &[1.0, 4.0, 10.0, 12.0, 9.0]);
/// let result = -p1;
/// assert_eq!(result.coeff(), &[-1.0, -2.0, -3.0]);
/// let p3 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
/// assert_eq!(p3.coeff(), &[1.0, 2.0]);
/// let p4 = Polynomial::new(&[0.0]);
/// let empty: &[f32] = &[];
/// assert_eq!(p4.coeff(), empty);
/// let p5 = Polynomial::empty();
/// assert_eq!(p5.coeff(), empty);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    coeff: Vec<f32>,
}

impl Polynomial {
    /// Creates a new polynomial from the given coefficients.
    /// The coefficients should be provided in order of increasing power.
    /// For example, to create the polynomial `2*x^2 + 3*x + 4`, you would pass `[2.0, 3.0, 4.0]`.
    /// The polynomial is simplified to remove leading zeros.
    ///
    /// # Parameters
    /// * `coeff` - A slice of f32 values representing the coefficients of the polynomial.
    /// # Returns
    /// A new `Polynomial` instance with the specified coefficients.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.coeff(), &[1.0, 2.0, 3.0]);
    /// ```
    pub fn new(coeff: &[f32]) -> Self {
        let output = Polynomial {
            coeff: coeff.to_vec(),
        };

        output.simplify()
    }

    /// Creates an empty polynomial with no coefficients.
    /// This is equivalent to a polynomial with a degree of -1.
    /// The empty polynomial is represented by an empty vector of coefficients.
    ///
    /// # Returns
    /// A new `Polynomial` instance with no coefficients.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::empty();
    /// let empty: &[f32] = &[];
    /// assert_eq!(p.coeff(), empty);
    /// ```
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

    /// Raises the polynomial to the specified power.
    /// If the exponent is 0, it returns a polynomial with a single coefficient of 1.0.
    /// If the exponent is 1, it returns the polynomial itself.
    /// For any other positive exponent, it multiplies the polynomial by itself the specified number of times.
    ///
    /// # Parameters
    /// * `exp` - The exponent to which the polynomial should be raised.
    /// # Returns
    /// A new `Polynomial` instance representing the polynomial raised to the specified power.
    /// If the exponent is 0, it returns a polynomial with a single coefficient of 1.0.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let result = p.clone().pow(2);
    /// assert_eq!(result.coeff(), &[1.0, 4.0, 10.0, 12.0, 9.0]);
    /// let result = p.clone().pow(0);
    /// assert_eq!(result.coeff(), &[1.0]);
    /// let result = p.clone().pow(1);
    /// assert_eq!(result.coeff(), &[1.0, 2.0, 3.0]);
    /// let result = p.pow(3);
    /// assert_eq!(result.coeff(), &[1.0, 6.0, 21.0, 44.0, 63.0, 54.0, 27.0]);
    /// ```
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

    /// Returns the degree of the polynomial.
    /// The degree is defined as the highest power of x with a non-zero coefficient.
    /// If the polynomial has no coefficients, it is considered to have a degree of -1.
    /// The degree is calculated as the length of the coefficients vector minus one.
    /// If the coefficients vector is empty, the degree is -1.
    ///
    /// # Returns
    /// The degree of the polynomial as an `isize`.
    /// If the polynomial is empty, it returns -1.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.degree(), 2);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// assert_eq!(p2.degree(), 1);
    /// let p3 = Polynomial::empty();
    /// assert_eq!(p3.degree(), -1);
    /// ```
    pub fn degree(&self) -> isize {
        self.coeff.len() as isize - 1
    }

    fn positive_degree(&self) -> usize {
        self.coeff.len() - 1
    }

    /// Returns the coefficients of the polynomial.
    /// The coefficients are returned as a slice of f32 values.
    /// The coefficients are in order of increasing power, starting from the constant term.
    /// If the polynomial is empty, it returns an empty slice.
    ///
    /// # Returns
    /// A slice of f32 values representing the coefficients of the polynomial.
    /// If the polynomial is empty, it returns an empty slice.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.coeff(), &[1.0, 2.0, 3.0]);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// assert_eq!(p2.coeff(), &[1.0, 2.0]);
    /// let p3 = Polynomial::empty();
    /// let empty: &[f32] = &[];
    /// assert_eq!(p3.coeff(), empty);
    /// ```
    pub fn coeff(&self) -> &[f32] {
        &self.coeff
    }

    /// Returns the lead coefficient of the polynomial.
    /// The lead coefficient is the first element of the coefficients vector.
    /// If the coefficients vector is empty, the lead coefficient is 0.0.
    ///
    /// # Returns
    /// The lead coefficient of the polynomial as an f32.
    /// If the polynomial is empty, it returns 0.0.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.lead_coeff(), 1.0);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// assert_eq!(p2.lead_coeff(), 1.0);
    /// let p3 = Polynomial::empty();
    /// assert_eq!(p3.lead_coeff(), 0.0);
    /// ```
    pub fn lead_coeff(&self) -> f32 {
        self.coeff.get(0).copied().unwrap_or(0.0)
    }

    /// Returns the companion matrix of the polynomial.
    /// The size of the matrix is determined by the degree of the polynomial.
    /// The matrix is constructed such that the first row contains the coefficients of the polynomial,
    /// and the subsequent rows are filled with zeros except for the last row, which contains the negated coefficients.
    /// If the polynomial is less than one, it returns an empty matrix.
    ///
    /// # Returns
    /// An `Array2<f32>` representing the companion matrix of the polynomial.
    /// If the polynomial is less than one, it returns an empty matrix.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    /// use ndarray::Array2;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let companion = p.companion_matrix();
    /// assert_eq!(companion, Array2::from_shape_vec((2, 2), vec![
    /// 0.0, 1.0,
    /// -3.0, -2.0,
    /// ]).unwrap());
    /// let p2 = Polynomial::empty();
    /// let companion2 = p2.companion_matrix();
    /// assert_eq!(companion2, Array2::<f32>::default((0, 0)));
    /// let p3 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// let companion3 = p3.companion_matrix();
    /// assert_eq!(companion3, Array2::from_shape_vec((3, 3), vec![
    /// 0.0, 1.0, 0.0,
    /// 0.0, 0.0, 1.0,
    /// -4.0, -3.0, -2.0,
    /// ]).unwrap());
    /// ```
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

    /// Adds two polynomials together.
    /// If either polynomial is empty, it returns an empty polynomial.
    /// If both polynomials have coefficients, it adds their coefficients together.
    /// If the degree of one of the polynomials is less than the other, it pads the coefficients with the remaining values of the higher degree polynomial.
    /// The resulting polynomial has coefficients that are the sum of the corresponding coefficients of the two polynomials.
    /// If the resulting polynomial has no coefficients, it is simplified to an empty polynomial.
    /// # Parameters
    /// * `rhs` - The polynomial to add to the current polynomial.
    /// # Returns
    /// A new `Polynomial` instance representing the sum of the two polynomials.
    /// If the resulting polynomial has no coefficients, it is simplified to an empty polynomial.
    /// If the degree of one of the polynomials is less than the other, it pads the coefficients with the remaining values of the higher degree polynomial.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let p2 = Polynomial::new(&[0.0, 4.0, 5.0]);
    /// let result = p1.clone() + p2;
    /// assert_eq!(result.coeff(), &[1.0, 6.0, 8.0]);
    /// let p3 = Polynomial::new(&[4.0, 5.0]);
    /// let result = p1.clone() + p3;
    /// assert_eq!(result.coeff(), &[1.0, 6.0, 8.0]);
    /// let p4 = Polynomial::empty();
    /// let result = p1.clone() + p4;
    /// assert_eq!(result.coeff(), &[1.0, 2.0, 3.0]);
    /// let p5 = Polynomial::new(&[-1.0, -2.0, -3.0]);
    /// let result = p1.clone() + p5;
    /// let expected: &[f32] = &[];
    /// assert_eq!(result.coeff(), expected);
    /// let p6 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// let result = p6.clone() + p1;
    /// assert_eq!(result.coeff(), &[1.0, 3.0, 5.0, 7.0]);
    /// ```
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
    /// Subtracts one polynomial from another.
    /// If either polynomial is empty, it returns the negation of the other polynomial.
    /// If both polynomials have coefficients, it subtracts their coefficients.
    /// If the degree of one of the polynomials is less than the other, it pads the coefficients with the remaining values of the higher degree polynomial.
    /// The resulting polynomial has coefficients that are the difference of the corresponding coefficients of the two polynomials.
    /// If the resulting polynomial has no coefficients, it is simplified to an empty polynomial.
    ///
    /// # Parameters
    /// * `rhs` - The polynomial to subtract from the current polynomial.
    /// # Returns
    /// A new `Polynomial` instance representing the difference of the two polynomials.
    /// If the resulting polynomial has no coefficients, it is simplified to an empty polynomial.
    /// If the degree of one of the polynomials is less than the other, it pads the coefficients with the remaining values of the higher degree polynomial.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let p2 = Polynomial::new(&[0.0, 4.0, 5.0]);
    /// let result = p1.clone() - p2;
    /// assert_eq!(result.coeff(), &[1.0, -2.0, -2.0]);
    /// let p3 = Polynomial::new(&[4.0, 5.0]);
    /// let result = p1.clone() - p3;
    /// assert_eq!(result.coeff(), &[1.0, -2.0, -2.0]);
    /// let p4 = Polynomial::empty();
    /// let result = p1.clone() - p4;
    /// assert_eq!(result.coeff(), &[1.0, 2.0, 3.0]);
    /// let p5 = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let result = p1.clone() - p5;
    /// let expected: &[f32] = &[];
    /// assert_eq!(result.coeff(), expected);
    /// let p6 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// let result = p1 - p6;
    /// assert_eq!(result.coeff(), &[-1.0, -1.0, -1.0, -1.0]);
    /// ```
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

    /// Multiplies two polynomials together.
    /// If either polynomial is empty, it returns an empty polynomial.
    /// If both polynomials have coefficients, it multiplies their coefficients.
    /// The resulting polynomial has coefficients that are the product of the corresponding coefficients of the two polynomials.
    /// The degree of the resulting polynomial is the sum of the degrees of the two polynomials.
    /// If the resulting polynomial has no coefficients, it is simplified to an empty polynomial.
    ///
    /// # Parameters
    /// * `rhs` - The polynomial to multiply with the current polynomial.
    /// # Returns
    /// A new `Polynomial` instance representing the product of the two polynomials.
    /// If the resulting polynomial has no coefficients, it is simplified to an empty polynomial.
    /// If the degree of either polynomial is less than 0, it returns an empty polynomial.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p1 = Polynomial::new(&[1.0, 2.0]);
    /// let p2 = Polynomial::new(&[3.0, 4.0]);
    /// let result = p1 * p2;
    /// assert_eq!(result.coeff(), &[3.0, 10.0, 8.0]);
    /// let p3 = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let p4 = Polynomial::new(&[4.0, 5.0]);
    /// let result = p3 * p4;
    /// assert_eq!(result.coeff(), &[4.0, 13.0, 22.0, 15.0]);
    /// let p5 = Polynomial::new(&[1.0, 0.0]);
    /// let p6 = Polynomial::new(&[1.0, 1.0]);
    /// let result = p5 * p6;
    /// assert_eq!(result.coeff(), &[1.0, 1.0, 0.0]);
    /// let p7 = Polynomial::empty();
    /// let p8 = Polynomial::new(&[1.0, 2.0]);
    /// let result = p7 * p8;
    /// let expected: &[f32] = &[];
    /// assert_eq!(result.coeff(), expected);
    /// ```
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

    /// Negates the polynomial by negating each of its coefficients.
    /// The resulting polynomial has coefficients that are the negation of the corresponding coefficients of the original polynomial.
    /// If the polynomial is empty, it returns an empty polynomial.
    ///
    /// # Returns
    /// A new `Polynomial` instance representing the negation of the original polynomial.
    /// If the polynomial is empty, it returns an empty polynomial.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let result = -p;
    /// assert_eq!(result.coeff(), &[-1.0, -2.0, -3.0]);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// let result = -p2;
    /// assert_eq!(result.coeff(), &[-1.0, -2.0]);
    /// let p3 = Polynomial::empty();
    /// let result = -p3;
    /// let expected: &[f32] = &[];
    /// assert_eq!(result.coeff(), expected);
    /// ```
    fn neg(self) -> Self::Output {
        Polynomial {
            coeff: self.coeff.iter().map(|&c| -c).collect(),
        }
    }
}

impl Display for Polynomial {
    /// Formats the polynomial as a human-readable string.
    /// The string representation includes the coefficients and their corresponding powers of x.
    /// For example, a polynomial `2*x^2 + 3*x + 4` would be formatted as `2*x^2 + 3*x + 4`.
    /// If the polynomial is empty, it returns an empty string.
    /// If the polynomial has a degree of 0, it returns the lead coefficient as a string.
    /// If the polynomial has a degree of 1, it returns the lead coefficient followed by `*x`.
    /// For higher degrees, it returns the lead coefficient followed by `*x^n`, where `n` is the degree of the term.
    ///
    /// # Parameters
    /// * `f` - A mutable reference to a `std::fmt::Formatter` instance used for formatting the output.
    /// # Returns
    /// A `std::fmt::Result` indicating the success or failure of the formatting operation.
    ///
    /// # Example
    /// ```
    /// use aule::poly::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let result = format!("{}", p);
    /// assert_eq!(result, "1*x^2 + 2*x + 3");
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// let result2 = format!("{}", p2);
    /// assert_eq!(result2, "1*x + 2");
    /// let p3 = Polynomial::empty();
    /// let result3 = format!("{}", p3);
    /// assert_eq!(result3, "");
    /// let p4 = Polynomial::new(&[0.0, 0.0, 0.0]);
    /// let result4 = format!("{}", p4);
    /// assert_eq!(result4, "");
    /// let p5 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// let result5 = format!("{}", p5);
    /// assert_eq!(result5, "1*x^3 + 2*x^2 + 3*x + 4");
    /// ```
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
