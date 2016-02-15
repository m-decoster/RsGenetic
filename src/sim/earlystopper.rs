// file: earlystopper.rs
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

use super::iterlimit::*;
use pheno::Fitness;

/// Used for early stopping.
pub struct EarlyStopper {
    /// Minimum difference required for early stopping.
    delta: Fitness,
    /// Previously recorded fitness value.
    previous: Fitness,
    /// The number of iterations before stopping early.
    iter_limit: IterLimit,
}

impl EarlyStopper {
    /// Create a new `EarlyStopper`.
    pub fn new(delta: Fitness, n_iters: u64) -> EarlyStopper {
        EarlyStopper {
            delta: delta,
            previous: Fitness::new(0.0),
            iter_limit: IterLimit::new(n_iters),
        }
    }

    /// Update the `EarlyStopper` with a new fitness value.
    pub fn update(&mut self, fitness: Fitness) {
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
    use ::pheno::Fitness;

    #[test]
    fn test_early_stopper_reset() {
        let mut stopper = EarlyStopper::new(Fitness::new(10.0), 5);
        for _ in 0..4 {
            stopper.update(Fitness::new(1.0));
        }
        assert_eq!(stopper.reached(), false);
        stopper.update(Fitness::new(20.0));
        assert_eq!(stopper.reached(), false);
    }

    #[test]
    fn test_early_stopper_reached() {
        let mut stopper = EarlyStopper::new(Fitness::new(10.0), 5);
        for _ in 0..5 {
            stopper.update(Fitness::new(1.0));
        }
        assert!(stopper.reached());
    }
}
