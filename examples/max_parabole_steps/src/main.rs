//! This simple example shows how to use a simulator
//! that finds the maximum of the function f(x) = 10-(x+3)^2 (which is (-3,10)).
//! This example is the same as the `max_parabole` example, but it runs in steps
//! and prints out intermediate results.
extern crate rsgenetic;
extern crate rand;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::pheno::*;
use rand::distributions::{IndependentSample, Range};

struct MyData {
    x: f64,
}

impl Phenotype for MyData {
    fn fitness(&self) -> f64 {
        // Calculate the function here, because it's what we wish to maximize.
        10.0 - ((self.x + 3.0) * (self.x + 3.0))
    }

    fn crossover(&self, other: &MyData) -> MyData {
        // We take the average for crossover.
        MyData {
            x: (self.x + other.x) / 2.0
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
        MyData {
            x: self.x + offset
        }
    }
}

impl Clone for MyData {
    fn clone(&self) -> MyData {
        MyData { x: self.x }
    }
}

fn main() {
    let population = (-300..300).map(|i| Box::new(MyData{ x: i as f64 })).collect();
    let mut s = *Simulator::builder(&population)
                            .set_max_iters(50)
                            .set_selection_type(SelectionType::Stochastic {
                                count: 10
                            })
                            .build();
    while s.step().is_none() {
        let result = s.get();
        println!("Intermediate result: ({}, {}).", result.x, result.fitness());
    }
    let result = s.get();
    let time = s.time();
    println!("Execution time: {} ns.", time.unwrap());
    println!("Expected result: (-3, 10).");
    println!("Result: ({}, {}).", result.x, result.fitness());
}
