use bigint_poly::Polynomial;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use num_bigint::BigInt;
use num_traits::{One, Zero};

fn create_test_polynomials(degree: usize) -> (Polynomial, Polynomial) {
    let mut coeffs1 = Vec::new();
    let mut coeffs2 = Vec::new();

    for i in 0..=degree {
        coeffs1.push(BigInt::from(i as i64 + 1));
        coeffs2.push(BigInt::from((i + 1) as i64 * 2));
    }

    (Polynomial::new(coeffs1), Polynomial::new(coeffs2))
}

fn benchmark_polynomial_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_addition");

    for degree in [10, 50, 100, 500] {
        let (poly1, poly2) = create_test_polynomials(degree);

        group.bench_function(&format!("degree_{}", degree), |b| {
            b.iter(|| black_box(poly1.add(&poly2)))
        });
    }

    group.finish();
}

fn benchmark_polynomial_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_multiplication");

    for degree in [5, 10, 20, 50] {
        let (poly1, poly2) = create_test_polynomials(degree);

        group.bench_function(&format!("degree_{}", degree), |b| {
            b.iter(|| black_box(poly1.mul(&poly2)))
        });
    }

    group.finish();
}

fn benchmark_polynomial_division(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_division");

    for degree in [10, 20, 50, 100] {
        let (poly1, poly2) = create_test_polynomials(degree);

        group.bench_function(&format!("degree_{}", degree), |b| {
            b.iter(|| black_box(poly1.div(&poly2).unwrap()))
        });
    }

    group.finish();
}

fn benchmark_polynomial_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_evaluation");

    for degree in [10, 50, 100, 500] {
        let (poly1, _) = create_test_polynomials(degree);
        let x = BigInt::from(42);

        group.bench_function(&format!("degree_{}", degree), |b| {
            b.iter(|| black_box(poly1.evaluate(&x)))
        });
    }

    group.finish();
}

fn benchmark_modular_reduction(c: &mut Criterion) {
    let mut group = c.benchmark_group("modular_reduction");

    for degree in [10, 50, 100, 500] {
        let (poly1, _) = create_test_polynomials(degree);
        let modulus = BigInt::from(1000000007); // Large prime

        group.bench_function(&format!("degree_{}", degree), |b| {
            b.iter(|| black_box(poly1.reduce_and_center(&modulus)))
        });
    }

    group.finish();
}

fn benchmark_cyclotomic_reduction(c: &mut Criterion) {
    let mut group = c.benchmark_group("cyclotomic_reduction");

    // Create cyclotomic polynomial x^N + 1
    for n in [64, 128, 256, 512] {
        let mut cyclo_coeffs = vec![BigInt::zero(); n + 1];
        cyclo_coeffs[0] = BigInt::one(); // x^N
        cyclo_coeffs[n] = BigInt::one(); // + 1

        let cyclo = cyclo_coeffs;
        let (poly1, _) = create_test_polynomials(n * 2); // Polynomial of higher degree

        group.bench_function(&format!("cyclo_degree_{}", n), |b| {
            b.iter(|| black_box(poly1.reduce_by_cyclotomic(&cyclo).unwrap()))
        });
    }

    group.finish();
}

fn benchmark_utility_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("utility_functions");

    // Benchmark reduce_and_center
    let x = BigInt::from(123456789);
    let modulus = BigInt::from(1000000007);
    let half_modulus = &modulus / 2;

    group.bench_function("reduce_and_center", |b| {
        b.iter(|| {
            black_box(bigint_poly::utils::reduce_and_center(
                &x,
                &modulus,
                &half_modulus,
            ))
        })
    });

    // Benchmark range checking
    let coeffs: Vec<BigInt> = (0..1000).map(|i| BigInt::from(i)).collect();
    let bound = BigInt::from(500);

    group.bench_function("range_check_standard", |b| {
        b.iter(|| {
            black_box(bigint_poly::utils::range_check_standard(
                &coeffs, &bound, &modulus,
            ))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_polynomial_addition,
    benchmark_polynomial_multiplication,
    benchmark_polynomial_division,
    benchmark_polynomial_evaluation,
    benchmark_modular_reduction,
    benchmark_cyclotomic_reduction,
    benchmark_utility_functions
);
criterion_main!(benches);
