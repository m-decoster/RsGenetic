//! Contains a parallel implementation of `::sim::Simulation`,
//! called a `Simulator`. This implementation runs multiple
//! sequential `Simulator`s in parallel on multiple threads,
//! and communicates every few iterations to obtain better
//! results.
//!
//! To use a `Simulator`, you need a `SimulatorBuilder`, which you can
//! obtain by calling `Simulator::builder()`.

use pheno::Phenotype;
use super::*;
use super::select::*;
use super::iterlimit::*;
use super::earlystopper::*;
use time::SteadyTime;

/// A parallel implementation of `::sim::Simulation`.
/// The genetic algorithm runs on multiple threads
/// on the same machine.
pub struct Simulator<T: Phenotype>
{
    simulators: Vec<seq::Simulator<T>>,
    iter_limit: IterLimit,
    earlystopper: Option<EarlyStopper>,
    duration: Option<NanoSecond>,
    error: Option<String>,
}

impl<T: Phenotype> Simulation<T> for Simulator<T> {
    type B = SimulatorBuilder<T>;

    fn builder() -> SimulatorBuilder<T> {
        SimulatorBuilder {
            sim: Simulator {
                simulators: Vec::new(),
                iter_limit: IterLimit::new(100),
                earlystopper: None,
                duration: Some(0),
                error: None,
            },
        }
    }

    fn step(&mut self) -> StepResult {
        StepResult::Failure
    }

    fn run(&mut self) -> RunResult {
        RunResult::Failure
    }

    fn get(&self) -> SimResult<T> {
        Err(String::new())
    }

    fn iterations(&self) -> u64 {
        self.iter_limit.get()
    }

    fn time(&self) -> Option<NanoSecond> {
        self.duration
    }
}

/// A `Builder` for the `Simulator` type.
pub struct SimulatorBuilder<T: Phenotype>
{
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

    /// Set early stopping. If for `n_iters` iterations, the change in the highest fitness
    /// is smaller than `delta`, the simulator will stop running.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_early_stop(mut self, delta: f64, n_iters: u64) -> Self {
        self.sim.earlystopper = Some(EarlyStopper::new(delta, n_iters));
        self
    }

    /// Set the simulators to use.
    ///
    /// Returns itself for chaining purposes.
    pub fn set_simulators(mut self, simulators: Vec<seq::Simulator<T>>) -> Self {
        self.sim.simulators = simulators;
        self
    }
}

impl<T: Phenotype> Builder<Box<Simulator<T>>> for SimulatorBuilder<T> {
    fn build(self) -> Box<Simulator<T>> {
        Box::new(self.sim)
    }
}
