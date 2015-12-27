//! Contains a sequential implementation of `::sim::Simulation`,
//! called a `Simulator`.
//!
//! To use a `Simulator`, you need a `SimulatorBuilder`, which you can
//! obtain by calling `Simulator::builder(p)`, with `p` your initial population.

use pheno::Phenotype;
use std::cmp::Ordering;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use super::*;
use super::shared::*;
use time::SteadyTime;

/// A sequential implementation of `::sim::Simulation`.
/// The genetic algorithm is run in a single thread.
pub struct Simulator<T: Phenotype> {
    population: Vec<Box<T>>,
    iter_limit: IterLimit,
    selection_type: SelectionType,
    fitness_type: FitnessType,
    earlystopper: Option<EarlyStopper>,
    duration: Option<NanoSecond>,
    error: Option<String>
}

/// Used for early stopping.
struct EarlyStopper {
    /// Minimum difference required for early stopping.
    delta: f64,
    /// Previously recorded fitness value.
    previous: f64,
    /// The number of iterations before stopping early.
    iter_limit: IterLimit,
}

impl EarlyStopper {
    /// Create a new `EarlyStopper`.
    fn new(delta: f64, n_iters: u64) -> EarlyStopper {
        EarlyStopper {
            delta: delta,
            previous: 0.0,
            iter_limit: IterLimit::new(n_iters),
        }
    }

    /// Update the `EarlyStopper` with a new fitness value.
    fn update(&mut self, fitness: f64) {
        if (fitness - self.previous).abs() < self.delta {
            self.previous = fitness;
            self.iter_limit.inc();
        } else {
            self.iter_limit.reset();
        }
    }

    /// Returns whether the `Simulator` should stop.
    fn reached(&self) -> bool {
        self.iter_limit.reached()
    }
}

impl<T: Phenotype> Simulation<T> for Simulator<T> {
    type B = SimulatorBuilder<T>;

    /// Create builder.
    fn builder(pop: &Vec<Box<T>>) -> SimulatorBuilder<T> {
        SimulatorBuilder {
            sim: Simulator {
                population: pop.clone(),
                iter_limit: IterLimit::new(100),
                selection_type: SelectionType::Maximize { count: 5 },
                fitness_type: FitnessType::Maximize,
                earlystopper: None,
                duration: Some(0),
                error: None,
            },
        }
    }

    fn step(&mut self) -> StepResult {
        let time_start = SteadyTime::now();
        let should_stop = match self.earlystopper {
            Some(ref x) => self.iter_limit.reached() || x.reached(),
            None => self.iter_limit.reached(),
        };
        if should_stop {
            return StepResult::Done
        } else {
            // Perform selection
            let parents_tmp = match self.selection_type {
                SelectionType::Maximize{count: c} => self.selection_maximize(c),
                SelectionType::Tournament{num: n, count: c} => self.selection_tournament(n, c),
                SelectionType::Stochastic{count: c} => self.selection_stochastic(c),
                SelectionType::Roulette{count: c} => self.selection_roulette(c),
            };
            if parents_tmp.is_err() {
                self.error = Some(parents_tmp.err().unwrap());
                return StepResult::Failure
            }
            let parents = parents_tmp.ok().unwrap();
            // Create children from the selected parents and mutate them.
            let mut children: Vec<Box<T>> = parents.iter()
                                                   .map(|pair: &(Box<T>, Box<T>)| {
                                                       pair.0.crossover(&*(pair.1))
                                                   })
                                                   .map(|c| Box::new(c.mutate()))
                                                   .collect();
            // Kill off parts of the population at random to make room for the children
            match self.kill_off(children.len()) {
                Ok(_) => {
                    self.population.append(&mut children);
                }
                Err(e) => {
                    self.error = Some(e);
                    return StepResult::Failure
                }
            }

            if let Some(ref mut stopper) = self.earlystopper {
                let mut cloned = self.population.clone();
                cloned.sort_by(|x, y| {
                    (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
                });
                let highest_fitness = match self.fitness_type {
                                          FitnessType::Maximize => cloned[cloned.len() - 1].clone(),
                                          FitnessType::Minimize => cloned[0].clone(),
                                      }
                                      .fitness();
                stopper.update(highest_fitness);
            }

            self.iter_limit.inc();
        }
        let this_time = (SteadyTime::now() - time_start).num_nanoseconds();
        self.duration = match self.duration {
            Some(x) => {
                match this_time {
                    Some(y) => Some(x + y),
                    None => None,
                }
            }
            None => None,
        };
        StepResult::Success // Not done yet, but successful
    }

    /// Run.
    fn run(&mut self) {
        // Loop until Failure or Done.
        while self.step() == StepResult::Success {}
    }

    fn get(&self) -> SimResult<T> {
        match self.error {
            Some(ref e) => Err(e.clone()),
            None => {
                let mut cloned = self.population.clone();
                cloned.sort_by(|x, y| {
                    (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
                });
                Ok(match self.fitness_type {
                    FitnessType::Maximize => cloned[cloned.len() - 1].clone(),
                    FitnessType::Minimize => cloned[0].clone(),
                })
            }
        }
    }

    fn iterations(&self) -> u64 {
        self.iter_limit.get()
    }

    fn time(&self) -> Option<NanoSecond> {
        self.duration
    }
}

impl<T: Phenotype> Selector<T> for Simulator<T> {
    /// Select count*2 parents for breeding.
    fn selection_maximize(&self, count: usize) -> Result<Parents<T>, String> {
        if count <= 0 || count * 2 >= self.population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than half the population size.",
                               count));
        }

