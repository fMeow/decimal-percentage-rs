<!-- cargo-sync-readme start -->

# Percentage Type with Decimal

[![Build Status](https://github.com/fMeow/decimal-percentage-rs/workflows/Rust/badge.svg)](https://github.com/fMeow/decimal-percentage-rs/actions)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/decimal-percentage.svg)](https://crates.io/crates/decimal-percentage)
[![decimal-percentage](https://docs.rs/decimal-percentage/badge.svg)](https://docs.rs/decimal-percentage)

A type to represent percentage with high precision
thanks to [`rust_decimal`](https://docs.rs/rust_decimal/latest/rust_decimal/).

A percentage can perform addition, subtraction and multiplication.

```rust
use decimal_percentage::Percentage;
use rust_decimal::Decimal;

let p1 = Percentage::from(0.1f64);
let p2 = Percentage::from(0.1f32);
let p3 = Percentage::try_from("0.1").unwrap();
let p4 = Percentage::from(Decimal::from_f64(0.3).unwrap());

assert_eq!(p1 + p2, Percentage::from(0.2));
assert_eq!(p1 + 0.2, Percentage::from(0.3));
assert_eq!(p4 - p2, Percentage::from(0.2));
assert_eq!(p1 * 66.0, 6.6);
assert_eq!(p1 * 100u32, 10u32);
assert_eq!(p1 * -100i32, -10i32);
// note that a multiplication to integer type can lose precision
assert_eq!(p1 * -33i32, -3i32);
// multiplication on extremely small value with Decimal,
// that is not representable with float point
let small_value = Decimal::from_str("0.0000000000000000002").unwrap();
assert_eq!(p1 * small_value, Decimal::from_str("0.00000000000000000002").unwrap());
```

## Contributing
Contributions and feed back are welcome following Github workflow.

## License
`decimal_percentage` is provided under the MIT license. See [LICENSE](./LICENSE).

<!-- cargo-sync-readme end -->
