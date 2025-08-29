use bigint_poly::utils::*;
use num_bigint::BigInt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_and_center() {
        let modulus = BigInt::from(7);
        let half_modulus = &modulus / 2;

        // Test positive number
        assert_eq!(
            reduce_and_center(&BigInt::from(10), &modulus, &half_modulus),
            BigInt::from(3)
        );

        // Test negative number
        assert_eq!(
            reduce_and_center(&BigInt::from(-3), &modulus, &half_modulus),
            BigInt::from(-3)
        );

        // Test number greater than half modulus
        assert_eq!(
            reduce_and_center(&BigInt::from(6), &modulus, &half_modulus),
            BigInt::from(-1)
        );
    }

    #[test]
    fn test_reduce_coefficients() {
        let coeffs = vec![BigInt::from(10), BigInt::from(-3), BigInt::from(15)];
        let modulus = BigInt::from(7);
        let result = reduce_coefficients(&coeffs, &modulus);
        assert_eq!(
            result,
            vec![BigInt::from(3), BigInt::from(4), BigInt::from(1)]
        );
    }

    #[test]
    fn test_range_check_centered() {
        let vec = vec![BigInt::from(-2), BigInt::from(0), BigInt::from(2)];
        let lower = BigInt::from(-3);
        let upper = BigInt::from(3);
        assert!(range_check_centered(&vec, &lower, &upper));

        let vec_out_of_range = vec![BigInt::from(-5), BigInt::from(0), BigInt::from(2)];
        assert!(!range_check_centered(&vec_out_of_range, &lower, &upper));
    }

    #[test]
    fn test_range_check_standard() {
        let vec = vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)];
        let bound = BigInt::from(5);
        let modulus = BigInt::from(7);
        assert!(range_check_standard(&vec, &bound, &modulus));
    }

    #[test]
    fn test_reduce_and_center_coefficients() {
        let coeffs = vec![BigInt::from(10), BigInt::from(15), BigInt::from(20)];
        let modulus = BigInt::from(7);
        let result = reduce_and_center_coefficients(&coeffs, &modulus);
        assert_eq!(
            result,
            vec![BigInt::from(3), BigInt::from(1), BigInt::from(-1)]
        );
    }

    #[test]
    fn test_reduce_scalar() {
        let x = BigInt::from(-3);
        let modulus = BigInt::from(7);
        let result = reduce_scalar(&x, &modulus);
        assert_eq!(result, BigInt::from(4));
    }

    #[test]
    fn test_reduce_and_center_scalar() {
        let x = BigInt::from(6);
        let modulus = BigInt::from(7);
        let result = reduce_and_center_scalar(&x, &modulus);
        assert_eq!(result, BigInt::from(-1));
    }
}
