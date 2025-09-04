use bigint_poly::{Polynomial, errors::PolynomialError};
use num_bigint::BigInt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_display() {
        let poly = Polynomial::new(vec![BigInt::from(2), BigInt::from(-3), BigInt::from(1)]);
        assert_eq!(poly.to_string(), "2x^2 - 3x + 1");
    }

    #[test]
    fn test_polynomial_addition() {
        let poly1 = Polynomial::new(vec![BigInt::from(1), BigInt::from(2)]);
        let poly2 = Polynomial::new(vec![BigInt::from(3), BigInt::from(4)]);
        let result = poly1.add(&poly2);
        assert_eq!(result.coefficients(), &[BigInt::from(4), BigInt::from(6)]);
    }

    #[test]
    fn test_polynomial_subtraction() {
        let poly1 = Polynomial::new(vec![BigInt::from(5), BigInt::from(3)]);
        let poly2 = Polynomial::new(vec![BigInt::from(2), BigInt::from(1)]);
        let result = poly1.sub(&poly2);
        assert_eq!(result.coefficients(), &[BigInt::from(3), BigInt::from(2)]);
    }

    #[test]
    fn test_polynomial_negation() {
        let poly = Polynomial::new(vec![BigInt::from(1), BigInt::from(-2), BigInt::from(3)]);
        let neg_poly = poly.neg();
        assert_eq!(
            neg_poly.coefficients(),
            &[BigInt::from(-1), BigInt::from(2), BigInt::from(-3)]
        );
    }

    #[test]
    fn test_polynomial_multiplication() {
        let poly1 = Polynomial::new(vec![BigInt::from(1), BigInt::from(2)]); // x + 2
        let poly2 = Polynomial::new(vec![BigInt::from(1), BigInt::from(3)]); // x + 3
        let result = poly1.mul(&poly2); // Should be x^2 + 5x + 6
        assert_eq!(
            result.coefficients(),
            &[BigInt::from(1), BigInt::from(5), BigInt::from(6)]
        );
    }

    #[test]
    fn test_polynomial_division() {
        let dividend = Polynomial::new(vec![BigInt::from(1), BigInt::from(5), BigInt::from(6)]); // x^2 + 5x + 6
        let divisor = Polynomial::new(vec![BigInt::from(1), BigInt::from(2)]); // x + 2
        let (quotient, remainder) = dividend.div(&divisor).unwrap();
        assert_eq!(quotient.coefficients(), &[BigInt::from(1), BigInt::from(3)]); // x + 3
        assert!(remainder.is_zero());
    }

    #[test]
    fn test_division_by_zero() {
        let poly = Polynomial::new(vec![BigInt::from(1), BigInt::from(2)]);
        let zero = Polynomial::zero(0);
        assert!(matches!(
            poly.div(&zero),
            Err(PolynomialError::DivisionByZero)
        ));
    }

    #[test]
    fn test_scalar_multiplication() {
        let poly = Polynomial::new(vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)]);
        let scalar = BigInt::from(5);
        let result = poly.scalar_mul(&scalar);
        assert_eq!(
            result.coefficients(),
            &[BigInt::from(5), BigInt::from(10), BigInt::from(15)]
        );
    }

    #[test]
    fn test_polynomial_evaluation() {
        let poly = Polynomial::new(vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)]); // x^2 + 2x + 3
        let result = poly.evaluate(&BigInt::from(2)); // 1*4 + 2*2 + 3 = 11
        assert_eq!(result, BigInt::from(11));
    }

    #[test]
    fn test_trim_leading_zeros() {
        let poly = Polynomial::new(vec![
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(1),
            BigInt::from(2),
        ]);
        let trimmed = poly.trim_leading_zeros();
        assert_eq!(trimmed.coefficients(), &[BigInt::from(1), BigInt::from(2)]);
    }

    #[test]
    fn test_ascending_coefficients_conversion() {
        // Test conversion from ascending format to Rust format
        let ascending_coeffs = vec![BigInt::from(2), BigInt::from(3), BigInt::from(1)]; // 2 + 3x + x^2
        let poly = Polynomial::from_ascending_coefficients(ascending_coeffs);
        assert_eq!(
            poly.coefficients(),
            &[BigInt::from(1), BigInt::from(3), BigInt::from(2)]
        ); // x^2 + 3x + 2

        // Test conversion back to ascending format
        let back_to_ascending = poly.to_ascending_coefficients();
        assert_eq!(
            back_to_ascending,
            vec![BigInt::from(2), BigInt::from(3), BigInt::from(1)]
        );
    }

    #[test]
    fn test_ascending_coefficients_conversion_edge_cases() {
        // Test empty polynomial
        let empty_ascending = vec![];
        let poly_empty = Polynomial::from_ascending_coefficients(empty_ascending);
        assert_eq!(poly_empty.coefficients(), &[]);
        assert_eq!(poly_empty.to_ascending_coefficients(), vec![]);

        // Test single coefficient
        let single_ascending = vec![BigInt::from(5)];
        let poly_single = Polynomial::from_ascending_coefficients(single_ascending);
        assert_eq!(poly_single.coefficients(), &[BigInt::from(5)]);
        assert_eq!(
            poly_single.to_ascending_coefficients(),
            vec![BigInt::from(5)]
        );

        // Test two coefficients
        let two_ascending = vec![BigInt::from(1), BigInt::from(2)]; // 1 + 2x
        let poly_two = Polynomial::from_ascending_coefficients(two_ascending);
        assert_eq!(poly_two.coefficients(), &[BigInt::from(2), BigInt::from(1)]); // 2x + 1
        assert_eq!(
            poly_two.to_ascending_coefficients(),
            vec![BigInt::from(1), BigInt::from(2)]
        );
    }

    #[test]
    fn test_ascending_coefficients_compatibility_example() {
        // This test demonstrates the exact scenario mentioned in the issue
        // Ascending: [2, 3, 1] represents 2 + 3x + x^2
        let ascending_coefficients = vec![BigInt::from(2), BigInt::from(3), BigInt::from(1)];
        let poly = Polynomial::from_ascending_coefficients(ascending_coefficients);

        // Rust: [1, 3, 2] represents x^2 + 3x + 2
        assert_eq!(
            poly.coefficients(),
            &[BigInt::from(1), BigInt::from(3), BigInt::from(2)]
        );
        assert_eq!(poly.to_string(), "x^2 + 3x + 2");

        // Convert back to ascending format
        let back_to_ascending = poly.to_ascending_coefficients();
        assert_eq!(
            back_to_ascending,
            vec![BigInt::from(2), BigInt::from(3), BigInt::from(1)]
        );
    }

    #[cfg(feature = "serde")]
    mod serialization_tests {
        use super::*;
        use bincode;

        #[test]
        fn test_polynomial_bincode_serialization() {
            let poly = Polynomial::new(vec![BigInt::from(2), BigInt::from(-3), BigInt::from(1)]); // 2x^2 - 3x + 1

            // Test binary serialization with bincode
            let bytes = bincode::serialize(&poly).expect("Failed to serialize");
            let reconstructed: Polynomial =
                bincode::deserialize(&bytes).expect("Failed to deserialize");

            // Verify the polynomial is correctly reconstructed
            assert_eq!(poly, reconstructed);
            assert_eq!(poly.coefficients(), reconstructed.coefficients());
            assert_eq!(poly.degree(), reconstructed.degree());
            assert_eq!(poly.to_string(), reconstructed.to_string());
        }

        #[test]
        fn test_polynomial_bincode_serialization_edge_cases() {
            // Test zero polynomial
            let zero_poly = Polynomial::zero(3);
            let bytes = bincode::serialize(&zero_poly).expect("Failed to serialize");
            let reconstructed: Polynomial =
                bincode::deserialize(&bytes).expect("Failed to deserialize");
            assert_eq!(zero_poly, reconstructed);
            assert!(reconstructed.is_zero());

            // Test constant polynomial
            let const_poly = Polynomial::constant(BigInt::from(42));
            let bytes = bincode::serialize(&const_poly).expect("Failed to serialize");
            let reconstructed: Polynomial =
                bincode::deserialize(&bytes).expect("Failed to deserialize");
            assert_eq!(const_poly, reconstructed);
            assert_eq!(reconstructed.degree(), 0);

            // Test empty polynomial
            let empty_poly = Polynomial::new(vec![]);
            let bytes = bincode::serialize(&empty_poly).expect("Failed to serialize");
            let reconstructed: Polynomial =
                bincode::deserialize(&bytes).expect("Failed to deserialize");
            assert_eq!(empty_poly, reconstructed);
        }

        #[test]
        fn test_polynomial_bincode_serialization_large_numbers() {
            // Test with very large numbers
            let large_coeffs = vec![
                BigInt::from(i64::MAX),
                BigInt::from(i64::MIN),
                BigInt::from(0),
                BigInt::from(42),
            ];
            let poly = Polynomial::new(large_coeffs);

            let bytes = bincode::serialize(&poly).expect("Failed to serialize");
            let reconstructed: Polynomial =
                bincode::deserialize(&bytes).expect("Failed to deserialize");

            assert_eq!(poly, reconstructed);
            assert_eq!(poly.coefficients(), reconstructed.coefficients());
        }

        #[test]
        fn test_polynomial_bincode_serialization_roundtrip() {
            // Test that operations work correctly after serialization/deserialization
            let poly1 = Polynomial::new(vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)]);
            let poly2 = Polynomial::new(vec![BigInt::from(4), BigInt::from(5)]);

            // Serialize and deserialize both polynomials
            let bytes1 = bincode::serialize(&poly1).expect("Failed to serialize");
            let bytes2 = bincode::serialize(&poly2).expect("Failed to serialize");

            let reconstructed1: Polynomial =
                bincode::deserialize(&bytes1).expect("Failed to deserialize");
            let reconstructed2: Polynomial =
                bincode::deserialize(&bytes2).expect("Failed to deserialize");

            // Test that operations still work
            let original_sum = poly1.add(&poly2);
            let reconstructed_sum = reconstructed1.add(&reconstructed2);
            assert_eq!(original_sum, reconstructed_sum);

            let original_product = poly1.mul(&poly2);
            let reconstructed_product = reconstructed1.mul(&reconstructed2);
            assert_eq!(original_product, reconstructed_product);
        }

        #[test]
        fn test_polynomial_bincode_serialization_ascending_conversion() {
            // Test that ascending coefficient conversion works after serialization
            let ascending_coeffs = vec![BigInt::from(2), BigInt::from(3), BigInt::from(1)];
            let poly = Polynomial::from_ascending_coefficients(ascending_coeffs.clone());

            let bytes = bincode::serialize(&poly).expect("Failed to serialize");
            let reconstructed: Polynomial =
                bincode::deserialize(&bytes).expect("Failed to deserialize");

            // Test ascending conversion still works
            let back_to_ascending = reconstructed.to_ascending_coefficients();
            assert_eq!(back_to_ascending, ascending_coeffs);
        }
    }
}
