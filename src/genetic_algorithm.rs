use anyhow::Error;
use derive_more::{Display, Error};
use log::debug;
use num_traits::Num;
use rand::Rng;

use crate::crossover_method::{crossover_method, CrossoverMethod};
use crate::individual::Individual;
use crate::mutation_method::{mutation_method, MutationMethod};
use crate::selection_method::{selection_method, SelectionMethod};

const BOOL_PROBABILITY: f64 = 0.5;

#[derive(Debug, Display, Error)]
#[display(fmt = "Weights and prices dimensions are not equal: {_0} != {_1} ")]
struct DimensionsError(#[error(not(source))] usize, usize);

#[derive(Debug, Display, Error)]
#[display(fmt = "The probability of {_0} ({_1}) is not in range of [0 - 1]")]
struct ProbabilityRangeError(#[error(not(source))] String, f64);

#[derive(Debug, Display, Error)]
#[display(fmt = "population_size ({_0}) must be even and non-zero value")]
struct PopulationSizeError(#[error(not(source))] usize);

pub struct GeneticAlgorithmData<T>
where
    T: Num,
{
    pub weights: Vec<T>,
    pub prices: Vec<T>,
    pub capacity: T,
    pub population_size: usize,
    pub generations: usize,
    pub crossover_method: CrossoverMethod,
    pub crossover_rate: f64,
    pub mutation_method: MutationMethod,
    pub mutation_rate: f64,
    pub selection_method: SelectionMethod,
}

#[derive(Debug)]
pub struct GeneticAlgorithmResultData<T>
where
    T: Num,
{
    pub best_individual: Individual<T>,
    pub score_per_generation: Vec<T>,
}

pub fn genetic_algorithm<T>(
    data: &GeneticAlgorithmData<T>,
) -> Result<GeneticAlgorithmResultData<T>, Error>
where
    T: Num + std::fmt::Debug + Default + for<'a> std::iter::Sum<&'a T> + PartialOrd + Ord + Clone,
{
    validate_data(data)?;

    let data_length = data.weights.len();
    let mut population = generate_random_population::<T>(data_length, data.population_size);
    debug!(
        "Generated population [{}]: {:?}",
        population.len(),
        population
    );

    let mut result = GeneticAlgorithmResultData {
        best_individual: find_best_individual(&population),
        score_per_generation: Vec::new(),
    };

    result.score_per_generation.reserve(data.population_size);

    for _ in 0..data.generations {
        population
            .iter_mut()
            .for_each(|individual| individual.fitness_score = calculate_fitness(data, individual));

        let current_best_individual = find_best_individual(&population);
        if current_best_individual > result.best_individual {
            result.best_individual = current_best_individual.clone();
        }

        result
            .score_per_generation
            .push(current_best_individual.fitness_score);

        population = generate_new_population(data, &population)?;
    }

    Ok(result)
}

fn validate_data<T>(data: &GeneticAlgorithmData<T>) -> Result<(), Error>
where
    T: Num,
{
    if data.weights.len() != data.prices.len() {
        return Err(Error::from(DimensionsError(
            data.weights.len(),
            data.prices.len(),
        )));
    }

    if data.crossover_rate > 1.0 || data.crossover_rate < 0.0 {
        return Err(Error::from(ProbabilityRangeError(
            "crossover_rate".to_string(),
            data.crossover_rate,
        )));
    }

    if data.mutation_rate > 1.0 || data.mutation_rate < 0.0 {
        return Err(Error::from(ProbabilityRangeError(
            "mutation_rate".to_string(),
            data.mutation_rate,
        )));
    }

    if data.population_size % 2 != 0 || data.population_size == 0 {
        return Err(Error::from(PopulationSizeError(data.population_size)));
    }

    Ok(())
}

fn generate_random_population<T>(data_length: usize, population_size: usize) -> Vec<Individual<T>>
where
    T: Num + Default,
{
    (0..population_size)
        .map(|_| generate_random_individual(data_length))
        .collect()
}

fn generate_random_individual<T>(data_length: usize) -> Individual<T>
where
    T: Num + Default,
{
    let mut rng = rand::thread_rng();
    Individual {
        chromosome: (0..data_length)
            .map(|_| rng.gen_bool(BOOL_PROBABILITY))
            .collect(),
        fitness_score: T::default(),
    }
}

fn calculate_fitness<T>(data: &GeneticAlgorithmData<T>, individual: &Individual<T>) -> T
where
    T: Num + Default + for<'a> std::iter::Sum<&'a T> + std::cmp::PartialOrd,
{
    let total_weight: T = individual
        .chromosome
        .iter()
        .zip(data.weights.iter())
        .filter(|(&is_taken, _)| is_taken)
        .map(|(_, weight)| weight)
        .sum();

    if total_weight > data.capacity {
        return T::default();
    }

    individual
        .chromosome
        .iter()
        .zip(data.prices.iter())
        .filter(|(&is_taken, _)| is_taken)
        .map(|(_, price)| price)
        .sum()
}

fn find_best_individual<T>(population: &[Individual<T>]) -> Individual<T>
where
    T: Num + Ord + Clone,
{
    population.iter().max().unwrap().clone()
}

fn generate_new_population<T>(
    data: &GeneticAlgorithmData<T>,
    population: &Vec<Individual<T>>,
) -> Result<Vec<Individual<T>>, Error>
where
    T: Num + Ord + Clone,
{
    let mut new_population = Vec::new();
    new_population.reserve(data.population_size);

    while new_population.len() < data.population_size {
        let parents = (
            &selection_method(population, &data.selection_method)?,
            &selection_method(population, &data.selection_method)?,
        );

        let children = crossover_method(parents, &data.crossover_method, data.crossover_rate);
        let children = (
            mutation_method(&children.0, &data.mutation_method, data.mutation_rate),
            mutation_method(&children.1, &data.mutation_method, data.mutation_rate),
        );

        new_population.push(children.0);
        new_population.push(children.1);
    }

    Ok(new_population)
}
