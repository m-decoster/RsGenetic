// file: truck_loading.rs
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

//! This example shows another optimization problem.
//!
//! # Problem definition
//! We have access to 5 trucks, with capacity 10. We wish to load
//! several items on these trucks, with weights in the range [1,8].
//! We would like to use as little trucks as possible, and minimize
//! the lost space.
extern crate rsgenetic;
extern crate rand;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;
use rsgenetic::pheno::*;
use rand::Rng;

type TruckIndex = usize;
type PackageSize = i32;
type Scheme = Vec<(TruckIndex, PackageSize)>;
// Fitness is implemented for i32 by RsGenetic!
type SchemeFitness = i32;

const NUM_TRUCKS: usize = 5;
const CAPACITY: i32 = 10;
const PACKAGES: &'static [i32] = &[3, 8, 2, 7, 6, 1, 3];

struct LoadingScheme {
    scheme: Scheme,
}

impl Phenotype<SchemeFitness> for LoadingScheme {
    fn fitness(&self) -> SchemeFitness {
        let mut ret: i32 = 0;
        // Calculate for each truck the total load.
        let mut trucks: Vec<PackageSize> = vec![0; NUM_TRUCKS];
        for load in self.scheme.clone() {
            trucks[load.0] += load.1;
        }
        for load in trucks {
            let space_left = CAPACITY - load;
            if space_left < 0 {
                // We have overfilled a truck: penalize this solution heavily.
                return i32::min_value();
            }
            if space_left == CAPACITY {
                // We have an empty truck: give this solution a little boost.
                // Normally, the contribution to the fitness value is 0, but now we
                // add 1000 to make this an even fitter solution.
                // Note that this is an empirically found value. We could even optimize this value
                // with a separate genetic algorithm!
                ret += 1000;
            } else {
                ret -= space_left;
            }
        }
        ret
    }

    fn crossover(&self, other: &LoadingScheme) -> LoadingScheme {
        // 2-way crossover
        let mut rng = ::rand::thread_rng();
        let crossover_indices = (rng.gen::<usize>() % PACKAGES.len(),
                                 rng.gen::<usize>() % PACKAGES.len());
        let mut crossed_over: Scheme = vec![(0, 0); PACKAGES.len()];
        for i in 0..crossover_indices.0 {
            crossed_over[i] = self.scheme[i];
        }
        for i in crossover_indices.0..crossover_indices.1 {
            crossed_over[i] = other.scheme[i];
        }
        for i in crossover_indices.1..PACKAGES.len() {
            crossed_over[i] = self.scheme[i];
        }
        LoadingScheme { scheme: crossed_over }
    }

    fn mutate(&self) -> LoadingScheme {
        // Put some stuff on other trucks
        let mut rng = ::rand::thread_rng();
        LoadingScheme {
            scheme: self.scheme
                        .iter()
                        .map(|&(_, size)| (rng.gen::<usize>() % NUM_TRUCKS, size))
                        .collect(),
        }
    }
}

impl Clone for LoadingScheme {
    fn clone(&self) -> LoadingScheme {
        LoadingScheme { scheme: self.scheme.clone() }
    }
}

fn main() {
    let mut population: Vec<LoadingScheme> = Vec::with_capacity(300);
    let mut rng = ::rand::thread_rng();
    for _ in 0..300 {
        let mut pheno: Scheme = Vec::with_capacity(PACKAGES.len());
        for j in 0..PACKAGES.len() {
            let index = rng.gen::<usize>() % NUM_TRUCKS;
            pheno.push((index, PACKAGES[j]));
        }
        population.push(LoadingScheme { scheme: pheno });
    }
    let mut s = Simulator::builder(&mut population)
                    .set_selector(Box::new(MaximizeSelector::new(10)))
                    .set_max_iters(100)
                    .build();
    s.run();
    let result = s.get().unwrap();
    let time = s.time();
    println!("Execution time: {} ns.", time.unwrap());
    println!("Result: {:?} | Fitness: {}.",
             result.scheme,
             result.fitness());
    let mut trucks: Vec<_> = vec![0; NUM_TRUCKS];
    for &(index, size) in &result.scheme {
        trucks[index] += size;
    }
    println!("Load per truck: {:?}.", trucks);
}
