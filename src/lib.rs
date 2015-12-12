//! # RsGenetic
//!
//! RsGenetic provides a simple framework for genetic algorithms.
//! You need to provide the definition of a Phenotype (also known as an Individual),
//! define how crossover and mutation work, present a fitness function, choose some settings
//! and this library takes care of the rest.
//!
//! # Example usage:
//!
//! ```ignore
//! use std::cmp;
//! extern crate rsgenetic;
//! use rsgenetic::{pheno, sim};
//!
//! // Define the structure of your Phenotype
//! struct Test {
//!     i: i32,
//! }
//!
//! // Implement the Phenotype trait.
//! impl pheno::Phenotype for Test {
//!     fn fitness(&self) -> f64 {
//!         (self.i - 0).abs() as f64
//!     }
//!
//!     fn crossover(&self, t: &Test) -> Test {
//!         Test { i: cmp::min(self.i, t.i) }
//!     }
//!
//!     fn mutate(&self) -> Self {
//!         if self.i < 0 {
//!             Test { i: self.i + 1 }
//!         } else {
//!             Test { i: self.i - 1}
//!         }
//!     }
//! }
//!
//! // Implement the Clone trait.
//! impl Clone for Test {
//!     fn clone(&self) -> Self {
//!         Test { i: self.i }
//!     }
//! }
//!
//! // Generate a random population.
//! let mut tests: Vec<Box<Test>> = Vec::new();
//! for i in 0..100 {
//!     tests.push(Box::new(Test { i: i + 10 }));
//! }
//! // Create a simulator.
//! let mut s = sim::seq::Simulator::new(tests, // Population
//!                                      100, // Iterations
//!                                      sim::seq::SelectionType::Tournament{
//!                                         size: 3,
//!                                         count:5}, // Tournament Selection
//!                                      sim::seq::FitnessType::Minimize); // Minimize the fitness
//! s.run();
//! assert!((*s.get()).i == 0); // For this simple example, we should always get 0.

extern crate rand;

/// Contains the definition of a Phenotype.
pub mod pheno;
/// Contains implementations of Simulators, which can run genetic algorithms.
pub mod sim;
