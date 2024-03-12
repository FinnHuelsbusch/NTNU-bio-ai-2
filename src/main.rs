use genetic_algorithm::run_genetic_algorithm_instance;
use log::{ self, LevelFilter };
use core::num;
use std::io::{Write};

use crate::config::MetaConfig;
use crate::{ config::initialize_config, problem_instance::initialize_problem_instance };

use std::{env, thread};
use crate::genetic_algorithm::{run_genetic_algorithm, Statistics};
use crate::individual::Individual;

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
    let config_type = initialize_config(config_path);
    let output: (Individual, Statistics);
    let output_file: String;
    
    match config_type { 
        config::ConfigType::Config(mut config) => {
            println!("{}", serde_json::to_string_pretty(&config).unwrap());

            simple_logging::log_to_file(config.log_file.clone().unwrap_or("./python/statistics_rust.txt".parse().unwrap()), LevelFilter::Info);

            // Load the specified problem instance
            let problem_instance = initialize_problem_instance(&config.problem_instance);
      
    


            output = run_genetic_algorithm(&problem_instance, &mut config);
            output_file = config.output_file.clone().unwrap_or("./python/solution.json".parse().unwrap());


            
        },
        config::ConfigType::MetaConfig (meta_config) => {
            // sum up the number of instances 
            let number_of_threads: usize = meta_config.configs.len();
            let mut handles: Vec<thread::JoinHandle<(Individual, Statistics)>> = Vec::with_capacity(number_of_threads);
            output_file = meta_config.output_file.clone().unwrap_or("./python/solution.json".parse().unwrap());
            for mut config_instance in meta_config.configs {
                let handle = thread::spawn(move || {
                    // Mute the stdout within the thread
                    let output  = run_genetic_algorithm_instance(&initialize_problem_instance(&config_instance.problem_instance), &mut config_instance);
                    (output.0, output.1)
                });
                handles.push(handle);
            }
            
            let mut results: Vec<(Individual, Statistics)> = Vec::with_capacity(number_of_threads);
            println!("Waiting for threads to finish");
            for handle in handles {
                results.push(handle.join().unwrap());
            }
            println!("Threads finished");
            // sort the results by fitness
            let mut sorted_results: Vec<(Individual, Statistics)> = Vec::with_capacity(number_of_threads);
            for result in results.iter() {
                sorted_results.push(result.clone());
            }
            sorted_results.sort_unstable_by(|a, b| a.0.fitness.partial_cmp(&b.0.fitness).unwrap());
            for result in sorted_results.iter() {
                let config_index = results.iter().position(|x| x.0.fitness == result.0.fitness).unwrap();
                println!("Config: {:?} Fitness: {:?}", config_index, result.0.fitness);
            }
            
            output = results[0].clone();
        }
    }

    
    // Write output to file
    let json_string = serde_json::to_string(&output).unwrap();
    std::fs::write(output_file, json_string).expect("Unable to write to file");
    
}
