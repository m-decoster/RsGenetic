//! The selection module provides a trait that can be implemented
//! to implement new selection algorithms. This module also provides a couple
//! of useful selection algorithms.

use pheno::Phenotype;

/// `Parents` come in a `Vec` of two `Box<T>`'s.
pub type Parents<T> = Vec<(Box<T>, Box<T>)>;

/// A `Selector` can select `Parents` for a new iteration of a `Simulation`.
pub trait Selector<T: Phenotype> {
    fn select(&self, population: Vec<Box<T>>) -> Result<Parents<T>, String>;
}
