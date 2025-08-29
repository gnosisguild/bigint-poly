//! Polynomial arithmetic implementation.

use crate::errors::PolynomialError;
use crate::utils::reduce_and_center;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A polynomial represented by its coefficients in descending order of degree.
///
/// The coefficients are stored as `BigInt` to support arbitrary precision arithmetic
/// required for cryptographic operations. The polynomial is represented as:
/// `a_n * x^n + a_{n-1} * x^{n-1} + ... + a_1 * x + a_0`
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Polynomial {
    /// Coefficients in descending order (highest degree first).
    pub(crate) coefficients: Vec<BigInt>,
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coefficients.is_empty() {
            return write!(f, "0");
        }

        let mut first = true;
        for (i, coeff) in self.coefficients.iter().enumerate() {
            let degree = self.coefficients.len() - 1 - i;

            if coeff.is_zero() {
                continue;
            }

            if !first {
                if coeff > &BigInt::zero() {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            first = false;

            let abs_coeff = if coeff < &BigInt::zero() {
                -coeff
            } else {
                coeff.clone()
            };

            if degree == 0 || !abs_coeff.is_one() {
                write!(f, "{abs_coeff}")?;
            }

            if degree > 0 {
                write!(f, "x")?;
                if degree > 1 {
                    write!(f, "^{degree}")?;
                }
            }
        }

        if first {
            write!(f, "0")?;
        }

        Ok(())
    }
}

impl Polynomial {
    /// Creates a new polynomial from a vector of coefficients.
    ///
    /// # Arguments
    ///
    /// * `coefficients` - Vector of coefficients in descending order of degree.
    pub fn new(coefficients: Vec<BigInt>) -> Self {
        Self { coefficients }
    }

    /// Creates a polynomial from coefficients in ascending order format.
    ///
    /// This method converts from ascending order coefficient ordering (lowest degree first)
    /// to this library's ordering (highest degree first).
    ///
    /// # Arguments
    ///
    /// * `ascending_coefficients` - Vector of coefficients in ascending order.
    pub fn from_ascending_coefficients(ascending_coefficients: Vec<BigInt>) -> Self {
        let mut coefficients = ascending_coefficients;
        coefficients.reverse();
        Self { coefficients }
    }

    /// Converts the polynomial to ascending order coefficient format.
    ///
    /// This method converts from this library's ordering (highest degree first)
    /// to ascending order (lowest degree first).
    ///
    /// # Returns
    ///
    /// Vector of coefficients in ascending order.
    pub fn to_ascending_coefficients(&self) -> Vec<BigInt> {
        let mut coefficients = self.coefficients.clone();
        coefficients.reverse();
        coefficients
    }

    /// Creates a zero polynomial of specified degree.
    ///
    /// # Arguments
    ///
    /// * `degree` - The degree of the zero polynomial.
    pub fn zero(degree: usize) -> Self {
        Self {
            coefficients: vec![BigInt::zero(); degree + 1],
        }
    }

    /// Creates a constant polynomial.
    ///
    /// # Arguments
    ///
    /// * `constant` - The constant value.
    pub fn constant(constant: BigInt) -> Self {
        Self {
            coefficients: vec![constant],
        }
    }

    /// Returns the coefficients of the polynomial.
    pub fn coefficients(&self) -> &[BigInt] {
        &self.coefficients
    }

    /// Returns the degree of the polynomial.
    ///
    /// The degree of a zero polynomial is 0.
    pub fn degree(&self) -> usize {
        if self.coefficients.is_empty() {
            0
        } else {
            self.coefficients.len() - 1
        }
    }

    /// Checks if the polynomial is zero.
    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|c| c.is_zero())
    }

    /// Removes leading zero coefficients from the polynomial.
    pub fn trim_leading_zeros(mut self) -> Self {
        while self.coefficients.len() > 1 && self.coefficients[0].is_zero() {
            self.coefficients.remove(0);
        }
        self
    }

    /// Returns the leading coefficient of the polynomial.
    pub fn leading_coefficient(&self) -> Option<&BigInt> {
        self.coefficients.first()
    }

    /// Adds two polynomials together.
    ///
    /// This function performs polynomial addition by:
    /// 1. Finding the maximum length between the two polynomials.
    /// 2. Creating a new polynomial with the maximum length.
    /// 3. Adding the coefficients of both polynomials term by term.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the polynomial to add to `self`.
    ///
    /// # Returns
    ///
    /// A new polynomial containing the sum of the two polynomials.
    pub fn add(&self, other: &Self) -> Self {
        let max_length = std::cmp::max(self.coefficients.len(), other.coefficients.len());
        let mut result = vec![BigInt::zero(); max_length];

        // Copy coefficients from the first polynomial
        for (i, coeff) in self.coefficients.iter().enumerate() {
            result[max_length - self.coefficients.len() + i] = coeff.clone();
        }

        // Add coefficients from the second polynomial
        for (i, coeff) in other.coefficients.iter().enumerate() {
            result[max_length - other.coefficients.len() + i] += coeff;
        }

        Polynomial::new(result)
    }

    /// Subtracts one polynomial from another.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the polynomial to subtract from `self`.
    ///
    /// # Returns
    ///
    /// A new polynomial containing the difference.
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// Negates all coefficients of the polynomial.
    ///
    /// # Returns
    ///
    /// A new polynomial with all coefficients negated.
    pub fn neg(&self) -> Self {
        Polynomial::new(self.coefficients.iter().map(|x| -x).collect())
    }

    /// Multiplies two polynomials using the naive algorithm.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the polynomial to multiply with `self`.
    ///
    /// # Returns
    ///
    /// A new polynomial containing the product.
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Polynomial::zero(0);
        }

        let product_len = self.coefficients.len() + other.coefficients.len() - 1;
        let mut product = vec![BigInt::zero(); product_len];

        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                product[i + j] += &self.coefficients[i] * &other.coefficients[j];
            }
        }

        Polynomial::new(product)
    }

    /// Divides one polynomial by another, returning the quotient and remainder.
    ///
    /// # Arguments
    ///
    /// * `divisor` - A reference to the divisor polynomial.
    ///
    /// # Returns
    ///
    /// A result containing a tuple of (quotient, remainder) or an error.
    ///
    /// # Errors
    ///
    /// Returns `PolynomialError::DivisionByZero` if the divisor is zero.
    /// Returns `PolynomialError::InvalidPolynomial` if the divisor has zero leading coefficient.
    pub fn div(&self, divisor: &Self) -> Result<(Self, Self), PolynomialError> {
        if divisor.is_zero() {
            return Err(PolynomialError::DivisionByZero);
        }

        if divisor.coefficients.is_empty() || divisor.coefficients[0].is_zero() {
            return Err(PolynomialError::InvalidPolynomial(
                "Leading coefficient of divisor cannot be zero".to_string(),
            ));
        }

        if self.degree() < divisor.degree() {
            return Ok((Polynomial::zero(0), self.clone()));
        }

        let mut quotient =
            vec![BigInt::zero(); self.coefficients.len() - divisor.coefficients.len() + 1];
        let mut remainder = self.coefficients.clone();

        for i in 0..quotient.len() {
            if i >= remainder.len() {
                break;
            }
            let coeff = &remainder[i] / &divisor.coefficients[0];
            quotient[i] = coeff.clone();

            for j in 0..divisor.coefficients.len() {
                if i + j < remainder.len() {
                    remainder[i + j] = &remainder[i + j] - &divisor.coefficients[j] * &coeff;
                }
            }
        }

        // Remove leading zero coefficients from remainder
        while !remainder.is_empty() && remainder[0].is_zero() {
            remainder.remove(0);
        }

        Ok((Polynomial::new(quotient), Polynomial::new(remainder)))
    }

    /// Multiplies each coefficient of the polynomial by a scalar.
    ///
    /// # Arguments
    ///
    /// * `scalar` - A `BigInt` scalar to multiply with each coefficient.
    ///
    /// # Returns
    ///
    /// A new polynomial with each coefficient multiplied by the scalar.
    pub fn scalar_mul(&self, scalar: &BigInt) -> Self {
        Polynomial::new(self.coefficients.iter().map(|x| x * scalar).collect())
    }

    /// Reduces the polynomial modulo a cyclotomic polynomial.
    ///
    /// This function performs polynomial division by the cyclotomic polynomial
    /// and returns the remainder.
    ///
    /// # Arguments
    ///
    /// * `cyclo` - Coefficients of the cyclotomic polynomial.
    ///
    /// # Returns
    ///
    /// A new polynomial representing the remainder after reduction.
    pub fn reduce_by_cyclotomic(&self, cyclo: &[BigInt]) -> Result<Self, PolynomialError> {
        let cyclo_poly = Polynomial::new(cyclo.to_vec());
        let (_, remainder) = self.div(&cyclo_poly)?;

        let n = cyclo.len() - 1;
        let mut out = vec![BigInt::zero(); n];

        if !remainder.coefficients.is_empty() {
            let start_idx = n.saturating_sub(remainder.coefficients.len());
            let end_idx = std::cmp::min(start_idx + remainder.coefficients.len(), n);
            out[start_idx..end_idx]
                .clone_from_slice(&remainder.coefficients[..end_idx - start_idx]);
        }

        Ok(Polynomial::new(out))
    }

    /// Reduces coefficients modulo a prime and centers them.
    ///
    /// # Arguments
    ///
    /// * `modulus` - The prime modulus.
    ///
    /// # Returns
    ///
    /// A new polynomial with coefficients reduced and centered.            
    pub fn reduce_and_center(&self, modulus: &BigInt) -> Self {
        let half_modulus = modulus / 2;
        let reduced_coeffs = self
            .coefficients
            .iter()
            .map(|x| reduce_and_center(x, modulus, &half_modulus))
            .collect();
        Polynomial::new(reduced_coeffs)
    }

    /// Evaluates the polynomial at a given point using Horner's method.
    ///
    /// # Arguments
    ///
    /// * `x` - The point at which to evaluate the polynomial.
    ///
    /// # Returns
    ///
    /// The value of the polynomial at the given point.
    pub fn evaluate(&self, x: &BigInt) -> BigInt {
        if self.coefficients.is_empty() {
            return BigInt::zero();
        }

        // Use Horner's method for efficient evaluation
        let mut result = self.coefficients[0].clone();
        for coeff in &self.coefficients[1..] {
            result = result * x + coeff;
        }
        result
    }
}
