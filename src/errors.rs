//! Error types for polynomial operations.

use std::fmt;

/// Errors that can occur during polynomial operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PolynomialError {
    /// Division by zero polynomial
    DivisionByZero,
    /// Invalid polynomial (e.g., empty coefficients or zero leading coefficient)
    InvalidPolynomial(String),
    /// Modulus operation error
    ModulusError(String),
    /// Cyclotomic polynomial error
    CyclotomicError(String),
    /// Range check failure
    RangeCheckError(String),
    /// Arithmetic overflow or underflow
    ArithmeticError(String),
}

impl fmt::Display for PolynomialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolynomialError::DivisionByZero => write!(f, "Division by zero polynomial"),
            PolynomialError::InvalidPolynomial(msg) => write!(f, "Invalid polynomial: {}", msg),
            PolynomialError::ModulusError(msg) => write!(f, "Modulus error: {}", msg),
            PolynomialError::CyclotomicError(msg) => write!(f, "Cyclotomic polynomial error: {}", msg),
            PolynomialError::RangeCheckError(msg) => write!(f, "Range check error: {}", msg),
            PolynomialError::ArithmeticError(msg) => write!(f, "Arithmetic error: {}", msg),
        }
    }
}

impl std::error::Error for PolynomialError {}

impl From<std::io::Error> for PolynomialError {
    fn from(err: std::io::Error) -> Self {
        PolynomialError::ArithmeticError(format!("I/O error: {}", err))
    }
}

impl From<num_bigint::ParseBigIntError> for PolynomialError {
    fn from(err: num_bigint::ParseBigIntError) -> Self {
        PolynomialError::InvalidPolynomial(format!("Parse error: {}", err))
    }
}
