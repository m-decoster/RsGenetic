/// Contains a sequential Simulator implementation.
pub mod seq {
    use pheno::Phenotype;
    use std::cmp::Ordering;
    use rand::Rng;

    /// The type of parent selection.
    pub enum SelectionType {
        /// Select only the `count * 2` best performing parents in terms of fitness.
        Maximize {
            /// Should be larger than 0.
            count: u32,
        },
        /// Perform tournament selection with tournament size `count`, running `num` tournaments.
        /// This yields `num * 2` parents.
        Tournament {
            /// Indicates the number of tournaments. Should be larger than 0.
            num: u32,
            /// Should be larger than 0.
            count: u32,
        },
        /// Perform Stochastic Universal Sampling to do the selection.
        /// Selects `count * 2` parents.
        Stochastic {
            /// Should be larger than 0.
            count: u32,
        },
    }

    /// Whether to maximize or to minimize the fitness value
    pub enum FitnessType {
        Maximize,
        Minimize,
    }

    /// A Simulator can run genetic algorithm simulations.
    pub struct Simulator<T: Phenotype> {
        population: Vec<Box<T>>,
        max_iters: i32,
        n_iters: i32,
        selection_type: SelectionType,
        fitness_type: FitnessType,
    }

    impl<T: Phenotype> Simulator<T> {
        /// Create a new Simulator.
        ///
        /// * `max_iters` indicates the maximum number of iterations to run
        /// before stopping.
        pub fn new(starting_population: Vec<Box<T>>,
                   max_iters: i32,
                   selection_type: SelectionType,
                   fitness_type: FitnessType)
                   -> Simulator<T> {
            Simulator {
                population: starting_population,
                max_iters: max_iters,
                n_iters: 0,
                selection_type: selection_type,
                fitness_type: fitness_type,
            }
        }

        /// Run the simulation, according to the settings
        /// chosen in the constructor of the Simulator.
        pub fn run(&mut self) {
            while self.n_iters < self.max_iters {
                // Perform selection
                let parents = match self.selection_type {
                    SelectionType::Maximize{count: c} => self.selection_maximize(c),
                    SelectionType::Tournament{num: n, count: c} => self.selection_tournament(n, c),
                    SelectionType::Stochastic{count: c} => self.selection_stochastic(c),
                };
                // Create children from the selected parents and mutate them
                let children: Vec<Box<T>> = parents.iter()
                                                   .map(|pair: &(Box<T>, Box<T>)| {
                                                       pair.0.crossover(&*(pair.1))
                                                   })
                                                   .map(|c| Box::new(c.mutate()))
                                                   .collect();
                // Kill off parts of the population at random to make room for the children
                self.kill_off(children.len());
                // Add the newly born children to the population
                for child in children {
                    self.population.push(child);
                }

                self.n_iters += 1
            }
        }

