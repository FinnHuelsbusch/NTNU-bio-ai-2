use genetic_algorithm::run_genetic_algorithm_instance;

use crate::{config::initialize_config, problem_instance::initialize_problem_instance};

mod config;
mod crossover_functions;
mod depot;
mod genetic_algorithm;
mod individual;
mod mutation_functions;
mod patient;
mod population;
mod problem_instance;
mod selection_functions;

fn main() {
    // Load config
    let config = initialize_config("./config.json");
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
    // Load the specified problem instance
    let problem_instance = initialize_problem_instance(&config.problem_instance);

    run_genetic_algorithm_instance(&problem_instance, &config)
}
