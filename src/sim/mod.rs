use pheno::Phenotype;

/// Contains a sequential `Simulator` implementation.
pub mod seq;
/// Contains private information.
mod shared;

/// A `Builder` can create new `Simulation` instances.
pub trait Builder<T> {
    /// Return the result.
    fn build(self) -> T;
}

/// Used to make it more clear that run time is defined in nanoseconds.
pub type NanoSecond = i64;

/// A `Simulation` is an execution of a genetic algorithm.
pub trait Simulation<T: Phenotype, B: Builder<Box<Self>>> : shared::Selector<T> {
    /// Create a `Builder` to create an instance.
    /// Because the population is a required parameter, you have to pass it here,
    /// instead of using a builder function.
    fn builder(population: Vec<Box<T>>) -> B;
    /// Run the simulation. Returns the time spent running in ns on success (unless
    /// an overflow occurred, then the time will be `None`)
    /// or a string containing an error.
    ///
    /// For example: we run with a `SelectionType` with invalid parameters. The returned value
    /// will be `Err(String)`. Otherwise, the result will be `Ok(Some(NanoSecond))`,
    /// or `Ok(None)` in case of an overflow.
    fn run(&mut self) -> Result<Option<NanoSecond>, String>;
    /// Get the best performing result of the simulation when it has ended.
    fn get(&self) -> Box<T>;
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
    /// Roulette Wheel Selection can only be used when the selected `FitnessType` is
    /// `FitnessType::Maximize`.
    /// This yields `count` parents.
    Roulette {
        /// Should be larger than 0 and smaller than the population size.
        count: u32,
    },
}

/// Whether to maximize or to minimize the fitness value
pub enum FitnessType {
    Maximize,
    Minimize,
}
