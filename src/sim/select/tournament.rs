use pheno::Phenotype;
use super::*;
use super::super::FitnessType;
use std::cmp::Ordering;
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

impl<T: Phenotype> Selector<T> for TournamentSelector {
    fn select(&self,
              population: &Vec<Box<T>>,
              fitness_type: FitnessType)
              -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, \
                                a multiple of two and \
                                less than half the population size.",
                               self.count));
        }
        if self.participants <= 0 || self.participants >= population.len() {
            return Err(format!("Invalid parameter `participants`: {}. Should be larger than zero and \
                                less than the population size.",
                               self.participants));
        }

        let mut result: Parents<T> = Vec::new();
        let mut rng = ::rand::thread_rng();
        for _ in 0..(self.count / 2) {
            let mut tournament: Vec<Box<T>> = Vec::with_capacity(self.participants);
            for _ in 0..self.participants {
                let index = rng.gen_range::<usize>(0, population.len());
                tournament.push(population[index].clone());
            }
            tournament.sort_by(|x, y| {
                (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
            });
            match fitness_type {
                FitnessType::Maximize => {
                    result.push((tournament[tournament.len() - 1].clone(),
                                 tournament[tournament.len() - 2].clone()));
                }
                FitnessType::Minimize => {
                    result.push((tournament[0].clone(), tournament[1].clone()));
                }
            }
        }
        Ok(result)
    }
}
