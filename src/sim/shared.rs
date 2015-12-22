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

/// An iteration limiter.
pub struct IterLimit {
    /// Maximum number of iterations allowed.
    max: u64,
    /// Current number of iterations.
    cur: u64
}

impl IterLimit {
    /// Create a new iteration limiter.
    pub fn new(max: u64) -> IterLimit {
        IterLimit {
            max: max,
            cur: 0
        }
    }

    /// Increment the number of iterations.
    pub fn inc(&mut self) {
        self.cur += 1;
    }

    /// Check if the maximum has been reached.
    pub fn reached(&self) -> bool {
        self.cur >= self.max
    }

    /// Reset the number of iterations to zero.
    pub fn reset(&mut self) {
        self.cur = 0;
    }

    /// Get the current number of iterations.
    pub fn get(&self) -> u64 {
        self.cur
    }
}
