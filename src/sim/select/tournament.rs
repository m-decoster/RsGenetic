// file: tournament.rs
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
use rand::Rng;

/// Runs several tournaments, and selects best performing phenotypes from each tournament.
pub struct TournamentSelector {
    count: usize,
    participants: usize,
}

impl TournamentSelector {
    /// Create and return a tournament selector.
    ///
    /// Such a selector runs `count / 2` tournaments, each with `participants` participants.
    /// From each tournament, the best 2 phenotypes are selected, yielding
    /// `count` parents.
    ///
    /// * `count`: must be larger than zero, a multiple of two and less than the population size.
    /// * `participants`: must be larger than zero and less than the population size.
    pub fn new(count: usize, participants: usize) -> TournamentSelector {
        TournamentSelector {
            count: count,
            participants: participants,
        }
    }
}

impl<T, F> Selector<T, F> for TournamentSelector where T: Phenotype<F>, F: Fitness {
    fn select(&self, population: &Vec<T>) -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and less than half the population size.",
                               self.count));
        }
        if self.participants <= 0 || self.participants >= population.len() {
            return Err(format!("Invalid parameter `participants`: {}. Should be larger than \
                                zero and less than the population size.",
                               self.participants));
        }

        let mut result: Parents<T> = Vec::new();
        let mut rng = ::rand::thread_rng();
        for _ in 0..(self.count / 2) {
            let mut tournament: Vec<T> = Vec::with_capacity(self.participants);
            for _ in 0..self.participants {
                let index = rng.gen_range::<usize>(0, population.len());
                tournament.push(population[index].clone());
            }
            tournament.sort_by(|x, y| {
                x.fitness().cmp(&y.fitness())
            });
                    result.push((tournament[tournament.len() - 1].clone(),
                                 tournament[tournament.len() - 2].clone()));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use ::sim::select::*;
    use ::test::Test;

    #[test]
    fn test_count_zero() {
        let selector = TournamentSelector::new(0, 1);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_participants_zero() {
        let selector = TournamentSelector::new(2, 0);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = TournamentSelector::new(5, 1);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = TournamentSelector::new(100, 1);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_participants_too_large() {
        let selector = TournamentSelector::new(2, 100);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert!(selector.select(&population).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = TournamentSelector::new(20, 5);
        let population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        assert_eq!(20,
                   selector.select(&population).unwrap().len() * 2);
    }
}