        let mut cloned = self.population.clone();
        cloned.sort_by(|x, y| {
            (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
        });
        if let FitnessType::Maximize = self.fitness_type {
            cloned.reverse();
        }
        let sorted: Vec<&Box<T>> = cloned.iter().take(2 * count).collect();
        let mut index = 0;
        let mut result: Parents<T> = Vec::new();
        while index < sorted.len() {
            result.push((sorted[index].clone(), sorted[index + 1].clone()));
            index += 2;
        }
        Ok(result)
    }

    /// Select parents using tournament selection.
    fn selection_tournament(&self, num: usize, count: usize) -> Result<Parents<T>, String> {
        if num <= 0 || num * 2 >= self.population.len() {
            return Err(format!("Invalid parameter `num`: {}. Should be larger than zero and \
                                less than half the population size.",
                               num));
        }
        if count <= 0 || count >= self.population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than half the population size.",
                               count));
        }

        let mut result: Parents<T> = Vec::new();
        let mut rng = ::rand::thread_rng();
        for _ in 0..num {
            let mut tournament: Vec<Box<T>> = Vec::with_capacity(count);
            for _ in 0..count {
                let index = rng.gen::<usize>() % self.population.len();
                tournament.push(self.population[index].clone());
            }
            tournament.sort_by(|x, y| {
                (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
            });
            match self.fitness_type {
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

    /// Select parents using stochastic universal sampling.
    fn selection_stochastic(&self, count: usize) -> Result<Parents<T>, String> {
        if count <= 0 || count >= self.population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than the population size.",
                               count));
        }

        let ratio = self.population.len() / count;
        let mut result: Parents<T> = Vec::new();
        let mut i = ::rand::random::<usize>() % self.population.len();
        let mut selected = 0;
        while selected < count {
            result.push((self.population[i].clone(),
                         self.population[(i + ratio - 1) % self.population.len()].clone()));
            i += ratio - 1;
            i = i % self.population.len();
            selected += 2;
        }
        Ok(result)
    }

    /// Select parents using roulette wheel selection.
    fn selection_roulette(&self, count: usize) -> Result<Parents<T>, String> {
        if count <= 0 || count >= self.population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero and \
                                less than the population size.",
                               count));
        }

        let mut results: Parents<T> = Vec::new();

        let mut cloned = self.population.clone();
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
        while selected < count {
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

    /// Kill off phenotypes using stochastic universal sampling.
    fn kill_off(&mut self, count: usize) -> Result<(), String> {
        let old_len = self.population.len();
        let ratio = self.population.len() / count;
        let mut i = ::rand::random::<usize>() % self.population.len();
        let mut selected = 0;
        while selected < count {
            self.population.remove(i);
            i += ratio - 1;
            i = i % self.population.len();

            selected += 1;
        }
        if self.population.len() == old_len - count {
            Ok(())
        } else {
            Err(format!("Something went wrong during reduction of population. Invalid number of \
                         results."))
        }
    }
}

