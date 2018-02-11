// file: max_parabole_steps.rs
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

//! This simple example shows how to use a simulator
//! that finds the maximum of the function f(x) = 10-(x+3)^2 (which is (-3,10)).
//! This example is the same as the `max_parabole` example, but it runs in steps
//! and prints out intermediate results.
extern crate rand;
extern crate rsgenetic;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;
use rsgenetic::pheno::*;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;

struct MyFitness {
    f: f64,
}

impl Eq for MyFitness {}

impl PartialEq for MyFitness {
    fn eq(&self, other: &MyFitness) -> bool {
        (self.f - other.f).abs() < 0.0001
    }
}

impl PartialOrd for MyFitness {
    fn partial_cmp(&self, other: &MyFitness) -> Option<Ordering> {
        self.f.partial_cmp(&other.f)
    }
}

impl Ord for MyFitness {
    fn cmp(&self, other: &MyFitness) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Fitness for MyFitness {
    fn zero() -> MyFitness {
        MyFitness { f: 0.0 }
    }

    fn abs_diff(&self, other: &MyFitness) -> MyFitness {
        MyFitness {
            f: (self.f - other.f).abs(),
        }
    }
}

struct MyData {
    x: f64,
}

impl Phenotype<MyFitness> for MyData {
    fn fitness(&self) -> MyFitness {
        // Calculate the function here, because it's what we wish to maximize.
        MyFitness {
            f: 10.0 - ((self.x + 3.0) * (self.x + 3.0)),
        }
    }

    fn crossover(&self, other: &MyData) -> MyData {
        // We take the average for crossover.
        MyData {
            x: (self.x + other.x) / 2.0,
        }
    }

    fn mutate(&self) -> MyData {
        // Shift x with a random number.
        // (This RNG code should reside somewhere else, not in this function, but it's just an
        // example).

        // Because we don't want to have too big mutations, we limit the range to -1, +1.
        // Smaller values can cause slower convergence, but larger values may cause completely
        // wrong values.
        let between = Range::new(-1.0, 1.0);
        let mut rng = rand::thread_rng();
        let offset = between.ind_sample(&mut rng);
        MyData { x: self.x + offset }
    }
}

impl Clone for MyData {
    fn clone(&self) -> MyData {
        MyData { x: self.x }
    }
}

fn main() {
    let mut population = (-300..300).map(|i| MyData { x: f64::from(i) }).collect();
    let mut s = Simulator::builder(&mut population)
        .set_selector(Box::new(StochasticSelector::new(10)))
        .set_max_iters(50)
        .build();
    while let StepResult::Success = s.checked_step() {
        let result = s.get().unwrap();
        println!(
            "Intermediate result: ({}, {}).",
            result.x,
            result.fitness().f
        );
    }
    let result = s.get().unwrap();
    let time = s.time();
    println!("Execution time: {} ns.", time.unwrap());
    println!("Expected result: (-3, 10).");
    println!("Result: ({}, {}).", result.x, result.fitness().f);
}
