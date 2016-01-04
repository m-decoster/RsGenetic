use pheno::Phenotype;

pub mod seq;
pub mod select;
mod iterlimit;
mod earlystopper;

/// A `Builder` can create new instances of an object.
/// For this library, only `Simulation` objects use this `Builder`.
pub trait Builder<T> {
    /// Return the result.
    fn build(self) -> T;
}

/// Simulation run time is defined in nanoseconds.
pub type NanoSecond = i64;
/// The result of a simulation, containing the best phenotype
/// or an error message.
pub type SimResult<T> = Result<Box<T>, String>;

/// The result of running a single step.
#[derive(PartialEq,Eq,Debug)]
pub enum StepResult {
    /// The step was successful, but the simulation has not finished.
    Success,
    /// The step was not successful.
    Failure,
    /// The step was successful and the simulation finished.
    Done
}

/// The result of running an entire simulation.
#[derive(PartialEq,Eq,Debug)]
pub enum RunResult {
    /// An error occurred somewhere during simulation.
    Failure,
    /// The simulation finished without errors.
    Done
}

/// A `Simulation` is an execution of a genetic algorithm.
pub trait Simulation<T: Phenotype> {
    type B: Builder<Box<Self>>;

    /// Create a `Builder` to create an instance.
    /// Because the population is a required parameter, you have to pass it here,
    /// instead of using a builder function.
    fn builder(population: &Vec<Box<T>>) -> Self::B;
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
    fn get(&self) -> SimResult<T>;
    /// Get the number of nanoseconds spent running, or `None` in case of an overflow.
    fn time(&self) -> Option<NanoSecond>;
    /// Get the number of iterations the `Simulator` has executed so far.
    fn iterations(&self) -> u64;
}

/// The type of parent selection.
pub enum SelectionType {
    /// Select only the `count * 2` best performing parents in terms of fitness.
    Maximize {
        /// Should be larger than 0 and smaller than half the population size.
        count: usize,
    },
    /// Perform tournament selection with tournament size `count`, running `num` tournaments.
    /// This yields `num * 2` parents.
    Tournament {
        /// Indicates the number of tournaments. Should be larger than 0 and smaller than the
        /// population size.
        num: usize,
        /// Should be larger than 0 and smaller than the population size.
        count: usize,
    },
    /// Perform Stochastic Universal Sampling to do the selection.
    /// Selects `count` parents.
    Stochastic {
        /// Should be larger than 0 and smaller than the population size.
        count: usize,
    },
    /// Perform Roulette Wheel Selection, also known as Fitness Proportionate Selection.
    /// This yields `count` parents.
    Roulette {
        /// Should be larger than 0 and smaller than the population size.
        count: usize,
    },
}

/// Whether to maximize or to minimize the fitness value.
pub enum FitnessType {
    /// The `Simulation` will try to increase the fitness value of phenotypes.
    Maximize,
    /// The `Simulation` will try to decrease the fitness value of phenotypes.
    Minimize,
}
