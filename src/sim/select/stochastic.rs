use pheno::Phenotype;
use super::*;
use super::super::FitnessType;
use rand::Rng;

/// Selects phenotypes at random, starting from a random index and taking equidistant jumps.
///
/// Commonly known as *Stochastic Universal Sampling*.
pub struct StochasticSelector {
    count: usize,
}

impl StochasticSelector {
    /// Create and return a stochastic selector.
    ///
    /// Such a selector selects elements using stochastic universal sampling,
    /// yielding parents with low, medium and high fitness values. In total,
    /// `count` parents are selected.
    ///
    /// * `count`: must be larger than zero, a multiple of 2 and less than the population size.
    pub fn new(count: usize) -> StochasticSelector {
        StochasticSelector { count: count }
    }
}

impl<T: Phenotype> Selector<T> for StochasticSelector {
    fn select(&self, population: &Vec<Box<T>>, _: FitnessType) -> Result<Parents<T>, String> {
        if self.count <= 0 || self.count % 2 != 0 || self.count >= population.len() {
            return Err(format!("Invalid parameter `count`: {}. Should be larger than zero, a \
                                multiple of two and less than the population size.",
                               self.count));
        }

        let ratio = population.len() / self.count;
        let mut result: Parents<T> = Vec::new();
        let mut i = ::rand::thread_rng().gen_range::<usize>(0, population.len());
        let mut selected = 0;
        while selected < self.count {
            result.push((population[i].clone(),
                         population[(i + ratio - 1) % population.len()].clone()));
            i += ratio - 1;
            i = i % population.len();
            selected += 2;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use ::sim::*;
    use ::sim::select::*;
    use ::pheno::*;
    use std::cmp;

    #[derive(Clone)]
    struct Test {
        f: i64,
    }

    impl Phenotype for Test {
        fn fitness(&self) -> f64 {
            (self.f - 0).abs() as f64
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

    #[test]
    fn test_count_zero() {
        let selector = StochasticSelector::new(0);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_odd() {
        let selector = StochasticSelector::new(5);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_count_too_large() {
        let selector = StochasticSelector::new(100);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert!(selector.select(&population, FitnessType::Minimize).is_err());
    }

    #[test]
    fn test_result_size() {
        let selector = StochasticSelector::new(20);
        let population: Vec<Box<Test>> = (0..100).map(|i| Box::new(Test { f: i })).collect();
        assert_eq!(20,
                   selector.select(&population, FitnessType::Minimize).unwrap().len() * 2);
    }
}
