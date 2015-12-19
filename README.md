# RsGenetic
[![Build Status](https://travis-ci.org/m-decoster/RsGenetic.svg?branch=master)](https://travis-ci.org/m-decoster/RsGenetic)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)

## Summary and Features
RsGenetic is a simple framework for executing a genetic algorithm in Rust.  
This library is not yet feature-complete. It currently offers a sequential implementation that
can run genetic algorithms with a limited set of selection algorithms. The goal is
to provide at least one parallel implementation, to optimize these implementations,
and to provide more selection algorithms if needed.

## Examples and Documentation
Documentation is available [here](http://m-decoster.github.io/RsGenetic).  
Examples are available as Cargo projects in the `examples` directory. Simply `cd` into
an example directory and execute `cargo run` to test the library.  

## License
This library is available under Apache 2.0.

## Contributing
Contributions are always welcome. Take a look at the issues for any enhancements that need to be
done or bugs that need to be fixed. If you encounter any bugs while using the library, feel free to
open an issue and/or fix the bug, and submit pull requests.

## Notes
This library has only been tested with Rust stable 1.5.0.
