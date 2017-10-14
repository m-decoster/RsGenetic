#![feature(test)]

//! This benchmark runs a simulation which attempts to separate a collection of points
//! as much as possible, by maximizing the average distance between every point pair.
//! You can tweak the `NUM_POINTS` and `POPULATION_SIZE` constants to see how
//! the population size and the complexity of the fitness calculation (in this case O(n^2))
//! impacts both a sequential and parallel `Simulator`.

extern crate rsgenetic;
extern crate rand;
extern crate test;

use rsgenetic::sim::seq::Simulator as SeqSimulator;
use rsgenetic::sim::par::Simulator as ParSimulator;
use rsgenetic::sim::select::*;
use rsgenetic::pheno::*;
use std::cmp::Ordering;
use rand::Rng;

/// The number of points per Phenotype
const NUM_POINTS: usize = 1000;
/// The size of a population
const POPULATION_SIZE: usize = 500;

#[derive(Clone, Copy, Default)]
struct Point {
    x: f32,
    y: f32,
}

struct BenchFitness {
    f: f32,
}

impl Eq for BenchFitness {}

impl PartialEq for BenchFitness {
    fn eq(&self, other: &BenchFitness) -> bool {
        (self.f - other.f).abs() < 0.0001
    }
}

impl PartialOrd for BenchFitness {
    fn partial_cmp(&self, other: &BenchFitness) -> Option<Ordering> {
        self.f.partial_cmp(&other.f)
    }
}

impl Ord for BenchFitness {
    fn cmp(&self, other: &BenchFitness) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Fitness for BenchFitness {
    fn zero() -> BenchFitness {
        BenchFitness { f: 0.0 }
    }

    fn abs_diff(&self, other: &BenchFitness) -> BenchFitness {
        BenchFitness { f: (self.f - other.f).abs() }
    }
}

struct BenchPheno {
    points: Vec<Point>,
}

impl Phenotype<BenchFitness> for BenchPheno {
    fn fitness(&self) -> BenchFitness {
        // Calculate the average distance to all the points
        // This is an expensive (O(NUM_POINTS^2)) calculation
        let mut avg_dist = 0.0;
        let mut iter = self.points.iter();
        let mut i1 = 0;
        while let Some(point1) = iter.next() {
            let mut i2 = 0;
            for point2 in iter.by_ref() {
                if i1 == i2 {
                    continue;
                }

                let distance = ((point1.x - point2.x).powf(2.0) + (point1.y - point2.y).powf(2.0)).sqrt();
                avg_dist += distance;

                i2 += 1;
            }

            i1 += 1;
        }

        BenchFitness { f: avg_dist / (NUM_POINTS as f32) * (NUM_POINTS as f32) }
    }

    fn crossover(&self, other: &BenchPheno) -> BenchPheno {
        // 2-way crossover
        let crossover_indices = (POPULATION_SIZE / 3,
                                 2 * POPULATION_SIZE / 3);
        let mut crossed_over = vec![Point::default(); NUM_POINTS];
        for i in 0..crossover_indices.0 {
            crossed_over[i] = self.points[i];
        }
        for i in crossover_indices.0..crossover_indices.1 {
            crossed_over[i] = other.points[i];
        }
        for i in crossover_indices.1..NUM_POINTS {
            crossed_over[i] = self.points[i];
        }
        BenchPheno { points: crossed_over }
    }

    fn mutate(&self) -> BenchPheno {
        // Add a small number to both dimensions of each point
        BenchPheno {
            points: self.points
                .iter()
                .map(|&v| Point {
                    x: v.x + 0.1,
                    y: v.y - 0.1,
                })
                .collect(),
        }
    }
}

impl Clone for BenchPheno {
    fn clone(&self) -> BenchPheno {
        BenchPheno { points: self.points.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsgenetic::sim::{Builder, Simulation};
    use test::Bencher;

    #[bench]
    fn bench_sequential(b: &mut Bencher) {
        b.iter(|| {
            let mut population: Vec<BenchPheno> = Vec::with_capacity(POPULATION_SIZE);
            let mut rng = ::rand::thread_rng();
            for _ in 0..NUM_POINTS {
                let mut points = Vec::with_capacity(NUM_POINTS);
                for _ in 0..NUM_POINTS {
                    let x = rng.gen::<f32>();
                    let y = rng.gen::<f32>();
                    points.push(Point {x, y});
                }
                population.push(BenchPheno { points });
            }

            let mut s = SeqSimulator::builder(&mut population)
                .set_selector(Box::new(MaximizeSelector::new(10)))
                .set_max_iters(100)
                .build();
            s.run();
        });
    }

    #[bench]
    fn bench_parallel(b: &mut Bencher) {
        b.iter(|| {
            let mut population: Vec<BenchPheno> = Vec::with_capacity(POPULATION_SIZE);
            let mut rng = ::rand::thread_rng();
            for _ in 0..NUM_POINTS {
                let mut points = Vec::with_capacity(NUM_POINTS);
                for _ in 0..NUM_POINTS {
                    let x = rng.gen::<f32>();
                    let y = rng.gen::<f32>();
                    points.push(Point {x, y});
                }
                population.push(BenchPheno { points });
            }

            let mut s = ParSimulator::builder(&mut population)
                .set_selector(Box::new(MaximizeSelector::new(10)))
                .set_max_iters(100)
                .build();
            s.run();
        });
    }
}
