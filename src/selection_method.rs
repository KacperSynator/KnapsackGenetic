use crate::individual::Individual;

use anyhow::Error;
use derive_more::{Display, Error};
use num_traits::Num;
use rand::distributions::uniform::SampleUniform;
use rand::seq::SliceRandom;
use rand::Rng;
use std::iter::Sum;
use std::ops::AddAssign;

#[derive(Debug, Display, Error)]
#[display(
    fmt = "Population size is smaller than \"tournament size\" or \"elites number\": {_0} < {_1} "
)]
struct PopulationSizeError(#[error(not(source))] usize, usize);

#[derive(Debug, Display, Error)]
#[display(fmt = "Roulette selection failed")]
struct RouletteError;

#[derive(Debug, Display, Error)]
#[display(fmt = "Secondary selection cannot be Elitism")]
struct InvalidSecondarySelectionError;

pub enum SelectionMethod {
    Tournament {
        size: usize,
    },
    Elitism {
        n_elites: usize,
        secondary_selection: Box<SelectionMethod>,
    },
    Roulette,
}

pub fn selection_method<T>(
    population: &Vec<Individual<T>>,
    method: &SelectionMethod,
) -> Result<Individual<T>, Error>
where
    T: Num + Ord + Clone + Sum + AddAssign + SampleUniform,
{
    match method {
        SelectionMethod::Tournament { size } => tournament_selection(population, *size),
        SelectionMethod::Roulette => roulette_selection(population),
        SelectionMethod::Elitism {
            n_elites: _,
            secondary_selection,
        } => handle_secondary_method(population, secondary_selection),
    }
}

pub fn select_elites<T>(
    population: &Vec<Individual<T>>,
    n_elites: usize,
) -> Result<Vec<Individual<T>>, Error>
where
    T: Num + Ord + Clone,
{
    if population.len() < n_elites {
        return Err(Error::from(PopulationSizeError(population.len(), n_elites)));
    }

    let mut cloned_population: Vec<Individual<T>> = population.to_vec();
    cloned_population.sort();
    cloned_population.reverse();
    let elites: Vec<Individual<T>> = cloned_population[..n_elites].to_vec();

    Ok(elites)
}

fn handle_secondary_method<T>(
    population: &Vec<Individual<T>>,
    secondary_method: &Box<SelectionMethod>,
) -> Result<Individual<T>, Error>
where
    T: Num + Ord + Clone + Sum + AddAssign + SampleUniform,
{
    if let SelectionMethod::Elitism { .. } = **secondary_method {
        return Err(Error::from(InvalidSecondarySelectionError));
    }

    Ok(selection_method(population, &secondary_method)?)
}

fn tournament_selection<T>(
    population: &Vec<Individual<T>>,
    tournament_size: usize,
) -> Result<Individual<T>, Error>
where
    T: Num + Ord + Clone,
{
    if population.len() < tournament_size {
        return Err(Error::from(PopulationSizeError(
            population.len(),
            tournament_size,
        )));
    }

    let mut rng = rand::thread_rng();

    let selected_individuals: Vec<Individual<T>> = population
        .choose_multiple(&mut rng, tournament_size)
        .cloned()
        .collect();

    let winner = selected_individuals.into_iter().max().unwrap();

    Ok(winner)
}

fn roulette_selection<T>(population: &Vec<Individual<T>>) -> Result<Individual<T>, Error>
where
    T: Num + Ord + Clone + Sum + AddAssign + SampleUniform,
{
    let mut rng = rand::thread_rng();
    let total_fitness = population
        .iter()
        .map(|individual| individual.fitness_score.clone())
        .sum();
    let random_number = rng.gen_range(T::zero()..total_fitness);

    let mut cumulative_fitness = T::zero();
    for individual in population.iter() {
        cumulative_fitness += individual.fitness_score.clone();
        if cumulative_fitness >= random_number {
            return Ok(individual.clone());
        }
    }

    Err(Error::from(RouletteError))
}
