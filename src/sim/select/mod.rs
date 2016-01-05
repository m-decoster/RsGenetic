//! The selection module provides a trait that can be implemented
//! to implement new selection algorithms. This module also provides a couple
//! of useful selection algorithms.

mod max;
mod tournament;
mod stochastic;
mod roulette;

use pheno::Phenotype;
use super::FitnessType;

pub use self::max::maximize_selector;
pub use self::tournament::tournament_selector;
pub use self::stochastic::stochastic_selector;
pub use self::roulette::roulette_selector;

/// `Parents` come in a `Vec` of two `Box<T>`'s.
pub type Parents<T> = Vec<(Box<T>, Box<T>)>;

/// A `Selector` can select `Parents` for a new iteration of a `Simulation`.
pub trait Selector<T: Phenotype> {
    /// Select elements from a `population`, either maximizing or minimizing the fitness
    /// (`fitness_type`).
    ///
    /// If invalid parameters are supplied or the algorithm fails, this function returns an
    /// `Err(String)`, containing a message indicating the error.
    ///
    /// Otherwise it contains a vector of parent pairs wrapped in `Ok`.
    fn select(&self, population: &Vec<Box<T>>, fitness_type: FitnessType) -> Result<Parents<T>, String>;
}
