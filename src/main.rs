use genetic_algorithm::run_genetic_algorithm_instance;
use log::{ self, LevelFilter };

use crate::{ config::initialize_config, problem_instance::initialize_problem_instance };

use std::env;

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
    let args: Vec<String> = env::args().collect();
    let config_path : &str;
    if args.len() < 2 {
        config_path = "./configs/config.json";
    } else {
        config_path = &args[1];
    }
    // Load config
    let mut config = initialize_config(config_path);

    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    simple_logging::log_to_file(config.log_file.clone().unwrap_or("./python/statistics_rust.txt".parse().unwrap()), LevelFilter::Info);

    // Load the specified problem instance
    let problem_instance = initialize_problem_instance(&config.problem_instance);

    let best = run_genetic_algorithm_instance(&problem_instance, &mut config);

    let ouput_file = config.output_file.clone().unwrap_or("./python/solution.json".parse().unwrap());
    best.export_to_file(ouput_file.as_str());
}
