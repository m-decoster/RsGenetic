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
    pub fn new(delta: f64, n_iters: u64) -> EarlyStopper {
        EarlyStopper {
            delta: delta,
            previous: 0.0,
            iter_limit: IterLimit::new(n_iters),
        }
    }

    /// Update the `EarlyStopper` with a new fitness value.
    pub fn update(&mut self, fitness: f64) {
        if (fitness - self.previous).abs() < self.delta {
            self.previous = fitness;
            self.iter_limit.inc();
        } else {
            self.iter_limit.reset();
        }
    }

    /// Returns whether the `Simulator` should stop.
    pub fn reached(&self) -> bool {
        self.iter_limit.reached()
    }
}

#[cfg(test)]
mod tests {
    use super::EarlyStopper;

    #[test]
    fn test_early_stopper_reset() {
        let mut stopper = EarlyStopper::new(10.0, 5);
        for _ in 0..4 {
            stopper.update(1.0);
        }
        assert_eq!(stopper.reached(), false);
        stopper.update(20.0);
        assert_eq!(stopper.reached(), false);
    }

    #[test]
    fn test_early_stopper_reached() {
        let mut stopper = EarlyStopper::new(10.0, 5);
        for _ in 0..5 {
            stopper.update(1.0);
        }
        assert!(stopper.reached());
    }
}
