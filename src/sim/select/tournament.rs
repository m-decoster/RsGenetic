use pheno::Phenotype;
use super::*;
use super::super::FitnessType;
use std::cmp::Ordering;
use rand::Rng;

/// Create and return a tournament selector.
///
/// Such a selector runs `num` tournaments, each with `count` participants.
/// From each tournament, the best 2 phenotypes are selected, yielding
/// `num * 2` parents.
pub fn tournament_selector(num: usize, count: usize) -> TournamentSelector {
    TournamentSelector {
        num: num,
        count: count
    }
}

pub struct TournamentSelector {
    num: usize,
    count: usize
}

impl <T: Phenotype> Selector<T> for TournamentSelector {
    fn select(&self, population: &Vec<Box<T>>, fitness_type: FitnessType) -> Result<Parents<T>, String> {
        if self.num <= 0 || self.num * 2 >= population.len() {
            return Err(format!("Invalid parameter `num`: {}. Should be larger than zero and \
                                less than half the population size.",
                               self.num));
        }
        if self.count <= 0 || self.count >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than half the population size.",
                               self.count));
        }

        let mut result: Parents<T> = Vec::new();
        let mut rng = ::rand::thread_rng();
        for _ in 0..self.num {
            let mut tournament: Vec<Box<T>> = Vec::with_capacity(self.count);
            for _ in 0..self.count {
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
