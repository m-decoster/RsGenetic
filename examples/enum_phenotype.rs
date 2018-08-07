// file: enum_phenotype.rs
//
// Copyright 2015-2018 The RsGenetic Developers
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

//! This example shows how to provide several crossover or mutation implementations
//! for a single `Phenotype`.
//! 
//! This example was created in reference to [issue 30](https://github.com/m-decoster/RsGenetic/issues/30).
extern crate rsgenetic;

use rsgenetic::sim::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::select::*;
use rsgenetic::pheno::*;
use rsgenetic::stats::NoStats;

#[derive(Clone, Copy, Debug)]
struct MyPhenotype {
    variant: MyVariant,
    value: i32,
}

#[derive(Clone, Copy, Debug)]
enum MyVariant {
    Variant1,
    Variant2
}

impl Phenotype<i32> for MyPhenotype {
    fn fitness(&self) -> i32 {
        self.value
    }

    fn crossover(&self, other: &MyPhenotype) -> MyPhenotype {
        match self.variant {
            MyVariant::Variant1 => MyPhenotype {
                variant: self.variant,
                value: self.value + other.value
            },
            MyVariant::Variant2 => MyPhenotype {
                variant: self.variant,
                value: self.value - other.value
            }
        }
    }

    fn mutate(&self) -> MyPhenotype {
        match self.variant {
            MyVariant::Variant1 => MyPhenotype {
                variant: self.variant,
                value: self.value / 2
            },
            MyVariant::Variant2 => MyPhenotype {
                variant: self.variant,
                value: self.value * 2
            }
        }
    }
}

fn main() {
    let mut population: Vec<MyPhenotype> = Vec::with_capacity(300);
    for i in 0..150 {
        population.push(MyPhenotype { variant: MyVariant::Variant1, value: i });
        population.push(MyPhenotype { variant: MyVariant::Variant2, value: i })
    }
    #[allow(deprecated)]
    let mut s: Simulator<_, _, NoStats> = Simulator::builder(&mut population)
        .set_selector(Box::new(MaximizeSelector::new(10)))
        .set_max_iters(100)
        .build();
    s.run();
    let result = s.get().unwrap();
    let time = s.time();
    println!("Execution time: {} ns.", time.unwrap());
    println!(
        "Result: {:?} | Fitness: {}.",
        result,
        result.fitness()
    );
}
