use knapsack_genetic::crossover_method::CrossoverMethod;
use knapsack_genetic::genetic_algorithm::{genetic_algorithm, GeneticAlgorithmData};
use knapsack_genetic::mutation_method::MutationMethod;
use knapsack_genetic::selection_method::SelectionMethod;

use log::{error, info};

fn main() {
    pretty_env_logger::init();

    let data = GeneticAlgorithmData {
        weights: vec![27, 10, 25, 25, 7],
        prices: vec![13, 19, 7, 16, 3],
        capacity: 66,
        population_size: 50,
        generations: 10,
        crossover_method: CrossoverMethod::SinglePoint,
        crossover_rate: 0.5,
        mutation_method: MutationMethod::BitFlip,
        mutation_rate: 0.05,
        selection_method: SelectionMethod::Tournament { size: 10 },
    };

    match genetic_algorithm(&data) {
        Ok(result) => info!("Best chromosome: {:?}", result),
        Err(e) => error!("Genetic algorithm failed with error: {e}"),
    }
}
