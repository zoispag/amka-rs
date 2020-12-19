# amka-rs

<a href="https://crates.io/crates/amka"><img src="https://img.shields.io/crates/v/amka.svg" /></a>
![CI](https://github.com/zoispag/amka-rs/workflows/CI/badge.svg)

A validator for greek social security number (AMKA)

## Usage

Add `amka` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
amka = "0.1.0"
```

Use the validator:

```rust
use crate::valid;

// An invalid AMKA
assert!(!valid("09095986680"));

// An valid AMKA
assert!(valid("09095986684"));
```
