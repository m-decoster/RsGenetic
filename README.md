# RsGenetic
[![Build Status](https://travis-ci.org/m-decoster/RsGenetic.svg?branch=master)](https://travis-ci.org/m-decoster/RsGenetic)
[![Crates Version](https://img.shields.io/crates/v/rsgenetic.svg)](https://crates.io/crates/rsgenetic/)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![License Apache](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)

## Summary and Features
RsGenetic is a framework for executing genetic algorithms in Rust. It is designed to have a simple but modular API.

## Examples and Documentation
Documentation is available [here](https://docs.rs/rsgenetic).  

### Implementing the `Fitness` trait

Note that, if your fitness type is an integer type, you
do not need to write a wrapper struct around this integer. See
the `types` module documentation for more details.

```rust
use rsgenetic::pheno::*;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct MyFitness {
    value: i32,
}

impl Fitness for MyFitness {
    // The zero value for our custom type
    fn zero() -> MyFitness {
        MyFitness { value: 0 }
    }

    // The absolute difference between two instances
    fn abs_diff(&self, other: &MyFitness) -> MyFitness {
        MyFitness {
            value: (self.value - other.value).abs()
        }
    }
}
```

### Implementing the `Phenotype` trait

Note that we use an integer type as the fitness type parameter
to make this example more simple. Replace it with your custom type
if needed. In this example, we try to find individuals with
two integer components that sum to a target value.

This example is far-fetched, but simplified to show how
easy it is to define new individuals and implement
the `Phenotype` trait.

```rust
use rsgenetic::pheno::*;

const TARGET: i32 = 100;

#[derive(Copy, Clone)]
struct MyPheno {
    x: i32,
    y: i32,
}

impl Phenotype<i32> for MyPheno {
    // How fit is this individual?
    fn fitness(&self) -> i32 {
        TARGET - (self.x + self.y)
    }

    // Have two individuals create a new individual
    fn crossover(&self, other: &MyPheno) -> MyPheno {
        MyPheno {
            x: self.x,
            y: other.y,
        }
    }

    // Mutate an individual, changing its state
    fn mutate(&self) -> MyPheno {
        MyPheno {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
}
```

### Creating and running a `Simulator`

```rust
use rsgenetic::pheno::*;
use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;

const TARGET: i32 = 100;

#[derive(Copy, Clone)]
struct MyPheno {
    x: i32,
    y: i32,
}

impl Phenotype<i32> for MyPheno {
    // How fit is this individual?
    fn fitness(&self) -> i32 {
        TARGET - (self.x + self.y)
    }

    // Have two individuals create a new individual
    fn crossover(&self, other: &MyPheno) -> MyPheno {
        MyPheno {
            x: self.x,
            y: other.y,
        }
    }

    // Mutate an individual, changing its state
    fn mutate(&self) -> MyPheno {
        MyPheno {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
}

fn main() {
    let mut population = (0..100).map(|i| MyPheno { x: i, y: 100 - i }).collect();
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(StochasticSelector::new(10)))
                    .set_max_iters(50)
                    .build();
    s.run();
    let result = s.get().unwrap(); // The best individual
}
```

See the `examples` directory in the repository for more elaborate examples.

## Note

This library is currently in maintenance mode. There have been some indications that the API
needs an update to be more flexible, which would require an incrementation of the major version number (#23, #30).
Unfortunately, I currently do not have the time to implement such a redesign. I will however continue to reply to issues
and merge pull requests, but features might not be implemented by me, depending on their size.

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
