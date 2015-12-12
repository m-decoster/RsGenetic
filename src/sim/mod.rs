use pheno::Phenotype;

/// Contains a sequential `Simulator` implementation.
pub mod seq;
/// Contains private information.
mod shared;

/// A `Simulation` is an execution of a genetic algorithm.
pub trait Simulation<T: Phenotype> : shared::Selector<T> {
    /// Run the simulation.
    fn run(&mut self);
    /// Get the best performing result of the simulation when it has ended.
    fn get(&self) -> Box<T>;
}


/// The type of parent selection.
pub enum SelectionType {
    /// Select only the `count * 2` best performing parents in terms of fitness.
    Maximize {
        /// Should be larger than 0.
        count: u32,
    },
    /// Perform tournament selection with tournament size `count`, running `num` tournaments.
    /// This yields `num * 2` parents.
    Tournament {
        /// Indicates the number of tournaments. Should be larger than 0.
        num: u32,
        /// Should be larger than 0.
        count: u32,
    },
    /// Perform Stochastic Universal Sampling to do the selection.
    /// Selects `count * 2` parents.
    Stochastic {
        /// Should be larger than 0.
        count: u32,
    },
}

/// Whether to maximize or to minimize the fitness value
pub enum FitnessType {
    Maximize,
    Minimize,
}
