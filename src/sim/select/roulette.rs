use pheno::Phenotype;
use super::*;
use super::super::FitnessType;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;

/// Create and return a roulette selector.
///
/// Such a selector selects parents with a higher chance if those
/// phenotypes have high fitness values. This selector yields `count` parents.
pub fn roulette_selector(count: usize) -> RouletteSelector {
    RouletteSelector { count: count }
}

pub struct RouletteSelector {
    count: usize
}

impl <T: Phenotype> Selector<T> for RouletteSelector {
    fn select(&self, population: &Vec<Box<T>>, _: FitnessType) -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than the population size.",
                               self.count));
        }

        let mut results: Parents<T> = Vec::new();

        let mut cloned = population.clone();
        cloned.sort_by(|x, y| {
            (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
        });
        // Calculate cumulative fitness
        let cum_fitness: Vec<_> = cloned.iter()
                                        .scan(0.0, |state, ref x| {
                                            *state = *state + x.fitness();
                                            Some(*state)
                                        })
                                        .collect();

        let between = Range::new(cum_fitness[0], cum_fitness[cum_fitness.len() - 1]);
        let mut rng = ::rand::thread_rng();

        let mut selected = 0;
        while selected < self.count {
            let mut inner_selected: Vec<Box<T>> = Vec::with_capacity(2);
            while inner_selected.len() < 2 {
                let c = between.ind_sample(&mut rng);

                let result = cloned.iter().find(|p| c >= p.fitness());
                if result.is_none() {
                    // This should never be true, but we wish to avoid panicking.
                    return Err(format!("Could not complete Roulette Selection. This most likely \
                                        indicates a bug in rsgenetic."));
                }
                inner_selected.push(result.unwrap().clone());
            }
            results.push((inner_selected[0].clone(), inner_selected[1].clone()));

            selected += 2;
        }
        Ok(results)
    }
}
