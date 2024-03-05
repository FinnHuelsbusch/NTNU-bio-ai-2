

use crate::{config::initialize_config, problem_instance::initialize_problem_instance};

mod problem_instance;
mod patient;
mod depot;
mod config;
mod individual;
mod population;



fn main() {
    // Load config
    let config = initialize_config("./config.json");
    println!("file path: {:?}", config.problem_instance);
    let problem_instance = initialize_problem_instance(&config.problem_instance);
  

    
    
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
    
}
