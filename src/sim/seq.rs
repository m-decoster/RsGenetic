// file: seq.rs
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

//! Contains a sequential implementation of `::sim::Simulation`,
//! called a `Simulator`.
//!
//! To use a `Simulator`, you need a `SimulatorBuilder`, which you can
//! obtain by calling `Simulator::builder()`.

use pheno::Phenotype;
use pheno::Fitness;
use rand::Rng;
use super::*;
use super::select::*;
use super::iterlimit::*;
use super::earlystopper::*;
use time::SteadyTime;
use std::marker::PhantomData;

/// A sequential implementation of `::sim::Simulation`.
/// The genetic algorithm is run in a single thread.
pub struct Simulator<'a, T, F>
    where T: 'a + Phenotype<F>,
          F: Fitness
{
    population: &'a mut Vec<T>,
    iter_limit: IterLimit,
    selector: Box<Selector<T, F>>,
    earlystopper: Option<EarlyStopper<F>>,
    duration: Option<NanoSecond>,
    error: Option<String>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, F> Simulation<'a, T, F> for Simulator<'a, T, F>
    where T: Phenotype<F>,
          F: Fitness
{
    type B = SimulatorBuilder<'a, T, F>;

    /// Create builder.
    fn builder(population: &'a mut Vec<T>) -> SimulatorBuilder<'a, T, F> {
        SimulatorBuilder {
            sim: Simulator {
                population: population,
                iter_limit: IterLimit::new(100),
                selector: Box::new(MaximizeSelector::new(3)),
                earlystopper: None,
                duration: Some(0),
                error: None,
                phantom: PhantomData::default(),
            },
        }
    }

    fn step(&mut self) -> StepResult {
        if self.population.is_empty() {
            self.error = Some(format!("Tried to run a simulator without a population, or the \
                                       population was empty."));
            return StepResult::Failure;
        }
        let time_start = SteadyTime::now();
        let should_stop = match self.earlystopper {
            Some(ref x) => self.iter_limit.reached() || x.reached(),
            None => self.iter_limit.reached(),
        };
        if should_stop {
            return StepResult::Done;
        } else {
            // Perform selection
            let parents = match self.selector.select(&self.population) {
                Ok(parents) => parents,
                Err(e) => {
                    self.error = Some(e);
                    return StepResult::Failure;
                }
            };
            // Create children from the selected parents and mutate them.
            let mut children: Vec<T> = parents.iter()
                                              .map(|&(ref a, ref b)| a.crossover(b))
                                              .map(|c| c.mutate())
                                              .collect();
            // Kill off parts of the population at random to make room for the children
            self.kill_off(children.len());
            self.population.append(&mut children);

            if let Some(ref mut stopper) = self.earlystopper {
                let highest_fitness = self.population
                                          .iter()
                                          .max_by_key(|x| x.fitness())
                                          .unwrap()
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
    fn run(&mut self) -> RunResult {
        // Loop until Failure or Done.
        loop {
            match self.step() {
                StepResult::Success => {}
                StepResult::Failure => return RunResult::Failure,
                StepResult::Done => return RunResult::Done,
            }
        }
    }

    fn get(&'a self) -> SimResult<'a, T> {
        match self.error {
            Some(ref e) => Err(e),
            None => Ok(self.population.iter().max_by_key(|x| x.fitness()).unwrap()),
        }
    }

    fn iterations(&self) -> u64 {
        self.iter_limit.get()
    }

    fn time(&self) -> Option<NanoSecond> {
        self.duration
    }

    fn population(&self) -> Vec<T> {
        self.population.clone()
    }
}

impl<'a, T, F> Simulator<'a, T, F>
    where T: Phenotype<F>,
          F: Fitness
{
    /// Kill off phenotypes using stochastic universal sampling.
    fn kill_off(&mut self, count: usize) {
        let ratio = self.population.len() / count;
        let mut i = ::rand::thread_rng().gen_range::<usize>(0, self.population.len());
        let mut selected = 0;
        while selected < count {
            self.population.remove(i);
            i += ratio - 1;
            i = i % self.population.len();

            selected += 1;
        }
    }
}

/// A `Builder` for the `Simulator` type.
pub struct SimulatorBuilder<'a, T, F>
    where T: 'a + Phenotype<F>,
          F: Fitness
{
    sim: Simulator<'a, T, F>,
}

impl<'a, T, F> SimulatorBuilder<'a, T, F>
    where T: Phenotype<F>,
          F: Fitness
{
    /// Set the selector of the resulting `Simulator`.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_selector(mut self, sel: Box<Selector<T, F>>) -> Self {
        self.sim.selector = sel;
        self
    }

    /// Set the maximum number of iterations of the resulting `Simulator`.
    ///
    /// The `Simulator` will stop running after this number of iterations.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_max_iters(mut self, i: u64) -> Self {
        self.sim.iter_limit = IterLimit::new(i);
        self
    }

    /// Set early stopping. If for `n_iters` iterations, the change in the highest fitness
    /// is smaller than `delta`, the simulator will stop running.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_early_stop(mut self, delta: F, n_iters: u64) -> Self {
        self.sim.earlystopper = Some(EarlyStopper::new(delta, n_iters));
        self
    }
}

impl<'a, T, F> Builder<Simulator<'a, T, F>> for SimulatorBuilder<'a, T, F>
    where T: Phenotype<F>,
          F: Fitness
{
    fn build(self) -> Simulator<'a, T, F> {
        self.sim
    }
}

#[cfg(test)]
mod tests {
    use ::sim::*;
    use ::sim::select::*;
    use test::Test;
    use test::MyFitness;

    #[test]
    fn test_kill_off_count() {
        let selector = MaximizeSelector::new(2);
        let mut population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        let mut s = seq::Simulator::builder(&mut population)
                        .set_selector(Box::new(selector))
                        .build();
        s.kill_off(10);
        assert_eq!(s.population.len(), 90);
    }

    #[test]
    fn test_max_iters() {
        let selector = MaximizeSelector::new(2);
        let mut population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        let mut s = seq::Simulator::builder(&mut population)
                        .set_selector(Box::new(selector))
                        .set_max_iters(2)
                        .build();
        s.run();
        assert!(s.iterations() <= 2);
    }

    #[test]
    fn test_early_stopping() {
        let selector = MaximizeSelector::new(2);
        let mut population: Vec<Test> = (0..100).map(|_| Test { f: 0 }).collect();
        let mut s = seq::Simulator::builder(&mut population)
                        .set_selector(Box::new(selector))
                        .set_early_stop(MyFitness { f: 10 }, 5)
                        .set_max_iters(10)
                        .build();
        s.run();
        assert!(s.iterations() <= 5);
    }

    #[test]
    fn test_selector_error_propagate() {
        let selector = MaximizeSelector::new(0);
        let mut population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        let mut s = seq::Simulator::builder(&mut population)
                        .set_selector(Box::new(selector))
                        .build();
        s.run();
        assert!(s.get().is_err());
    }

    #[test]
    fn test_population_get() {
        let selector = MaximizeSelector::new(0);
        let mut population: Vec<Test> = (0..100).map(|i| Test { f: i }).collect();
        let population_len = population.len();
        let s = seq::Simulator::builder(&mut population)
                        .set_selector(Box::new(selector))
                        .build();
        let gotten_population = s.population();
        assert!(gotten_population.len() == population_len);
    }
}
