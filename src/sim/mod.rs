// file: mod.rs
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

use pheno::{Fitness,Phenotype};

pub mod seq;
pub mod select;
mod iterlimit;
mod earlystopper;

/// A `Builder` can create new instances of an object.
/// For this library, only `Simulation` objects use this `Builder`.
pub trait Builder<T: ?Sized> {
    /// Return the result.
    fn build(self) -> T where T: Sized;
}

/// Simulation run time is defined in nanoseconds.
pub type NanoSecond = i64;
/// The result of a simulation, containing the best phenotype
/// or an error message.
pub type SimResult<'a, T> = Result<&'a T, &'a str>;

/// The result of running a single step.
#[derive(PartialEq,Eq,Debug)]
pub enum StepResult {
    /// The step was successful, but the simulation has not finished.
    Success,
    /// The step was not successful.
    Failure,
    /// The step was successful and the simulation finished.
    Done,
}

/// The result of running an entire simulation.
#[derive(PartialEq,Eq,Debug)]
pub enum RunResult {
    /// An error occurred somewhere during simulation.
    Failure,
    /// The simulation finished without errors.
    Done,
}

/// A `Simulation` is an execution of a genetic algorithm.
pub trait Simulation<'a, T, F> where T: Phenotype<F>, F: Fitness {
    /// A `Builder` is used to create instances of a `Simulation`.
    type B: Builder<Self>;

    /// Create a `Builder` to create an instance.
    ///
    /// `population` is a required parameter of any `Simulation`, which
    /// is why it is a parameter of this function.
    fn builder(population: &'a mut Vec<T>) -> Self::B where Self: Sized;
    /// Run the simulation completely.
    fn run(&mut self) -> RunResult;
    /// Make one step in the simulation. This function returns a `StepResult`:
    ///
    /// * `StepResult::Success` when a step was successful, but the simulation is not done.
    /// * `StepResult::Failure` when an error occurred. Check the result of `get()`.
    /// * `StepResult::Done` on convergence or reaching the maximum iterations.
    ///
    /// Be careful to check for failures when running `step()` in a loop,
    /// to avoid infinite loops. To run the simulation until convergence or until
    /// reaching a maximum number of iterations, consider using `run()` instead:
    /// This function is mostly useful for making illustrations of the evolution.
    fn step(&mut self) -> StepResult;
    /// Get the result of the latest step or of a complete run.
    ///
    /// This function will either return the best performing individual,
    /// or an error string indicating what went wrong.
    fn get(&'a self) -> SimResult<'a, T>;
    /// Get the number of nanoseconds spent running, or `None` in case of an overflow.
    ///
    /// When `Self` is `par::Simulator`, i.e. a parallel simulator is used,
    /// the duration is the average duration of all child simulators.
    fn time(&self) -> Option<NanoSecond>;
    /// Get the number of iterations the `Simulator` has executed so far.
    ///
    /// When `Self` is `par::Simulator`, i.e. a parallel simulator is used,
    /// this returns the number of iterations made by the parallel simulator itself.
    fn iterations(&self) -> u64;
}
