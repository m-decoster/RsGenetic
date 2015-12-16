use pheno::Phenotype;

pub mod seq;
// Contains private information.
mod shared;

/// A `Builder` can create new instances of an object.
/// For this library, only `Simulation` objects use this `Builder`.
pub trait Builder<T> {
    /// Return the result.
    fn build(self) -> T;
}

/// Simulation run time is defined in nanoseconds.
pub type NanoSecond = i64;

/// A `Simulation` is an execution of a genetic algorithm.
pub trait Simulation<T: Phenotype> : shared::Selector<T> {
    type B: Builder<Box<Self>>;

    /// Create a `Builder` to create an instance.
    /// Because the population is a required parameter, you have to pass it here,
    /// instead of using a builder function.
    fn builder(population: Vec<Box<T>>) -> Self::B;
    /// Run the simulation. Returns the best phenotype
    /// or a string containing an error if something went wrong.
    fn run(&mut self) -> Result<Box<T>, String>;
    /// Get the number of nanoseconds spent running, or `None` in case of an overflow,
    /// or if the simulation wasn't run yet.
    fn time(&self) -> Option<NanoSecond>;
    /// Get the number of iterations the `Simulator` needed to converge.
    fn iterations(&self) -> i32;
}

/// The type of parent selection.
pub enum SelectionType {
    /// Select only the `count * 2` best performing parents in terms of fitness.
    Maximize {
        /// Should be larger than 0 and smaller than half the population size.
        count: u32,
    },
    /// Perform tournament selection with tournament size `count`, running `num` tournaments.
    /// This yields `num * 2` parents.
    Tournament {
        /// Indicates the number of tournaments. Should be larger than 0 and smaller than the
        /// population size.
        num: u32,
        /// Should be larger than 0 and smaller than the population size.
        count: u32,
    },
    /// Perform Stochastic Universal Sampling to do the selection.
    /// Selects `count` parents.
    Stochastic {
        /// Should be larger than 0 and smaller than the population size.
        count: u32,
    },
    /// Perform Roulette Wheel Selection, also known as Fitness Proportionate Selection.
    /// This yields `count` parents.
    Roulette {
        /// Should be larger than 0 and smaller than the population size.
        count: u32,
    },
}

/// Whether to maximize or to minimize the fitness value.
pub enum FitnessType {
    /// The `Simulation` will try to increase the fitness value of phenotypes.
    Maximize,
    /// The `Simulation` will try to decrease the fitness value of phenotypes.
    Minimize,
}
