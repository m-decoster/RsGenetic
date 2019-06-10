// file: earlystopper.rs
//
// Copyright 2015-2017 The RsGenetic Developers
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
#[derive(Copy, Clone, Debug)]
pub struct EarlyStopper<F: Fitness> {
    /// Minimum difference required for early stopping.
    delta: F,
    /// Previously recorded fitness value.
    previous: F,
    /// The number of iterations before stopping early.
    iter_limit: IterLimit,
}

impl<F: Fitness> EarlyStopper<F> {
    /// Create a new `EarlyStopper`.
    pub fn new(delta: F, n_iters: u64) -> EarlyStopper<F> {
        EarlyStopper {
            delta,
            previous: F::zero(),
            iter_limit: IterLimit::new(n_iters),
        }
    }

    /// Update the `EarlyStopper` with a new fitness value.
    pub fn update(&mut self, fitness: F) {
        if self.previous.abs_diff(&fitness) < self.delta {
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
    use test::MyFitness;

    impl MyFitness {
        fn new(f: i64) -> MyFitness {
            MyFitness { f }
        }
    }

    #[test]
    fn test_early_stopper_reset() {
        let mut stopper = EarlyStopper::new(MyFitness::new(10), 5);
        for _ in 0..4 {
            stopper.update(MyFitness::new(1));
        }
        assert_eq!(stopper.reached(), false);
        stopper.update(MyFitness::new(20));
        assert_eq!(stopper.reached(), false);
    }

    #[test]
    fn test_early_stopper_reached() {
        let mut stopper = EarlyStopper::new(MyFitness::new(10), 5);
        for _ in 0..5 {
            stopper.update(MyFitness::new(1));
        }
        assert!(stopper.reached());
    }
}
