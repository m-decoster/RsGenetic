# RsGenetic

## Summary and Features
RsGenetic is a simple framework for executing a genetic algorithm in Rust.  
This library is not yet feature-complete. It currently offers a sequential implementation that
can run genetic algorithms with a limited set of selection algorithms. The goal is
to provide at least one parallel implementation, to optimize these implementations,
and to provide more selection algorithms if needed.

## Examples and Documentation
Documentation is available [here](http://m-decoster.github.io/RsGenetic).  
Examples on how to use this library are shown below.

## Implementing Phenotype

```
// Define the structure of your Phenotype
struct Test {
    i: i32,
}

// Implement the Phenotype trait.
impl pheno::Phenotype for Test {
    fn fitness(&self) -> f64 {
        (self.i - 0).abs() as f64
    }

    fn crossover(&self, t: &Test) -> Test {
        Test { i: cmp::min(self.i, t.i) }
    }

    fn mutate(&self) -> Self {
        if self.i < 0 {
            Test { i: self.i + 1 }
        } else {
            Test { i: self.i - 1}
        }
    }
}

// Implement the Clone trait.
// This is required for the internal workings of the library.
impl Clone for Test {
    fn clone(&self) -> Self {
        Test { i: self.i }
    }
}
```

## Running a Simulation

```
// Generate a random population.
let mut tests: Vec<Box<Test>> = Vec::new();
for i in 0..100 {
    tests.push(Box::new(Test { i: i + 10 }));
}
// Create a simulator using a builder.
let mut s = *seq::Simulator::builder(tests) // Population is mandatory
                  .set_max_iters(1000)
                  .set_selection_type(sim::SelectionType::Tournament {
                        count: 3,
                        num: 5
                  })
                  .set_fitness_type(sim::FitnessType::Minimize)
                  .build();
// We can now run the simulator.
s.run();
assert!((*s.get()).i == 0); // For this simple example, we should always get 0.
```

## License
This library is available under Apache 2.0.

## Contributing
Contributions are always welcome. Take a look at the issues for any enhancements that need to be
done or bugs that need to be fixed. If you encounter any bugs while using the library, feel free to
open an issue and/or fix the bug, and submit pull requests.

## Notes
This library has only been tested with Rust stable 1.5.0.
