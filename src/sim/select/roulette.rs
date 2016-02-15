// file: roulette.rs
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

use pheno::Phenotype;
use pheno::Fitness;
use super::*;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;

/// Selects phenotypes with a probability based on their fitness value.
///
/// Commonly known as *Roulette Wheel Selection*.
pub struct RouletteSelector {
    count: usize,
}

impl RouletteSelector {
    /// Create and return a roulette selector.
    ///
    /// Such a selector selects parents with a higher chance if those
    /// phenotypes have high fitness values. This selector yields `count` parents.
    ///
    /// * `count`: must be larger than zero, a multiple of two and less than the population size.
    pub fn new(count: usize) -> RouletteSelector {
        RouletteSelector { count: count }
    }
}

impl<T, F> Selector<T, F> for RouletteSelector where T: Phenotype<F>, F: Fitness {
    fn select(&self, population: &Vec<T>) -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count % 2 != 0 || self.count >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and less than the population size.",
                               self.count));
        }

        let mut results: Parents<T> = Vec::new();

        let mut cloned = population.clone();
        cloned.sort_by(|x, y| {
            (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
        });
        // Calculate cumulative fitness
        let cum_fitness: Vec<_> = cloned.iter()
                                        .scan(F::zero(), |state: &mut F, ref x| {
                                            *state = state.add(x.fitness());
                                            Some(*state)
                                        })
                                        .collect();

        let between = Range::new(cum_fitness[0].into(),
                                 cum_fitness[cum_fitness.len() - 1].into());
        let mut rng = ::rand::thread_rng();

        let mut selected = 0;
        while selected < self.count {
            let mut inner_selected: Vec<T> = Vec::with_capacity(2);
            while inner_selected.len() < 2 {
                let c: f64 = between.ind_sample(&mut rng);

                let result = cloned.iter().find(|p| c >= p.fitness().into());
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


#[cfg(test)]
mod tests {
    use ::sim::*;
    use ::sim::select::*;
    use ::pheno::*;
    use std::cmp;

    #[derive(Clone)]
    struct Test {
        f: i64,
    }

    impl Phenotype for Test {
        fn fitness(&self) -> Fitness {
            Fitness::new((self.f - 0).abs() as f64)
        }

        fn crossover(&self, t: &Test) -> Test {
            Test { f: cmp::min(self.f, t.f) }
        }

        fn mutate(&self) -> Test {
            if self.f < 0 {
                Test { f: self.f + 1 }
            } else if self.f > 0 {
                Test { f: self.f - 1 }
            } else {
                self.clone()
            }
        }
    }

    #[test]
    fn test_count_zero() {
        let selector = RouletteSelector::new(0);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = RouletteSelector::new(5);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = RouletteSelector::new(100);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = RouletteSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert_eq!(20,
                   selector.select(&population, FitnessType::Minimize).unwrap().len() * 2);
    }
}
