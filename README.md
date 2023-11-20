# KnapsackGenetic
Genetic algorithm for solving Knapsack problem using different selection, crossover and mutation methods.

## Selection methods
For setting the selection method `selection_method` parameter is used.  
For available methods look below.

### Tournament
[Wikipedia - Tournament Selection](https://en.wikipedia.org/wiki/Tournament_selection#:~:text=Tournament%20selection%20is%20a%20method,at%20random%20from%20the%20population.)
```rust
selection_method: SelectionMethod::Tournament { size: 10 },
```

### Roulette (WIP)
[Wikipedia - Roulette Selection](https://en.wikipedia.org/wiki/Fitness_proportionate_selection)
```rust
selection_method: SelectionMethod::Roulette,
```

### Elitism (WIP)
[Wikipedia - Elitism Selection](https://en.wikipedia.org/wiki/Selection_(genetic_algorithm)#Elitist_Selection)
```rust
selection_method: SelectionMethod::Elitism { n_elites: 3, secondary_selection: Box::new(SelectionMethod::Roulette)},
```

## Crossover methods
For setting the crossover method `crossover_method` parameter is used and `crossover_rate` for probability of crossover. For available methods look below.

### Single Point
[Wikipedia - One-point crossover](https://en.wikipedia.org/wiki/Crossover_(genetic_algorithm)#One-point_crossover)
```rust
crossover_method: CrossoverMethod::SinglePoint,
```

### Multi Point
[Wikipedia - Two-point and k-point crossover](https://en.wikipedia.org/wiki/Crossover_(genetic_algorithm)#Two-point_and_k-point_crossover)
```rust
crossover_method: CrossoverMethod::MutltiPoint { n_points: 5 },
```

### Uniform
[Wikipedia - Uniform crossover](https://en.wikipedia.org/wiki/Crossover_(genetic_algorithm)#Uniform_crossover)
```rust
crossover_method: CrossoverMethod::Uniform,
```

## Mutation methods
For setting the crossover method `mutation_method` parameter is used and `mutation_rate` for probability of gene mutation. For available methods look below.

### Bit Flip
[Wikipedia - Bit string mutation](https://en.wikipedia.org/wiki/Mutation_(genetic_algorithm)#Bit_string_mutation)
```rust
mutation_method: MutationMethod::BitFlip,
```

### Swap
Swaps to random genes in chromosome
```rust
mutation_method: MutationMethod::Swap,
```

### Inversion
[Wikipedia - Inversion mutation](https://en.wikipedia.org/wiki/Mutation_(genetic_algorithm)#Inversion)
```rust
mutation_method: MutationMethod::Inversion,
```

## Build
```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# modify main for yor needs and run
export RUST_LOG=info  # "debug" for some more logs
cargo run
```
## Usage
Few prerequisites needs to be met:
- `weights` and `prices` needs to be of equal size/length
- `population_size` needs to be even and non-zero value
- `crossover_rate` and `mutation_rate` probabilities needs to be in range [0, 1]
- selection methods parameters like `Tournament {size}` or `Elitism {n_elites}` cannot be greater than `population_size`
- `secondary_selection` for `Elitism` selection method cannot be `Elitism`

### Graph plotting
A simple graph plotting is implemented using [plotters](https://docs.rs/plotters/latest/plotters/). To use it define a `GraphData` with graph options and then run 
`plot_graph(&result, &graph_data)` where `result` is the output of `genetic_algorithm`
function. Look at [example](#example) for further information.

### Example
Example usage can be found in [main.rs](src/main.rs):
```rust
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
const GENERATIONS: usize = 100;
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
```

Run using:
```bash
RUST_LOG=info cargo run --release
```

The console output shoud look like:
```bash
INFO  knapsack_genetic > Best chromosome: Individual { chromosome: [true, true, false, true, true, true, false, false, false, true, true, false, true, false, false, true, false, false, false, false, false, true, true, true], fitness_score: 13549094 }
```

And similar graph should be generated in `out.png`:
file:///home/kakkosbp/workspace/knapsack_genetic/out.png![image](https://github.com/KacperSynator/KnapsackGenetic/assets/62207289/e9987f2d-3914-4032-bb09-cac01926bcd8)

