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
selection_method: SelectionMethod::Elitism { n_elites: 3},
```

## Crossover methods
For setting the crossover method `crossover_method` parameter is used and `crossover_rate` for probability of crossover. For available methods look below.

### Single Point
[Wikipedia - One-point crossover](https://en.wikipedia.org/wiki/Crossover_(genetic_algorithm)#One-point_crossover)
```rust
crossover_method: CrossoverMethod::SinglePoint,
```

## Mutation methods
For setting the crossover method `mutation_method` parameter is used and `mutation_rate` for probability of gene mutation. For available methods look below.

### Bit Flip
[Wikipedia - Bit string mutation](https://en.wikipedia.org/wiki/Mutation_(genetic_algorithm)#Bit_string_mutation)
```rust
mutation_method: MutationMethod::BitFlip,
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

### Example
Example usage can be found in [main.rs](src/main.rs):
```rust
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

```
The console output shoud look like:
```bash
 INFO  knapsack_genetic > Best chromosome: Individual { chromosome: [false, true, true, true, false], fitness_score: 48 }
```
