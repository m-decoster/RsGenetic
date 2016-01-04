/// An iteration limiter.
pub struct IterLimit {
    /// Maximum number of iterations allowed.
    max: u64,
    /// Current number of iterations.
    cur: u64,
}

impl IterLimit {
    /// Create a new iteration limiter.
    pub fn new(max: u64) -> IterLimit {
        IterLimit { max: max, cur: 0 }
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
