use pheno::Phenotype;
use super::*;
use super::super::FitnessType;
use rand::Rng;

/// Selects phenotypes at random, starting from a random index and taking equidistant jumps.
///
/// Commonly known as *Stochastic Universal Sampling*.
pub struct StochasticSelector {
    count: usize,
}

impl StochasticSelector {
    /// Create and return a stochastic selector.
    ///
    /// Such a selector selects elements using stochastic universal sampling,
    /// yielding parents with low, medium and high fitness values. In total,
    /// `count` parents are selected.
    ///
    /// * `count`: must be larger than zero and less than the population size.
    pub fn new(count: usize) -> StochasticSelector {
        StochasticSelector { count: count }
    }
}

impl<T: Phenotype> Selector<T> for StochasticSelector {
    fn select(&self, population: &Vec<Box<T>>, _: FitnessType) -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than the population size.",
                               self.count));
        }

        let ratio = population.len() / self.count;
        let mut result: Parents<T> = Vec::new();
        let mut i = ::rand::thread_rng().gen_range::<usize>(0, population.len());
        let mut selected = 0;
        while selected < self.count {
            result.push((population[i].clone(),
                         population[(i + ratio - 1) % population.len()].clone()));
            i += ratio - 1;
            i = i % population.len();
            selected += 2;
        }
        Ok(result)
    }
}
