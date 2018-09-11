// file: stochastic.rs
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

use super::*;
use pheno::{Fitness, Phenotype};
use rand::Rng;

/// Selects phenotypes at random, starting from a random index and taking equidistant jumps.
///
/// Commonly known as *Stochastic Universal Sampling*.
#[derive(Clone, Copy, Debug)]
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
    /// * `count`: must be larger than zero, a multiple of 2 and less than the population size.
    pub fn new(count: usize) -> StochasticSelector {
        StochasticSelector { count: count }
    }
}

impl<T, F> Selector<T, F> for StochasticSelector
where
    T: Phenotype<F>,
    F: Fitness,
{
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
        if self.count == 0 || self.count % 2 != 0 || self.count >= population.len() {
            return Err(format!(
                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than the population size.",
                self.count
            ));
        }

        let ratio = population.len() / self.count;
        let mut result: Parents<&T> = Vec::new();
        let mut i = ::rand::thread_rng().gen_range::<usize>(0, population.len());
        let mut selected = 0;
        while selected < self.count {
            result.push((
                &population[i],
                &population[(i + ratio - 1) % population.len()],
            ));
            i += ratio - 1;
            i %= population.len();
            selected += 2;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use sim::select::*;
    use test::Test;

    #[test]
    fn test_count_zero() {
        let selector = StochasticSelector::new(0);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = StochasticSelector::new(5);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = StochasticSelector::new(100);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = StochasticSelector::new(20);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert_eq!(20, selector.select(&population).unwrap().len() * 2);
    }
}
