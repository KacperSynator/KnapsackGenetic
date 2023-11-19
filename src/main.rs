use knapsack_genetic::crossover_method::CrossoverMethod;
use knapsack_genetic::genetic_algorithm::{genetic_algorithm, GeneticAlgorithmData};
use knapsack_genetic::mutation_method::MutationMethod;
use knapsack_genetic::selection_method::SelectionMethod;

use knapsack_genetic::utils::{plot_graph, GraphData};
use log::{error, info};

fn run_and_plot_genetic_algorithm(
    data: &GeneticAlgorithmData<i32>,
    graph_data: &GraphData,
    n_optimal_results: &mut i32,
) {
    match genetic_algorithm(data) {
        Ok(result) => {
            if result.best_individual.fitness_score == 309 {
                *n_optimal_results += 1
            };
            let cr_mr = format!("cr: {}, mr: {}", data.crossover_rate, data.mutation_rate);
            info!("{cr_mr} => Best chromosome: {:?}", result.best_individual);
            if let Err(e) = plot_graph(&result, graph_data) {
                error!("Failed to plot with error: {e}");
            }
        }
        Err(e) => error!("Genetic algorithm failed with error: {e}"),
    }
}

// Other example optimal 48
// weights: vec![27, 10, 25, 25, 7],
// prices: vec![13, 19, 7, 16, 3],
// capacity: 66,

fn main() {
    pretty_env_logger::init();

    let optimal = 309;
    let n_reps = 1;
    let mut accuracy_table = String::from("cr\\mr =>\n");

    for crossover_rate in [0.1, 0.3, 0.5, 0.7, 0.9] {
        for mutation_rate in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let mut n_optimal_results = 0;
            for _ in 0..n_reps {
                let data = GeneticAlgorithmData {
                    weights: vec![23, 31, 29, 44, 53, 38, 63, 85, 89, 82],
                    prices: vec![92, 57, 49, 68, 60, 43, 67, 84, 87, 72],
                    capacity: 165, // optimal 309
                    population_size: 50,
                    generations: 50,
                    crossover_method: CrossoverMethod::SinglePoint,
                    crossover_rate: crossover_rate,
                    mutation_method: MutationMethod::Inversion,
                    mutation_rate: mutation_rate,
                    selection_method: SelectionMethod::Elitism { n_elites: 5, secondary_selection: Box::new(SelectionMethod::Roulette) },
                };

                let out_file = format!("graphs/cr{crossover_rate}mr{mutation_rate}.png");
                let title = format!("Cross={crossover_rate}, Mut={mutation_rate}");

                let graph_data = GraphData {
                    out_file: &out_file,
                    title: &title,
                    y_max_value: Some(optimal as f32 * 1.2),
                    optimal_value_line: Some(optimal as f32),
                    ..Default::default()
                };

                run_and_plot_genetic_algorithm(&data, &graph_data, &mut n_optimal_results);
            }
            println!();
            let accuracy = n_optimal_results as f32 / n_reps as f32;
            accuracy_table = format!("{accuracy_table} {} ", accuracy);
        }
        accuracy_table = format!("{accuracy_table}\n");
    }
    println!("{accuracy_table}");
}
