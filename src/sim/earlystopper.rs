use super::iterlimit::*;

/// Used for early stopping.
pub struct EarlyStopper {
    /// Minimum difference required for early stopping.
    delta: f64,
    /// Previously recorded fitness value.
    previous: f64,
    /// The number of iterations before stopping early.
    iter_limit: IterLimit,
}

impl EarlyStopper {
    /// Create a new `EarlyStopper`.
    fn new(delta: f64, n_iters: u64) -> EarlyStopper {
        EarlyStopper {
            delta: delta,
            previous: 0.0,
            iter_limit: IterLimit::new(n_iters),
        }
    }

    /// Update the `EarlyStopper` with a new fitness value.
    fn update(&mut self, fitness: f64) {
        if (fitness - self.previous).abs() < self.delta {
            self.previous = fitness;
            self.iter_limit.inc();
        } else {
            self.iter_limit.reset();
        }
    }

    /// Returns whether the `Simulator` should stop.
    fn reached(&self) -> bool {
        self.iter_limit.reached()
    }
}
