# BigInt Polynomial Library

A big integer polynomial library designed for cryptographic applications, particularly lattice-based cryptography and homomorphic encryption schemes.

[![License](https://img.shields.io/badge/license-LGPL--3.0-blue.svg)](LICENSE.md)
[![Rust Version](https://img.shields.io/badge/rust-1.86.0+-blue.svg)](https://www.rust-lang.org)

## Features

- Uses `num-bigint` for coefficient representation.
- Addition, subtraction, multiplication, division reduction modulo cyclotomic polynomials and prime moduli.
- Utilities for coefficient range validation.
- Optional serde support for polynomial serialization.

### Mathematical Background

This library implements polynomial arithmetic over the ring of integers, with support for modular reduction operations commonly used in:

- **Lattice-based cryptography**: Polynomial rings over cyclotomic fields
- **Homomorphic encryption**: BFV, BGV, and CKKS schemes
- **Zero-knowledge proofs**: Polynomial commitment schemes

### Polynomial Representation

Polynomials are represented as:
```
a_n * x^n + a_{n-1} * x^{n-1} + ... + a_1 * x + a_0
```

Where coefficients are stored in descending order (highest degree first) of degree using `BigInt` for arbitrary precision. You can rely on some methods to transform to ascending order (lowest degree first) and viceversa.

### Performance

The library is optimized for cryptographic workloads with:

- Efficient coefficient storage and manipulation
- Optimized modular reduction algorithms
- Minimal memory allocations
- Horner's method for polynomial evaluation

## Usage

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bigint-poly = { git = "https://github.com/gnosisguild/bigint-poly" }
```

For serialization support, enable the `serde` feature:

```toml
[dependencies]
bigint-poly = { git = "https://github.com/gnosisguild/bigint-poly", features = ["serde"] }
```

### Testing

Run the test suite:

```bash
cargo test
```

Run tests with verbose output:

```bash
cargo test -- --nocapture
```

### Benchmarks

Run benchmarks:

```bash
cargo bench
```

### Quick Start

```rust
use bigint_poly::{Polynomial, BigInt};

// Create polynomials
let poly1 = Polynomial::new(vec![BigInt::from(1), BigInt::from(2), BigInt::from(3)]);
let poly2 = Polynomial::new(vec![BigInt::from(1), BigInt::from(1)]);

// Perform arithmetic
let sum = poly1.add(&poly2);
let product = poly1.mul(&poly2);

// Modular reduction
let modulus = BigInt::from(7);
let reduced = poly1.reduce_and_center(&modulus);

println!("Sum: {}", sum);
println!("Product: {}", product);
println!("Reduced: {}", reduced);
```

## License

This project is licensed under the LGPL-3.0 License - see the [LICENSE](LICENSE.md) file for details.
