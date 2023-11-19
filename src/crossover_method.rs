use std::usize;

use rand::Rng;

use crate::individual::Individual;

pub enum CrossoverMethod {
    SinglePoint,
    MultiPoint { n_points: usize },
    Uniform,
}

pub fn crossover_method<T>(
    parents: (&Individual<T>, &Individual<T>),
    method: &CrossoverMethod,
    crossover_rate: f64,
) -> (Individual<T>, Individual<T>)
where
    T: Clone + Default,
{
    let mut rng = rand::thread_rng();

    if !rng.gen_bool(crossover_rate) {
        return (parents.0.clone(), parents.1.clone());
    }

    match method {
        CrossoverMethod::SinglePoint => multi_point_crossover(parents, 1),
        CrossoverMethod::MultiPoint { n_points } => multi_point_crossover(parents, *n_points),
        CrossoverMethod::Uniform => uniform_crossover(parents),
    }
}

fn multi_point_crossover<T>(
    parents: (&Individual<T>, &Individual<T>),
    n_points: usize,
) -> (Individual<T>, Individual<T>)
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let chromosome_len = parents.0.chromosome.len();
    let mut crossover_points: Vec<usize> = (0..n_points)
        .map(|_| rng.gen_range(0..chromosome_len))
        .collect();
    crossover_points.sort();

    let mut child1 = parents.0.clone();
    let mut child2 = parents.1.clone();

    crossover_points.iter().for_each(|crossover_point| {
        child1.chromosome[*crossover_point..]
            .copy_from_slice(&parents.1.chromosome[*crossover_point..]);
        child2.chromosome[*crossover_point..]
            .copy_from_slice(&parents.0.chromosome[*crossover_point..]);
    });

    (child1, child2)
}

fn uniform_crossover<T>(parents: (&Individual<T>, &Individual<T>)) -> (Individual<T>, Individual<T>)
where
    T: Clone + Default,
{
    let mut rng = rand::thread_rng();
    let chromosome_len = parents.0.chromosome.len();

    let mut child1 = Individual {
        chromosome: Vec::with_capacity(chromosome_len),
        ..Default::default()
    };
    let mut child2 = Individual {
        chromosome: Vec::with_capacity(chromosome_len),
        ..Default::default()
    };

    parents
        .0
        .chromosome
        .iter()
        .zip(parents.1.chromosome.iter())
        .for_each(|genes| {
            if rng.gen_bool(0.5) {
                child1.chromosome.push(*genes.0);
                child2.chromosome.push(*genes.1);
            } else {
                child1.chromosome.push(*genes.1);
                child2.chromosome.push(*genes.0);
            }
        });

    (child1, child2)
}
