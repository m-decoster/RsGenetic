//! # RsGenetic
//!
//! RsGenetic provides a simple framework for genetic algorithms.
//! You need to provide the definition of a Phenotype (also known as an Individual),
//! define how crossover and mutation work, present a fitness function, choose some settings
//! and this library takes care of the rest.
//!
//! # Features
//! ## Available Simulators
//!
//! There is currently only one, sequential, simulator. This simulator will run
//! the genetic algorithm on a single thread.
//!
//! ## Available Selection Types
//!
//! There are currently three selection types available:
//! 
//! * Maximize
//! * Tournament
//! * Stochastic
//!
//! There is a short explanation for each of these below. Currently, the number of parents
//! may vary depending on the chosen selection type. We wish to make this uniform some time in
//! the future.
//!
//! ### Maximize
//!
//! Maximize takes 1 parameter: the count. This is half the number of parents
//! that will be selected. Selection happens by taking the top `count * 2` individuals,
//! ranked by fitness. The resulting number of parents is `count * 2`.
//!
//! ### Tournament
//!
//! Tournament takes 2 parameters: the number of tournaments and the count. The count indicates how
//! many phenotypes participate in a tournament. The resulting number of parents is `num * 2`.
//!
//! ### Stochastic
//!
//! Stochastic takes 1 parameter: the count. The resulting number of parents is `count`.
//!
//! # Examples
//!
//! ## Implementing Phenotype
//!
//! ```ignore
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
//! // This is required for the internal workings of the library.
//! impl Clone for Test {
//!     fn clone(&self) -> Self {
//!         Test { i: self.i }
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
//! let mut s = *seq::Simulator::builder(tests) // Population is mandatory
//!                   .set_max_iters(1000)
//!                   .set_selection_type(sim::SelectionType::Tournament {
//!                         count: 3,
//!                         num: 5
//!                   })
//!                   .set_fitness_type(sim::FitnessType::Minimize)
//!                   .build();
//! // We can now run the simulator.
//! s.run();
//! assert!((*s.get()).i == 0); // For this simple example, we should always get 0.
//! ```

extern crate rand;

/// Contains the definition of a Phenotype.
pub mod pheno;
/// Contains implementations of Simulators, which can run genetic algorithms.
pub mod sim;
