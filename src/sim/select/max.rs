// file: max.rs
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

use pheno::{Fitness, Phenotype};
use super::*;

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

impl<T, F> Selector<T, F> for MaximizeSelector
    where T: Phenotype<F>,
          F: Fitness
{
    fn select(&self, population: &[T]) -> Result<Parents<T>, String> {
        if self.count == 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and less than half the population size.",
                               self.count));
        }

        let mut cloned = population.to_vec();
        cloned.sort_by(|x, y| x.fitness().cmp(&y.fitness()));
        let sorted: Vec<&T> = cloned.iter().take(self.count).collect();
        let mut index = 0;
        let mut result: Parents<T> = Vec::new();
        while index < sorted.len() {
            result.push((sorted[index].clone(), sorted[index + 1].clone()));
            index += 2;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use ::sim::select::*;
    use ::pheno::*;
    use test::Test;

    #[test]
    fn test_count_zero() {
        let selector = MaximizeSelector::new(0);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = MaximizeSelector::new(5);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = MaximizeSelector::new(100);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = MaximizeSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert_eq!(20, selector.select(&population).unwrap().len() * 2);
    }

    #[test]
    fn test_result_ok() {
        let selector = MaximizeSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        // The lowest fitness should be zero.
        assert!(selector.select(&population).unwrap()[0].0.fitness().f == 0);
    }
}
