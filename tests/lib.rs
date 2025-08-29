use bigint_poly::Polynomial;
use num_bigint::BigInt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_polynomial_creation() {
        let poly = Polynomial::new(vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)]);
        assert_eq!(poly.degree(), 2);
        assert_eq!(poly.coefficients(), &[BigInt::from(1), BigInt::from(2), BigInt::from(3)]);
    }

    #[test]
    fn test_zero_polynomial() {
        let zero = Polynomial::zero(3);
        assert_eq!(zero.degree(), 3);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_constant_polynomial() {
        let const_poly = Polynomial::constant(BigInt::from(42));
        assert_eq!(const_poly.degree(), 0);
        assert_eq!(const_poly.coefficients(), &[BigInt::from(42)]);
    }
}
