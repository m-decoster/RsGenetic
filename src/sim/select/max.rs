use pheno::Phenotype;
use super::*;
use super::super::FitnessType;
use std::cmp::Ordering;

/// Selects best performing phenotypes from the population.
pub struct MaximizeSelector {
    count: usize,
}

impl MaximizeSelector {
    /// Create and return a maximizing selector.
    ///
    /// Such a selector selects only the `count` best performing phenotypes
    /// as parents.
    ///
    /// * `count`: must be larger than zero, a multiple of two and less than the population size.
    pub fn new(count: usize) -> MaximizeSelector {
        MaximizeSelector { count: count }
    }
}

impl<T: Phenotype> Selector<T> for MaximizeSelector {
    fn select(&self,
              population: &Vec<Box<T>>,
              fitness_type: FitnessType)
              -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and \
                                less than half the population size.",
                               self.count));
        }

        let mut cloned = population.clone();
        cloned.sort_by(|x, y| {
            (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
        });
        if let FitnessType::Maximize = fitness_type {
            cloned.reverse();
        }
        let sorted: Vec<&Box<T>> = cloned.iter().take(self.count).collect();
        let mut index = 0;
        let mut result: Parents<T> = Vec::new();
        while index < sorted.len() {
            result.push((sorted[index].clone(), sorted[index + 1].clone()));
            index += 2;
        }
        Ok(result)
    }
}
