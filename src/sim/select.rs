//! The selection module provides a trait that can be implemented
//! to implement new selection algorithms. This module also provides a couple
//! of useful selection algorithms.

use pheno::Phenotype;
use super::FitnessType;

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

/// Create and return a maximizing selector.
///
/// Such a selector selects only the `count * 2` best performing phenotypes
/// as parents.
pub fn selector_maximize(count: usize) -> MaximizeSelector {
    MaximizeSelector { count: count }
}

struct MaximizeSelector {
    count: usize
}

impl <T: Phenotype> Selector<T> for MaximizeSelector {
    fn select(&self, population: &Vec<Box<T>>, fitness_type: FitnessType) -> Result<Parents<T>, String> {
        if count <= 0 || count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than half the population size.",
                               count));
        }

        let mut cloned = population.clone();
        cloned.sort_by(|x, y| {
            (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
        });
        if let FitnessType::Maximize = fitness_type {
            cloned.reverse();
        }
        let sorted: Vec<&Box<T>> = cloned.iter().take(2 * count).collect();
        let mut index = 0;
        let mut result: Parents<T> = Vec::new();
        while index < sorted.len() {
            result.push((sorted[index].clone(), sorted[index + 1].clone()));
            index += 2;
        }
        Ok(result)
    }
}
