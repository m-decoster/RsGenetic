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

/// A `Fitness` value is used to determine the quality of a `Phenotype`.
/// `Fitness` values should have an ordering.
///
/// **Make sure the following statement holds:**
/// A `Phenotype` with a `Fitness` value of `f1` performs better than
/// another `Phenotype` with a `Fitness` value of `f2` iff `f1 > f2`.
pub trait Fitness: Ord + Eq {
    /// Get the zero value of this `Fitness` value.
    /// The internal value should be 0.
    fn zero() -> Self;
    /// Get the absolute difference between two `Fitness` values.
    fn abs_diff(&self, other: &Self) -> Self;
}

/// Defines what a Phenotype is.
/// A Phenotype can breed with other Phenotypes, resulting in a single child.
/// A Phenotype can also be mutated.
/// Finally, a Phenotype has a certain fitness value associated with it.
///
/// If reasonable, it is recommended to have your implementation derive `Copy`.
pub trait Phenotype<F>: Clone
    where F: Fitness
{
    /// Calculate the fitness of this Phenotype.
    fn fitness(&self) -> F;
    /// Perform crossover on this Phenotype, returning a new Phenotype.
    fn crossover(&self, &Self) -> Self;
    /// Perform mutation on this Phenotype, returning a new Phenotype.
    fn mutate(&self) -> Self;
}
