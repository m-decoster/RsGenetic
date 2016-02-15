// file: pheno.rs
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

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Sub, Add};

/// `Fitness` is a wrapper struct around a 64-bit floating point type,
/// implementing equality. This is useful to be able to use functions
/// for sorting and comparison, without requiring the population to be
/// `cloned`.
#[derive(Clone, Copy)]
pub struct Fitness {
    value: f64,
}

impl Fitness {
    /// Construct a new `Fitness` object.
    pub fn new(val: f64) -> Fitness {
        Fitness {
            value: val
        }
    }

    /// Calculate the absolute value of some `Fitness`.
    pub fn abs(&self) -> Fitness {
        Fitness {
            value: self.value.abs()
        }
    }
}

impl Into<f64> for Fitness {
    fn into(self) -> f64 {
        self.value
    }
}

impl Eq for Fitness {
}

impl PartialEq for Fitness {
    fn eq(&self, other: &Fitness) -> bool {
        (self.value - other.value).abs() < 0.0001
    }
}

impl PartialOrd for Fitness {
    fn partial_cmp(&self, other: &Fitness) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Fitness {
    fn cmp(&self, other: &Fitness) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Add for Fitness {
    type Output = Fitness;

    fn add(self, other: Fitness) -> Fitness {
        Fitness {
            value: self.value + other.value
        }
    }
}

impl Sub for Fitness {
    type Output = Fitness;

    fn sub(self, other: Fitness) -> Fitness {
        Fitness {
            value: self.value - other.value
        }
    }
}

impl fmt::Display for Fitness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Defines what a Phenotype is.
/// A Phenotype can breed with other Phenotypes, resulting in a single child.
/// A Phenotype can also be mutated.
/// Finally, a Phenotype has a certain fitness value associated with it.
///
/// If reasonable, it is recommended to have your implementation derive `Copy`.
pub trait Phenotype : Clone {
    /// Calculate the fitness of this Phenotype.
    fn fitness(&self) -> Fitness;
    /// Perform crossover on this Phenotype, returning a new Phenotype.
    fn crossover(&self, &Self) -> Self;
    /// Perform mutation on this Phenotype, returning a new Phenotype.
    fn mutate(&self) -> Self;
}
