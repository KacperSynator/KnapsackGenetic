use rand::Rng;

use crate::individual::Individual;

pub enum CrossoverMethod {
    SinglePoint,
}

pub fn crossover_method<T>(
    parents: (&Individual<T>, &Individual<T>),
    method: &CrossoverMethod,
    crossover_rate: f64,
) -> (Individual<T>, Individual<T>)
where
    T: Clone,
{
    let mut rng = rand::thread_rng();

    if !rng.gen_bool(crossover_rate) {
        return (parents.0.clone(), parents.1.clone());
    }

    match method {
        CrossoverMethod::SinglePoint => single_point_crossover(parents),
    }
}

fn single_point_crossover<T>(
    parents: (&Individual<T>, &Individual<T>),
) -> (Individual<T>, Individual<T>)
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let chromosome_len = parents.0.chromosome.len();
    let crossover_point = rng.gen_range(0..chromosome_len);

    let mut child1 = parents.0.clone();
    let mut child2 = parents.1.clone();

    child1.chromosome[crossover_point..].copy_from_slice(&parents.1.chromosome[crossover_point..]);
    child2.chromosome[crossover_point..].copy_from_slice(&parents.0.chromosome[crossover_point..]);

    (child1, child2)
}
