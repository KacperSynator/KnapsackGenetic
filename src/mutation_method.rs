use rand::Rng;

use crate::individual::Individual;

pub enum MutationMethod {
    BitFlip,
}

pub fn mutation_method<T>(
    individual: &Individual<T>,
    method: &MutationMethod,
    mutation_rate: f64,
) -> Individual<T>
where
    T: Clone,
{
    match method {
        MutationMethod::BitFlip => bit_flip_mutation(individual, mutation_rate),
    }
}

fn bit_flip_mutation<T>(individual: &Individual<T>, mutation_rate: f64) -> Individual<T>
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let mut new_individual = individual.clone();
    new_individual.chromosome.iter_mut().for_each(|gene| {
        if rng.gen_bool(mutation_rate) {
            *gene = !*gene;
        }
    });

    new_individual
}
