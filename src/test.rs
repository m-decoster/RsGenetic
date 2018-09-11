// file: test.rs
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

// This is a private module containing code used in
// several tests across the library.

use pheno::*;
use stats::{NoStats, StatsCollector};
use std::cmp;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct MyFitness {
    pub f: i64,
}

impl Fitness for MyFitness {
    fn zero() -> Self {
        MyFitness { f: 0 }
    }

    fn abs_diff(&self, other: &Self) -> Self {
        MyFitness {
            f: (self.f - other.f).abs(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Test {
    pub f: i64,
}

impl Phenotype<MyFitness> for Test {
    fn fitness(&self) -> MyFitness {
        MyFitness { f: self.f.abs() }
    }

    fn crossover(&self, t: &Test) -> Test {
        Test {
            f: cmp::min(self.f, t.f),
        }
    }

    fn mutate(&self) -> Test {
        if self.f < 0 {
            Test { f: self.f + 1 }
        } else if self.f > 0 {
            Test { f: self.f - 1 }
        } else {
            *self
        }
    }
}

impl StatsCollector<MyFitness> for NoStats {}
