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
              population: &Vec<T>,
              fitness_type: FitnessType)
              -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and less than half the population size.",
                               self.count));
        }

        let mut cloned = population.clone();
        cloned.sort_by(|x, y| {
            (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
        });
        if let FitnessType::Maximize = fitness_type {
            cloned.reverse();
        }
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
        let selector = MaximizeSelector::new(0);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = MaximizeSelector::new(5);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = MaximizeSelector::new(100);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = MaximizeSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert_eq!(20,
                   selector.select(&population, FitnessType::Minimize).unwrap().len() * 2);
    }

    #[test]
    fn test_result_ok() {
        let selector = MaximizeSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        // The lowest fitness should be zero.
        assert!((0.0 -
                 (selector.select(&population, FitnessType::Minimize)
                           .unwrap()[0]
                       .0)
                     .fitness())
                    .abs() < 0.001);
    }
}
