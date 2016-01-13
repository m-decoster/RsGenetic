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
use super::*;
use super::super::FitnessType;
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

impl<T: Phenotype> Selector<T> for RouletteSelector {
    fn select(&self, population: &Vec<Box<T>>, _: FitnessType) -> Result<Parents<T>, String> {
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
        fn fitness(&self) -> f64 {
            (self.f - 0).abs() as f64
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
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = RouletteSelector::new(5);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = RouletteSelector::new(100);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = RouletteSelector::new(20);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert_eq!(20,
                   selector.select(&population, FitnessType::Minimize).unwrap().len() * 2);
    }
}
