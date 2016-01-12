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

/// Defines what a Phenotype is.
/// A Phenotype can breed with other Phenotypes, resulting in a single child.
/// A Phenotype can also be mutated.
/// Finally, a Phenotype has a certain fitness value associated with it.
pub trait Phenotype : Clone {
    /// Calculate the fitness of this Phenotype.
    fn fitness(&self) -> f64;
    /// Perform crossover on this Phenotype, returning a new Phenotype.
    fn crossover(&self, &Self) -> Self;
    /// Perform mutation on this Phenotype, returning a new Phenotype.
    fn mutate(&self) -> Self;
}