/// A `Builder` for the `Simulator` type.
pub struct SimulatorBuilder<T: Phenotype> {
    sim: Simulator<T>,
}

impl<T: Phenotype> SimulatorBuilder<T> {
    /// Set the maximum number of iterations of the resulting `Simulator`.
    ///
    /// The `Simulator` will stop running after this number of iterations.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_max_iters(mut self, i: u64) -> Self {
        self.sim.iter_limit = IterLimit::new(i);
        self
    }

    /// Set the selection type of the resulting `Simulator`.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_selection_type(mut self, t: SelectionType) -> Self {
        self.sim.selection_type = t;
        self
    }

    /// Set the fitness type of the resulting `Simulator`,
    /// determining whether the `Simulator` will try to maximize
    /// or minimize the fitness values of `Phenotype`s.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_fitness_type(mut self, t: FitnessType) -> Self {
        self.sim.fitness_type = t;
        self
    }

    /// Set early stopping. If for `n_iters` iterations, the change in the highest fitness
    /// is smaller than `delta`, the simulator will stop running.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_early_stop(mut self, delta: f64, n_iters: u64) -> Self {
        self.sim.earlystopper = Some(EarlyStopper::new(delta, n_iters));
        self
    }
}

impl<T: Phenotype> Builder<Box<Simulator<T>>> for SimulatorBuilder<T> {
    fn build(self) -> Box<Simulator<T>> {
        Box::new(self.sim)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // seq
    use super::super::*; // sim
    use pheno;
    use std::cmp;

    struct Test {
        i: i32,
    }

    impl pheno::Phenotype for Test {
        fn fitness(&self) -> f64 {
            (self.i - 0).abs() as f64
        }

        fn crossover(&self, t: &Test) -> Test {
            Test { i: cmp::min(self.i, t.i) }
        }

        fn mutate(&self) -> Test {
            if self.i < 0 {
                Test { i: self.i + 1 }
            } else {
                Test { i: self.i - 1 }
            }
        }
    }

    impl Clone for Test {
        fn clone(&self) -> Self {
            Test { i: self.i }
        }
    }

    #[test]
    fn test_maximize_invalid() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        // count 0
        let mut s = *seq::Simulator::builder(&tests)
                         .set_selection_type(SelectionType::Maximize { count: 0 })
                         .build();
        s.run();
        assert!(s.get().is_err());

        // count 101
        s = *seq::Simulator::builder(&tests)
                 .set_selection_type(SelectionType::Maximize { count: 101 })
                 .build();
        s.run();
        assert!(s.get().is_err());
    }

    #[test]
    fn test_tournament_invalid() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        // count 0
        let mut s = *seq::Simulator::builder(&tests)
                         .set_selection_type(SelectionType::Tournament { num: 2, count: 0 })
                         .build();
        s.run();
        assert!(s.get().is_err());

        // num 0
        s = *seq::Simulator::builder(&tests)
                 .set_selection_type(SelectionType::Tournament { num: 0, count: 1 })
                 .build();
        s.run();
        assert!(s.get().is_err());

