use pheno::*;
use std::cmp;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct MyFitness {
    pub f: i64,
}

impl Fitness for MyFitness {
    fn zero() -> Self {
        MyFitness { f: 0 }
    }

    fn abs_diff(&self, other: &Self) -> Self {
        MyFitness { f: (self.f - other.f).abs() }
    }
}

#[derive(Clone, Copy)]
pub struct Test {
    pub f: i64,
}

impl Phenotype<MyFitness> for Test {
    fn fitness(&self) -> MyFitness {
        MyFitness { f: self.f.abs() }
    }

    fn crossover(&self, t: &Test) -> Test {
        Test { f: cmp::min(self.f, t.f) }
    }

    fn mutate(&self) -> Test {
        if self.f < 0 {
            Test { f: self.f + 1 }
        } else if self.f > 0 {
            Test { f: self.f - 1 }
        } else {
            self.clone()
        }
    }
}
