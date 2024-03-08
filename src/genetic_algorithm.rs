use crate::{
    config::Config,
    crossover_functions::crossover,
    mutation_functions::mutate,
    population::{get_average_travel_time, initialize_population, Population},
    problem_instance::ProblemInstance,
    selection_functions::{parent_selection, survivor_selection},
};

use std::io;
use std::io::Write;

fn log_population_statistics(population: &Population) {
    let mut sorted_population: Population = population.clone();
    sorted_population.sort();

    println!(
        "Best: {:?} Avg: {:?} Worst: {:?}",
        sorted_population[0].travel_time,
        get_average_travel_time(population),
        sorted_population[sorted_population.len() - 1].travel_time
    )
}

pub fn run_genetic_algorithm_instance(problem_instance: &ProblemInstance, config: &Config) {
    let mut population: Population = initialize_population(problem_instance, config);

    for generation in 0..config.number_of_generations {
        println!("Calculating Generation: {:?}", generation);
        print!("SEL|");
        io::stdout().flush().unwrap();
        let mut parents = parent_selection(&population, config);

        print!("CROSS|");
        io::stdout().flush().unwrap();
        let mut children = crossover(&mut parents, problem_instance, config);

        print!("MUT|");
        io::stdout().flush().unwrap();
        children = mutate(&mut children, problem_instance, config);

        println!("SURV_SEL");
        io::stdout().flush().unwrap();
        population = survivor_selection(&parents, &children, config);

        log_population_statistics(&population);
    }
}
