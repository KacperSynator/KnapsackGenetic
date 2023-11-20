use knapsack_genetic::crossover_method::CrossoverMethod;
use knapsack_genetic::genetic_algorithm::{genetic_algorithm, GeneticAlgorithmData};
use knapsack_genetic::mutation_method::MutationMethod;
use knapsack_genetic::selection_method::SelectionMethod;

use knapsack_genetic::utils::{plot_graph, GraphData};
use log::{error, info};

// Data from (P08) https://people.sc.fsu.edu/~jburkardt/datasets/knapsack_01/knapsack_01.html
const WEIGHTS: &[i32] = &[
    382745, 799601, 909247, 729069, 467902, 44328, 34610, 698150, 823460, 903959, 853665, 551830,
    610856, 670702, 488960, 951111, 323046, 446298, 931161, 31385, 496951, 264724, 224916, 169684,
];
const PRICES: &[i32] = &[
    825594, 1677009, 1676628, 1523970, 943972, 97426, 69666, 1296457, 1679693, 1902996, 1844992,
    1049289, 1252836, 1319836, 953277, 2067538, 675367, 853655, 1826027, 65731, 901489, 577243,
    466257, 369261,
];
const CAPACITY: i32 = 6404180;
const OPTIMAL: i32 = 13549094;

const POPULATION_SIZE: usize = 100;
const GENERATIONS: usize = 1000;
const CROSSOVER_METHOD: CrossoverMethod = CrossoverMethod::MultiPoint { n_points: 2 };
const CROSSOVER_RATE: f64 = 0.5;
const MUTATATION_METHOD: MutationMethod = MutationMethod::BitFlip;
const MUTATION_RATE: f64 = 0.1;

fn main() {
    pretty_env_logger::init();

    let data = GeneticAlgorithmData {
        weights: WEIGHTS.to_vec(),
        prices: PRICES.to_vec(),
        capacity: CAPACITY,
        population_size: POPULATION_SIZE,
        generations: GENERATIONS,
        crossover_method: CROSSOVER_METHOD,
        crossover_rate: CROSSOVER_RATE,
        mutation_method: MUTATATION_METHOD,
        mutation_rate: MUTATION_RATE,
        selection_method: SelectionMethod::Elitism {
            n_elites: 1,
            secondary_selection: Box::new(SelectionMethod::Tournament { size: 10 }),
        },
    };

    let graph_data = GraphData {
        y_label_area_size: 60,
        y_max_value: Some(OPTIMAL as f32 * 1.2),
        optimal_value_line: Some(OPTIMAL as f32),
        ..Default::default()
    };

    match genetic_algorithm(&data) {
        Ok(result) => {
            info!("Best chromosome: {:?}", &result.best_individual);
            if let Err(e) = plot_graph(&result, &graph_data) {
                error!("Failed to plot with error: {e}");
            }
        }

        Err(e) => error!("Genetic algorithm failed with error: {e}"),
    }
}
