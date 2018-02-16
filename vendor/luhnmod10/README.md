# Luhn (Mod 10)

[![Build Status](https://travis-ci.org/luhnmod10/rust.svg?branch=master)](https://travis-ci.org/luhnmod10/rust)

A fast and simple in-place implementation of the [luhn check algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) in Rust. Implementations in other languages can be found at [github.com/luhnmod10](https://github.com/luhnmod10).

## Usage

Add the crate to your `Cargo.toml`:

```
[dependencies]
luhnmod10 = "1.0.0"
```

Then:

```rust
extern crate luhnmod10;
luhnmod10::valid(number);
```

## Contributing

Contributions are welcome! If you can improve the execution time of this implementation without increasing its complexity, please open a pull request. To test your change, run `make` in the repository to run the tests and the benchmarks.
