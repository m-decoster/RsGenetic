use pheno::Phenotype;

/// `Parents` come in a `Vec` of two `Box<T>`'s.
pub type Parents<T> = Vec<(Box<T>, Box<T>)>;

/// A `Selector` can select `Parents` for a new iteration of a `Simulation`.
/// For information on what the different selection functions do, refer to `SelectionType`.
pub trait Selector<T: Phenotype> {
    fn selection_maximize(&self, count: u32) -> Result<Parents<T>, String>;
    fn selection_tournament(&self, num: u32, count: u32) -> Result<Parents<T>, String>;
    fn selection_stochastic(&self, count: u32) -> Result<Parents<T>, String>;
    fn selection_roulette(&self, count: u32) -> Result<Parents<T>, String>;
    /// Kill off phenotypes using stochastic universal sampling.
    fn kill_off(&mut self, count: usize) -> Result<(), String>;
}
