# amka-rs

<a href="https://crates.io/crates/amka"><img src="https://img.shields.io/crates/v/amka.svg" /></a>
![CI](https://github.com/zoispag/amka-rs/workflows/CI/badge.svg)

Un validador para el número de seguridad social griego (AMKA)

## Uso

Agrega `amka` bajo `[dependencies]` en tu `Cargo.toml`:

```toml
[dependencies]
amka = "1.1.0"
```

Usa el validador:

```rust
use amka;

// Un AMKA inválido
let (is_valid, err) = amka::validate("09095986680");
assert!(!is_valid);
println!("{}", err);

// Un AMKA válido
let (is_valid, err) = amka::validate("09095986684");
assert!(is_valid);
assert_eq!("", err)
```
