# Roman Numerals

This library can help you convert numbers in our [Base 10 Arabic Numeral System](https://en.wikipedia.org/wiki/Hindu–Arabic_numeral_system) to [Roman Numeral System](https://en.wikipedia.org/wiki/Roman_numerals), and vice versa.

It is compatible with all unsigned integer types, from `u8` to `u128`, including `usize`.

## Installation

Add `roman_numerals` to your `Cargo.toml`, or run the following command:

```bash
cargo add roman-numerals
```

The library is available on [crates.io](https://crates.io/crates/roman_numerals).

## Usage

### Convert to Roman

```rust
use roman_numerals::ToRoman;

assert_eq!(1u32.to_roman(), "I");
assert_eq!(2u32.to_roman(), "II");
assert_eq!(3u32.to_roman(), "III");
```

### Parse Roman

```rust
use roman_numerals::FromRoman;

assert_eq!(u32::from_roman("ABC"), None);
assert_eq!(u32::from_roman("I"), Some(1));
assert_eq!(u32::from_roman("II"), Some(2));
assert_eq!(u32::from_roman("III"), Some(3));
```

## Contributing

You can start a development environment instantly on this library by [clicking this link](https://open.docker.com/dashboard/dev-envs?url=https://github.com/ferdinandkeller/roman-numerals-rs). You need Docker Desktop installed and running on your machine for it to work.