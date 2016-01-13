// file: lib.rs
//
// Copyright 2015-2016 The RsGenetic Developers
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

//! # RsGenetic
//!
//! RsGenetic provides a simple framework for genetic algorithms.
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
//! rsgenetic = "0.10"
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
//! * Roulette
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
//! Tournament takes 2 parameters: the number of tournaments (`count`) and `participators`, which indicates how
//! many phenotypes participate in a tournament. The resulting number of parents is `count`.
//!
//! ### Stochastic
//!
//! Stochastic takes 1 parameter: the count. The resulting number of parents is `count`.
//!
//! ### Roulette
//!
//! Roulette takes 1 parameter: the count. The resulting number of parents is `count`.
//!
//! ## Early Stopping
//!
//! If you wish, you can stop early if the fitness value of the best performing Phenotype
//! doesn't improve by a large amount for a number of iterations. This can be done by calling the
//! `set_early_stop(delta: f64, n_iters: u32)` function on the `SimulatorBuilder`.
//!
//! # Examples
//!
//! ## Implementing Phenotype
//!
//! ```ignore
//! // Define the structure of your Phenotype
//! #[derive(Clone)]
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
//! ```
//!
//! ## Running a Simulation
//!
//! ```ignore
//! // Generate a random population.
//! let mut tests: Vec<Box<Test>> = Vec::new();
//! for i in 0..100 {
//!     tests.push(Box::new(Test { i: i + 10 }));
//! }
//! // Create a simulator using a builder.
//! let mut s = *seq::Simulator::builder(tests, Box::new(sim::select::TournamentSelector::new(4, 4)))
//!                   .set_max_iters(1000)
//!                   .set_fitness_type(sim::FitnessType::Minimize)
//!                   .build();
//! // We can now run the simulator.
//! s.run();
//! // This will fail if the result was an error:
//! let best = s.get().unwrap();
//! // For this simple example, we should always get 0.
//! assert!((*best).i == 0);
//! // We can also get the time spent running:
//! let time = match s.time() {
//!     Some(x) => x, // Contains the time in ns
//!     None    => -1 // Overflow occured
//! };
//! ```

#![warn(missing_docs)]

extern crate rand;
extern crate time;

/// Contains the definition of a Phenotype.
pub mod pheno;
/// Contains implementations of Simulators, which can run genetic algorithms.
pub mod sim;
