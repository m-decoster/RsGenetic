// file: lib.rs
//
// Copyright 2015-2017 The RsGenetic Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # `RsGenetic`
//!
//! `RsGenetic` provides a simple framework for genetic algorithms.
//! You need to provide the definition of a Phenotype (also known as an Individual),
//! define how crossover and mutation work, present a fitness function, choose some settings
//! and this library takes care of the rest.
//!
//! # Installation
//!
//! You can use this library by adding the following lines to your `Cargo.toml` file:
//!
//! ```ignore
//! [dependencies]
//! rsgenetic = "^1.7.0"
//! ```
//!
//! and adding `extern crate rsgenetic;` to your crate root.
//!
//! # Features
//! ## Available Simulators
//!
//! There is currently only one, sequential, simulator. This simulator will run
//! the genetic algorithm on a single thread.
//!
//! ## Available Selection Types
//!
//! There are currently four selection types available:
//!
//! * Maximize
//! * Tournament
//! * Stochastic
//!
//! There is a short explanation for each of these below. For more information, look at the
//! documentation of individual selectors.
//!
//! ### Maximize
//!
//! Maximize takes 1 parameter: the count. This is half the number of parents
//! that will be selected. Selection happens by taking the top `count` individuals,
//! ranked by fitness. The resulting number of parents is `count`.
//!
//! ### Tournament
//!
//! Tournament takes 2 parameters: the number of tournaments (`count`) and `participators`,
//! which indicates how many phenotypes participate in a tournament.
//! The resulting number of parents is `count`.
//!
//! ### Stochastic
//!
//! Stochastic takes 1 parameter: the count. The resulting number of parents is `count`.
//!
//! ## Early Stopping
//!
//! If you wish, you can stop early if the fitness value of the best performing Phenotype
//! doesn't improve by a large amount for a number of iterations. This can be done by calling the
//! `set_early_stop(delta: Fitness, n_iters: u32)` function on the `SimulatorBuilder`.
//!
//! # Examples
//!
//! ## Implementing the `Fitness` trait
//!
//! Note that, if your fitness type is an integer type, you
//! do not need to write a wrapper struct around this integer. See
//! the `types` module documentation for more details.
//!
//! ```
//! use rsgenetic::pheno::*;
//! use std::cmp::Ordering;
//!
//! #[derive(Eq, PartialEq, PartialOrd, Ord)]
//! struct MyFitness {
//!     value: i32,
//! }
//!
//! impl Fitness for MyFitness {
//!     // The zero value for our custom type
//!     fn zero() -> MyFitness {
//!         MyFitness { value: 0 }
//!     }
//!
//!     // The absolute difference between two instances
//!     fn abs_diff(&self, other: &MyFitness) -> MyFitness {
//!         MyFitness {
//!             value: (self.value - other.value).abs()
//!         }
//!     }
//! }
//! ```
//!
//! ## Implementing the `Phenotype` trait
//!
//! Note that we use an integer type as the fitness type parameter
//! to make this example more simple. Replace it with your custom type
//! if needed. In this example, we try to find individuals with
//! two integer components that sum to a target value.
//!
//! This example is far-fetched, but simplified to show how
//! easy it is to define new individuals and implement
//! the `Phenotype` trait.
//!
//! ```
//! use rsgenetic::pheno::*;
//!
//! const TARGET: i32 = 100;
//!
//! #[derive(Copy, Clone)]
//! struct MyPheno {
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl Phenotype<i32> for MyPheno {
//!     // How fit is this individual?
//!     fn fitness(&self) -> i32 {
//!         TARGET - (self.x + self.y)
//!     }
//!
//!     // Have two individuals create a new individual
//!     fn crossover(&self, other: &MyPheno) -> MyPheno {
//!         MyPheno {
//!             x: self.x,
//!             y: other.y,
//!         }
//!     }
//!
//!     // Mutate an individual, changing its state
//!     fn mutate(&self) -> MyPheno {
//!         MyPheno {
//!             x: self.x + 1,
//!             y: self.y - 1,
//!         }
//!     }
//! }
//! ```
//!
//! ## Creating and running a `Simulator`
//!
//! ```ignore
//!
//! use rsgenetic::sim::*;
//! use rsgenetic::sim::seq::Simulator;
//! use rsgenetic::sim::select::*;
//!
//! // (Assuming the above definition of `MyPheno` is in scope)
//! // [ ... ]
//!
//! fn main() {
//!     let mut population = (0..100).map(|i| MyPheno { x: i, y: 100 - i }).collect();
//!     let mut s = Simulator::builder(&mut population)
//!                     .set_selector(Box::new(StochasticSelector::new(10)))
//!                     .set_max_iters(50)
//!                     .build();
//!     s.run();
//!     let result = s.get().unwrap(); // The best individual
//! }
//! ```
//!
//! See the `examples` directory in the repository for more elaborate examples.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate rand;

/// Contains the definition of a Phenotype.
pub mod pheno;

/// Contains a way to collect information on steps via a trait.
pub mod stats;

/// Contains implementations of Simulators, which can run genetic algorithms.
pub mod sim;
/// Contains code used by unit tests.
#[cfg(test)]
mod test;
