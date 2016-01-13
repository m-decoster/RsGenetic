# RsGenetic
[![Build Status](https://travis-ci.org/m-decoster/RsGenetic.svg?branch=master)](https://travis-ci.org/m-decoster/RsGenetic)
[![Crates Version](https://img.shields.io/crates/v/rsgenetic.svg)](https://crates.io/crates/rsgenetic/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)

## Summary and Features
RsGenetic is a simple framework for executing a genetic algorithm in Rust.  
This library is not yet feature-complete. It currently offers a sequential implementation that
can run genetic algorithms with a limited set of selection algorithms. The goal is
to provide at least one parallel implementation, to optimize these implementations,
and to provide more selection algorithms if needed.

## Examples and Documentation
Documentation is available [here](http://m-decoster.github.io/RsGenetic).  
Examples are available in the `examples` directory. Execute `cargo run --example $EXAMPLE_NAME`
to run an example.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Contributions are always welcome. Take a look at the issues for any enhancements that need to be
done or bugs that need to be fixed. If you encounter any bugs while using the library, feel free to
open an issue and/or fix the bug, and submit pull requests.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