        /// Get the best performing organism.
        pub fn get(&self) -> Box<T> {
            let mut cloned = self.population.clone();
            cloned.sort_by(|x, y| {
                (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
            });
            match self.fitness_type {
                FitnessType::Maximize => cloned[cloned.len() - 1].clone(),
                FitnessType::Minimize => cloned[0].clone(),
            }
        }

        /// Select count*2 parents for breeding.
        fn selection_maximize(&self, count: u32) -> Vec<(Box<T>, Box<T>)> {
            assert!(count > 0);

            let mut cloned = self.population.clone();
            cloned.sort_by(|x, y| {
                (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
            });
            match self.fitness_type {
                FitnessType::Maximize => {
                    cloned.reverse();
                }
                _ => {}
            };
            let sorted: Vec<&Box<T>> = cloned.iter().take(2 * (count as usize)).collect();
            let mut index = 0;
            let mut result: Vec<(Box<T>, Box<T>)> = Vec::new();
            while index < sorted.len() {
                result.push((sorted[index].clone(), sorted[index + 1].clone()));
                index += 2;
            }
            result
        }

        /// Select parents using tournament selection.
        fn selection_tournament(&self, num: u32, count: u32) -> Vec<(Box<T>, Box<T>)> {
            assert!(num > 0);
            assert!(count > 0);

            let mut result: Vec<(Box<T>, Box<T>)> = Vec::new();
            let mut rng = ::rand::thread_rng();
            for _ in 0..num {
                let mut tournament: Vec<Box<T>> = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let index = rng.gen::<usize>() % self.population.len();
                    tournament.push(self.population[index].clone());
                }
                tournament.sort_by(|x, y| {
                    (*x).fitness().partial_cmp(&(*y).fitness()).unwrap_or(Ordering::Equal)
                });
                match self.fitness_type {
                    FitnessType::Maximize => {
                        result.push((tournament[tournament.len() - 1].clone(),
                                     tournament[tournament.len() - 2].clone()));
                    }
                    FitnessType::Minimize => {
                        result.push((tournament[0].clone(), tournament[1].clone()));
                    }
                }
            }
            result
        }

        /// Select parents using stochastic universal sampling.
        fn selection_stochastic(&self, count: u32) -> Vec<(Box<T>, Box<T>)> {
            assert!(count > 0);

            let ratio = self.population.len() / (count as usize);
            let mut result: Vec<(Box<T>, Box<T>)> = Vec::new();
            let mut i = ::rand::random::<usize>() % self.population.len() as usize;
            let mut selected = 0;
            while selected < count {
                result.push((self.population[i].clone(),
                             self.population[(i + ratio - 1) % self.population.len()].clone()));
                i += ratio - 1;
                i = i % self.population.len();
                selected += 2;
            }
            result
        }

        /// Kill off phenotypes using stochastic universal sampling.
        fn kill_off(&mut self, count: usize) {
            let old_len = self.population.len();
            let ratio = self.population.len() / count;
            let mut i = ::rand::random::<usize>() % self.population.len() as usize;
            let mut selected = 0;
            while selected < count {
                self.population.remove(i);
                i += ratio - 1;
                i = i % self.population.len();

                selected += 1;
            }
            assert!(self.population.len() == old_len - count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pheno;
    use std::cmp;

    struct Test {
        i: i32,
    }

    impl pheno::Phenotype for Test {
        fn fitness(&self) -> f64 {
            (self.i - 0).abs() as f64
        }

        fn crossover(&self, t: &Test) -> Test {
            Test { i: cmp::min(self.i, t.i) }
        }

        fn mutate(&self) -> Test {
            if self.i < 0 {
                Test { i: self.i + 1 }
            } else {
                Test { i: self.i - 1 }
            }
        }
    }

    impl Clone for Test {
        fn clone(&self) -> Self {
            Test { i: self.i }
        }
    }

    #[test]
    #[should_panic]
    fn test_maximize_count_0() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let mut s = seq::Simulator::new(tests,
                                        100,
                                        seq::SelectionType::Maximize { count: 0 },
                                        seq::FitnessType::Minimize);
        s.run();
    }

    #[test]
    #[should_panic]
    fn test_tournament_count_0() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let mut s = seq::Simulator::new(tests,
                                        100,
                                        seq::SelectionType::Tournament {
                                            num: 2,
                                            count: 0,
                                        },
                                        seq::FitnessType::Minimize);
        s.run();
    }

    #[test]
    #[should_panic]
    fn test_tournament_num_0() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let mut s = seq::Simulator::new(tests,
                                        100,
                                        seq::SelectionType::Tournament {
                                            num: 0,
                                            count: 1,
                                        },
                                        seq::FitnessType::Minimize);
        s.run();
    }

    #[test]
    #[should_panic]
    fn test_stochastic_count_0() {
        let tests = (0..100).map(|i| Box::new(Test { i: i })).collect();
        let mut s = seq::Simulator::new(tests,
                                        100,
                                        seq::SelectionType::Stochastic { count: 0 },
                                        seq::FitnessType::Minimize);
        s.run();
    }

    #[test]
    fn simple_convergence_test_maximize() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = seq::Simulator::new(tests,
                                        1000,
                                        seq::SelectionType::Maximize { count: 5 },
                                        seq::FitnessType::Minimize);
        s.run();
        assert_eq!((*s.get()).i, 0);
    }

    #[test]
    fn simple_convergence_test_tournament() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = seq::Simulator::new(tests,
                                        1000,
                                        seq::SelectionType::Tournament {
                                            count: 3,
                                            num: 5,
                                        },
                                        seq::FitnessType::Minimize);
        s.run();
        assert_eq!((*s.get()).i, 0);
    }

    #[test]
    fn simple_convergence_test_stochastic() {
        let tests = (0..100).map(|i| Box::new(Test { i: i + 10 })).collect();
        let mut s = seq::Simulator::new(tests,
                                        1000,
                                        seq::SelectionType::Stochastic { count: 5 },
                                        seq::FitnessType::Minimize);
        s.run();
        assert_eq!((*s.get()).i, 0);
    }
}
