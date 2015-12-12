
/// Defines what a Phenotype is.
/// A Phenotype can breed with other Phenotypes, resulting in a single child.
/// A Phenotype can also be mutated.
/// Finally, a Phenotype has a certain fitness value associated with it.
pub trait Phenotype : Clone {
    /// Calculate the fitness of this Phenotype.
    fn fitness(&self) -> f64;
    /// Perform crossover on this Phenotype, returning a new Phenotype.
    fn crossover(&self, &Self) -> Self;
    /// Perform mutation on this Phenotype, returning a new Phenotype.
    fn mutate(&self) -> Self;
}
