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
        /// Perform tournament selection with tournament size `size`.
        /// This yields `count * size` parents.
        Tournament {
            /// Should be larger than 0.
            count: u32,
            /// Should be larger than 1.
            size: u32,
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
                    SelectionType::Tournament{count: c, size: s} => self.selection_tournament(c, s),
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
                FitnessType::Minimize => cloned[0].clone()
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
        fn selection_tournament(&self, count: u32, size: u32) -> Vec<(Box<T>, Box<T>)> {
            assert!(size >= 2);
            assert!(count > 0);

            let mut result: Vec<(Box<T>, Box<T>)> = Vec::new();
            let mut rng = ::rand::thread_rng();
            for _ in 0..count {
                let mut tournament: Vec<Box<T>> = Vec::with_capacity(size as usize);
                for _ in 0..size {
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
