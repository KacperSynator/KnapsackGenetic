use crate::individual::Individual;

use anyhow::Error;
use derive_more::{Display, Error};
use num_traits::Num;
use rand::seq::SliceRandom;

#[derive(Debug, Display, Error)]
#[display(
    fmt = "Population size is smaller than \"tournament size\" or \"elites number\": {_0} < {_1} "
)]
struct PopulationSizeError(#[error(not(source))] usize, usize);

#[derive(Debug, Display, Error)]
#[display(fmt = "{_0} selection method is not implemented yet")]
struct NotImplementedError(#[error(not(source))] String);

pub enum SelectionMethod {
    Tournament { size: usize },
    Elitism { n_elites: usize },
    Roulette,
}

pub fn selection_method<T>(
    population: &Vec<Individual<T>>,
    method: &SelectionMethod,
) -> Result<Individual<T>, Error>
where
    T: Num + Ord + Clone,
{
    match method {
        SelectionMethod::Tournament { size } => tournament_selection(population, *size),
        SelectionMethod::Roulette => Err(Error::from(NotImplementedError("Roulette".to_string()))),
        SelectionMethod::Elitism { n_elites: _ } => {
            Err(Error::from(NotImplementedError("Elitism".to_string())))
        }
    }
}

fn elitism_selection<T>(
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