        // num 51
        s = *seq::Simulator::builder(&tests)
                 .set_selection_type(SelectionType::Tournament {
                     num: 51,
                     count: 1,
                 })
                 .build();
        s.run();
        assert!(s.get().is_err());
    }

    #[test]
    fn test_stochastic_invalid() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        // count 0
        let mut s = *seq::Simulator::builder(&tests)
                         .set_selection_type(SelectionType::Stochastic { count: 0 })
                         .build();
        s.run();
        assert!(s.get().is_err());

        // count 101
        s = *seq::Simulator::builder(&tests)
                 .set_selection_type(SelectionType::Stochastic { count: 101 })
                 .build();
        s.run();
        assert!(s.get().is_err());
    }

    #[test]
    fn test_roulette_invalid() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();

        // count 0
        let mut s = *seq::Simulator::builder(&tests)
                         .set_selection_type(SelectionType::Roulette { count: 0 })
                         .build();
        s.run();
        assert!(s.get().is_err());

        // count 101
        s = *seq::Simulator::builder(&tests)
                 .set_selection_type(SelectionType::Roulette { count: 101 })
                 .build();
        s.run();
        assert!(s.get().is_err());
    }

    #[test]
    fn test_runtime() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let mut s = *seq::Simulator::builder(&tests)
                         .set_max_iters(2000)
                         .set_selection_type(SelectionType::Stochastic { count: 1 })
                         .set_fitness_type(FitnessType::Minimize)
                         .build();
        s.run();
        assert!(s.get().is_ok()); // The algorithm should not fail.
        assert!(s.time().is_some()); // The run time should not overflow for this example.
    }

    #[test]
    fn test_time_norun() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let s = *seq::Simulator::builder(&tests).build();
        assert!(s.time().unwrap() == 0);
    }

    #[test]
    fn test_earlystop() {
        // Run two tests: one with early stopping, one without.
        // The one with early stopping should have less iterations.
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let mut s_early = *seq::Simulator::builder(&tests)
                               .set_max_iters(1000)
                               .set_selection_type(SelectionType::Stochastic { count: 5 })
                               .set_fitness_type(FitnessType::Minimize)
                               .set_early_stop(0.1, 5)
                               .build();

        let mut s_no_early = *seq::Simulator::builder(&tests)
                                  .set_max_iters(1000)
                                  .set_selection_type(SelectionType::Stochastic { count: 5 })
                                  .set_fitness_type(FitnessType::Minimize)
                                  .build();

        // Both should run without error.
        s_early.run();
        s_no_early.run();
        assert!(s_early.get().is_ok());
        assert!(s_no_early.get().is_ok());

        // The one with early stopping should have less iterations.
        // It is impossible to have more, because the maximum is 1000 and without early stopping
        // we will always go to 1000.
        assert!(s_early.iterations() < s_no_early.iterations());
    }

    #[test]
    fn simple_convergence_test_maximize() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = *seq::Simulator::builder(&tests)
                         .set_max_iters(1000)
                         .set_selection_type(SelectionType::Maximize { count: 5 })
                         .set_fitness_type(FitnessType::Minimize)
                         .build();
        s.run();
        let result = s.get().unwrap();
        assert_eq!(result.i, 0);
    }

    #[test]
    fn test_step() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = *seq::Simulator::builder(&tests)
                         .set_max_iters(1000)
                         .set_selection_type(SelectionType::Maximize { count: 5 })
                         .set_fitness_type(FitnessType::Minimize)
                         .build();
        let result = s.step();
        assert_eq!(result, StepResult::Success); // This should not converge in one step.
        assert_eq!(s.iterations(), 1);
        assert!(s.time().unwrap() > 0); // Should not be `None` (otherwise we are way too slow).
        s.run();
        let final_result = s.get().unwrap();
        assert_eq!(final_result.i, 0);
    }

    #[test]
    fn simple_convergence_test_tournament() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = *seq::Simulator::builder(&tests)
                         .set_max_iters(1000)
                         .set_selection_type(SelectionType::Tournament { count: 3, num: 5 })
                         .set_fitness_type(FitnessType::Minimize)
                         .build();
        s.run();
        let result = s.get().unwrap();
        assert_eq!(result.i, 0);
    }

    #[test]
    fn simple_convergence_test_stochastic() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = *seq::Simulator::builder(&tests)
                         .set_max_iters(1000)
                         .set_selection_type(SelectionType::Stochastic { count: 5 })
                         .set_fitness_type(FitnessType::Minimize)
                         .build();
        s.run();
        let result = s.get().unwrap();
        assert_eq!(result.i, 0);
    }

    #[test]
    fn simple_convergence_test_roulette() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = *seq::Simulator::builder(&tests)
                         .set_max_iters(1000)
                         .set_selection_type(SelectionType::Roulette { count: 5 })
                         .set_fitness_type(FitnessType::Minimize)
                         .build();
        s.run();
        let result = s.get().unwrap();
        assert_eq!(result.i, 0);
    }
}
