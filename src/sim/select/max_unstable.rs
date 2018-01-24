// file: max.rs
//
// Copyright 2015-2017 The RsGenetic Developers
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

use pheno::{Fitness, Phenotype};
use super::*;
use rayon::prelude::*;

/// Selects best performing phenotypes from the population.
#[derive(Clone, Copy, Debug)]
pub struct UnstableMaximizeSelector {
    count: usize,
}

impl UnstableMaximizeSelector {
    /// Create and return a maximizing selector with unstable parallel sorting.
    ///
    /// Such a selector selects only the `count` best performing phenotypes
    /// as parents.
    ///
    /// * `count`: must be larger than zero, a multiple of two and less than the population size.
    pub fn new(count: usize) -> UnstableMaximizeSelector {
        UnstableMaximizeSelector { count: count }
    }
}

impl<T, F> Selector<T, F> for UnstableMaximizeSelector
    where T: Phenotype<F>,
          F: Fitness,
          T: Send,
          T: Sync
{
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
        if self.count == 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and less than half the population size.",
                               self.count));
        }

        let mut borrowed: Vec<&T> = population.iter().collect();
        borrowed.par_sort_unstable_by(|x, y| y.fitness().cmp(&x.fitness()));
        let mut index = 0;
        let mut result: Parents<&T> = Vec::new();
        while index < self.count {
            result.push((borrowed[index], borrowed[index + 1]));
            index += 2;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use sim::select::*;
    use pheno::*;
    use test::Test;

    #[test]
    fn test_count_zero() {
        let selector = UnstableMaximizeSelector::new(0);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = UnstableMaximizeSelector::new(5);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = UnstableMaximizeSelector::new(100);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = UnstableMaximizeSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert_eq!(20, selector.select(&population).unwrap().len() * 2);
    }

    #[test]
    fn test_result_ok() {
        let selector = UnstableMaximizeSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        // The greatest fitness should be 99.
        assert!(selector.select(&population).unwrap()[0].0.fitness().f == 99);
    }

    #[test]
    fn test_contains_best() {
        let selector = UnstableMaximizeSelector::new(2);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        let parents = selector.select(&population).unwrap()[0];
        assert!(parents.0.fitness() ==
                population
                    .iter()
                    .max_by_key(|x| x.fitness())
                    .unwrap()
                    .fitness());
    }
}
