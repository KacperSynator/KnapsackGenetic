use rand::Rng;

use crate::individual::Individual;

pub enum MutationMethod {
    BitFlip,
    Swap,
    Inversion,
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
        MutationMethod::Swap => swap_mutation(individual, mutation_rate),
        MutationMethod::Inversion => inversion_mutation(individual, mutation_rate),
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

fn swap_mutation<T>(individual: &Individual<T>, mutation_rate: f64) -> Individual<T>
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let mut new_individual = individual.clone();

    if !rng.gen_bool(mutation_rate) {
        return new_individual;
    }

    let chromosome_len = new_individual.chromosome.len();
    let (i,j) = (rng.gen_range(0..chromosome_len), rng.gen_range(0..chromosome_len));
    new_individual.chromosome.swap(i, j);

    new_individual
}

fn inversion_mutation<T>(individual: &Individual<T>, mutation_rate: f64) -> Individual<T>
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let mut new_individual = individual.clone();

    if !rng.gen_bool(mutation_rate) {
        return new_individual;
    }

    let chromosome_len = new_individual.chromosome.len();
    let begin = rng.gen_range(0..chromosome_len);
    let end = rng.gen_range(begin..chromosome_len);

    new_individual.chromosome[begin..=end].reverse();

    new_individual
}
