# amka-rs

<a href="https://crates.io/crates/amka"><img src="https://img.shields.io/crates/v/amka.svg" /></a>
![CI](https://github.com/zoispag/amka-rs/workflows/CI/badge.svg)

A validator for greek social security number (AMKA)

## Usage

Add `amka` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
amka = "1.1.0"
```

Use the validator:

```rust
use amka;

// An invalid AMKA
let (is_valid, err) = amka::validate("09095986680");
assert!(!is_valid);
println!("{}", err);

// An valid AMKA
let (is_valid, err) = amka::validate("09095986684");
assert!(is_valid);
assert_eq!("", err)
```
