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
use rsgenetic::pheno::*;
use rand::Rng;

type TruckIndex = usize;
type PackageSize = i32;
type Scheme = Vec<(TruckIndex, PackageSize)>;

const NUM_TRUCKS: usize = 5;
const CAPACITY: i32 = 10;
const PACKAGES: &'static [i32] = &[3, 8, 2, 7, 6, 1, 3];

struct LoadingScheme {
    scheme: Scheme,
}

impl Phenotype for LoadingScheme {
    fn fitness(&self) -> f64 {
        let mut ret: f64 = 0.0;
        // Calculate for each truck the total load.
        let mut trucks: Vec<PackageSize> = vec![0; NUM_TRUCKS];
        for load in self.scheme.clone() {
            trucks[load.0] += load.1;
        }
        for load in trucks {
            let space_left = CAPACITY - load;
            if space_left < 0 {
                // We have overfilled a truck: penalize this solution heavily.
                return std::f64::INFINITY;
            }
            if space_left == CAPACITY {
                // We have an empty truck: give this solution a little boost.
                // Normally, the contribution to the fitness value is 0, but now we
                // detract 3 to make this an even fitter solution.
                // Note that this is an empirically found value. We could even optimize this value
                // with a separate genetic algorithm!
                ret -= 3.0;
            } else {
                ret += space_left as f64;
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
                        .map(|pair: &(TruckIndex, PackageSize)| {
                            (rng.gen::<usize>() % NUM_TRUCKS, pair.1)
                        })
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
    let mut population: Vec<Box<LoadingScheme>> = Vec::with_capacity(300);
    let mut rng = ::rand::thread_rng();
    for _ in 0..300 {
        let mut pheno: Scheme = Vec::with_capacity(PACKAGES.len());
        for j in 0..PACKAGES.len() {
            let index = rng.gen::<usize>() % NUM_TRUCKS;
            pheno.push((index, PACKAGES[j]));
        }
        population.push(Box::new(LoadingScheme { scheme: pheno }));
    }
    let mut s = *Simulator::builder(population)
                     .set_max_iters(50)
                     .set_selection_type(SelectionType::Stochastic { count: 10 })
                     .set_fitness_type(FitnessType::Minimize)
                     .build();
    let time = s.run();
    println!("Execution time: {} ns.", time.unwrap().unwrap());
    let result = *s.get();
    println!("Result: {:?} | Fitness: {}.",
             result.scheme,
             result.fitness());
    let mut trucks: Vec<_> = vec![0; NUM_TRUCKS];
    for placement in result.scheme {
        trucks[placement.0] += placement.1;
    }
    println!("Load per truck: {:?}.", trucks);
}
