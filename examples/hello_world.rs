// file: hello_world.rs
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

//! Daily programmer challenge 249.
//!
//! This program generates an input string
//! using a genetic algorithm.
extern crate rsgenetic;
extern crate rand;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;
use rsgenetic::pheno::*;
use rand::Rng;

#[derive(Clone)]
struct StringGuess {
    target: String,
    guess: String,
}

impl Phenotype for StringGuess {
    fn fitness(&self) -> f64 {
        // Hamming distance
        if self.target.len() != self.guess.len() {
            std::f64::INFINITY
        } else {
            self.target.chars().zip(self.guess.chars()).filter(|&(a, b)| a != b).count() as f64
        }
    }

    fn crossover(&self, other: &StringGuess) -> StringGuess {
        // 1-way crossover
        let mut rng = ::rand::thread_rng();
        let index = rng.gen::<usize>() % self.guess.len();
        let string_crossed_over = self.guess
                                      .chars()
                                      .take(index)
                                      .chain(other.guess.chars().skip(index))
                                      .collect();
        StringGuess {
            target: self.target.clone(),
            guess: string_crossed_over,
        }
    }

    fn mutate(&self) -> StringGuess {
        // Generate random character for one index in the string
        let mut rng = ::rand::thread_rng();
        // 50 % chance
        if rng.gen::<u8>() % 2 == 0 {
            let index = rng.gen::<usize>() % self.guess.len();
            let random_char = match rng.gen_ascii_chars().take(1).next() {
                Some(x) => x,
                None => panic!("Could not mutate phenotype."),
            };
            let mut new_guess = String::new();

            for (i, c) in self.guess.chars().enumerate() {
                if i != index {
                    new_guess.push(c);
                } else {
                    new_guess.push(random_char);
                }
            }
            StringGuess {
                target: self.target.clone(),
                guess: new_guess,
            }
        } else {
            self.clone()
        }
    }
}

fn main() {
    let input = "HelloWorld";
    let mut population: Vec<StringGuess> = Vec::with_capacity(500);
    let mut rng = ::rand::thread_rng();
    for _ in 0..500 {
        // Generate a random string
        let guess = rng.gen_ascii_chars().take(input.len()).collect::<String>();
        population.push(StringGuess {
            target: String::from(input),
            guess: guess,
        });
    }
    let mut s = Simulator::builder()
                     .set_population(&population)
                     .set_selector(Box::new(RouletteSelector::new(40)))
                     .set_max_iters(1000)
                     .set_fitness_type(FitnessType::Minimize)
                     .build();
    let mut index = 1;
    while let StepResult::Success = s.step() {
        let result = s.get().unwrap();
        println!("Gen: {} | Fitness: {} | {}",
                 index,
                 result.fitness(),
                 result.guess);
        if result.guess == input {
            break;
        }
        index += 1;
    }
    let time = s.time();
    println!("Execution time: {} ns.", time.unwrap());
}
