# Roman Numerals

Library that can convert our [Base 10 Arabic Numeral System](https://en.wikipedia.org/wiki/Hindu–Arabic_numeral_system) to the [Roman Numeral System](https://en.wikipedia.org/wiki/Roman_numerals), and vice versa.

It is compatible with all unsigned integer types, from `u8` to `u128`.

## Usage

### Convert to Roman

```rust
use roman_numerals::to_roman;

assert_eq!(1u32.to_roman(), "I");
assert_eq!(2u32.to_roman(), "II");
assert_eq!(3u32.to_roman(), "III");
```

### Parse Roman

todo

## Contributing

You can start a development environment instantly on this library by [clicking this link](https://open.docker.com/dashboard/dev-envs?url=https://github.com/ferdinandkeller/roman-numerals-rs). You need Docker Desktop installed and running on your machine for it to work.