use crossbeam::thread;
use log::{ info, warn };

use crate::{
    config::{self, Config},
    crossover_functions::crossover,
    individual::{self, Individual},
    mutation_functions::mutate,
    population::{ get_average_fitness, get_average_travel_time, initialize_population, Population },
    problem_instance::{ self, should_early_stop, ProblemInstance },
    selection_functions::{ parent_selection, survivor_selection },
};

use std::{ io, sync::{mpsc, Arc, Mutex}, thread::{spawn, JoinHandle} };
use std::io::Write;
use serde::Serialize;

fn log_population_statistics(generation: usize, population: &Population) {
    let mut feasible_population: Population = population.clone();
    // filter sorted_population to only include individuals with a feasible solution
    feasible_population.retain(|individual| individual.is_feasible());
    if feasible_population.is_empty() {
        warn!("No feasible solutions in the population. No statistics to log.");
        println!("No feasible solutions in the population. No statistics to log.");
    }

    // Travel time statistics
    // sort population by fitness
    let mut sorted_population = population.clone();
    sorted_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    feasible_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    println!("Travel Time statistics:");
    println!("{:<30} {:<15} {:<15} {:<15}", "Statistic", "Best", "Avg", "Worst");

    if !feasible_population.is_empty() {
        println!(
            "{:<30} {:<15.2} {:<15.2} {:<15.2}",
            "Feasible Population",
            feasible_population[0].travel_time,
            get_average_travel_time(&feasible_population),
            feasible_population[feasible_population.len() - 1].travel_time
        );
    }

    println!(
        "{:<30} {:<15.2} {:<15.2} {:<15.2}",
        "Population",
        sorted_population[0].travel_time,
        get_average_travel_time(&sorted_population),
        sorted_population[sorted_population.len() - 1].travel_time
    );

    // Fitness statistics
    // sort population by fitness
    sorted_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    feasible_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    println!("Fitness statistics:");

    if !feasible_population.is_empty() {
        println!(
            "{:<30} {:<15.2} {:<15.2} {:<15.2}",
            "Feasible Population",
            feasible_population[0].fitness,
            get_average_fitness(&feasible_population),
            feasible_population[feasible_population.len() - 1].fitness
        );
    }

    println!(
        "{:<30} {:<15.2} {:<15.2} {:<15.2} \n",
        "Population",
        sorted_population[0].fitness,
        get_average_fitness(&sorted_population),
        sorted_population[sorted_population.len() - 1].fitness
    );

    let min_travel_time_individual = population
        .iter()
        .min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
        .unwrap();
    info!(
        "Travel Time Best: {:?} Avg: {:?} Worst: {:?}",
        sorted_population[0].travel_time,
        get_average_travel_time(population),
        sorted_population[sorted_population.len() - 1].travel_time
    );
    info!(
        "Fitness Best: {:?} Avg: {:?} Worst: {:?}",
        sorted_population[0].fitness,
        get_average_fitness(population),
        sorted_population[sorted_population.len() - 1].fitness
    );
    info!("Genome: Name: Fastest Generation: {:?} Genome: {:?}", generation, population[0].genome);
    info!(
        "Genome: Name: Fittest Generation: {:?} Genome: {:?}",
        generation,
        min_travel_time_individual.genome
    );
}

fn cool_down_config(generation: usize, config: &mut Config) {
    for crossover_config in config.crossovers.iter_mut() {
        if let Some(delta) = crossover_config.annealing_delta {
            crossover_config.probability = Some(
                (
                    crossover_config.probability.unwrap_or(0.0) +
                    delta * ((generation as f64) + 1.0).log(2.0)
                ).clamp(0.0, 1.0)
            );
        }
    }

    for mutation_config in config.mutations.iter_mut() {
        if let Some(delta) = mutation_config.annealing_delta {
            mutation_config.probability = Some(
                (
                    mutation_config.probability.unwrap_or(0.0) +
                    delta * ((generation as f64) + 1.0).log(2.0)
                ).clamp(0.0, 1.0)
            );
        }
    }
}

pub fn run_genetic_algorithm_instance(
    problem_instance: &ProblemInstance,
    original_config: &mut Config
) -> (Individual, Statistics) {
    let mut population: Population = initialize_population(problem_instance, original_config);
    let mut best_individual: Individual = population[0].clone();
    let mut config = original_config.clone();

    let mut delta;
    let mut last = -f64::MAX;

    for generation in 0..config.number_of_generations {
        println!("Calculating Generation: {:?}", generation);

        print!("SEL|");
        io::stdout().flush().unwrap();
        let mut parents = parent_selection(&population, &config);

        print!("CROSS|");
        io::stdout().flush().unwrap();
        let mut children = crossover(&mut parents, problem_instance, &config);

        print!("MUT|");
        io::stdout().flush().unwrap();
        children = mutate(&mut children, problem_instance, &config);

        println!("SURV_SEL");
        io::stdout().flush().unwrap();
        population = survivor_selection(&parents, &children, &config);

        population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        if population[0].travel_time < best_individual.travel_time && population[0].is_feasible() {
            best_individual = population[0].clone();
            info!("New best individual. Genome: {:?}", best_individual.genome);
        }

        // Annealing
        cool_down_config(generation, &mut config);
        if generation % 10 == 0 {
            let avg = get_average_travel_time(&population);
            delta = (last - avg).abs();
            last = avg;
            if delta < 10.0 {
                println!("Delta {:?} Reheating", delta);
                config = original_config.clone();
            }
        }

        log_population_statistics(generation, &population);

        // // Early stopping
        // if should_early_stop(best_individual.fitness, &problem_instance) {
        //     println!("Reached benchmark! Stopping");
        //     break;
        // }
    }
    println!("Best Individual: {:?}", best_individual);

    let mut feasible_population: Population = population.clone();
    // filter sorted_population to only include individuals with a feasible solution
    feasible_population.retain(|individual| individual.is_feasible());
    if feasible_population.is_empty() {
        warn!("No feasible solutions in the population. No Statistics to log.");
        println!("No feasible solutions in the population. No Statistics to log.");
    }

    // Travel time Statistics
    // sort population by fitness

    let mut statistics: Statistics = Statistics {
        travel_time_min_feasible: 0.0,
        travel_time_avg_feasible: 0.0,
        travel_time_max_feasible: 0.0,
        travel_time_min_all: 0.0,
        travel_time_avg_all: 0.0,
        travel_time_max_all: 0.0,
        fitness_min_feasible: 0.0,
        fitness_avg_feasible: 0.0,
        fitness_max_feasible: 0.0,
        fitness_min_all: 0.0,
        fitness_avg_all: 0.0,
        fitness_max_all: 0.0,
    };

    let mut sorted_population = population.clone();
    sorted_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    feasible_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    if !feasible_population.is_empty() {
        statistics.fitness_min_feasible = feasible_population[0].fitness;
        statistics.fitness_avg_feasible = get_average_fitness(&feasible_population);
        statistics.fitness_max_feasible = feasible_population[
            feasible_population.len() - 1
        ].fitness;
        statistics.fitness_min_all = sorted_population[0].fitness;
        statistics.fitness_avg_all = get_average_fitness(&sorted_population);
        statistics.fitness_max_all = sorted_population[sorted_population.len() - 1].fitness;
    } else {
        statistics.fitness_min_all = sorted_population[0].fitness;
        statistics.fitness_avg_all = get_average_fitness(&sorted_population);
        statistics.fitness_max_all = sorted_population[sorted_population.len() - 1].fitness;
    }

    // sort population by fitness
    sorted_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    feasible_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    if !feasible_population.is_empty() {
        statistics.travel_time_min_feasible = feasible_population[0].travel_time;
        statistics.travel_time_avg_feasible = get_average_travel_time(&feasible_population);
        statistics.travel_time_max_feasible = feasible_population[
            feasible_population.len() - 1
        ].travel_time;
        statistics.travel_time_min_all = sorted_population[0].travel_time;
        statistics.travel_time_avg_all = get_average_travel_time(&sorted_population);
        statistics.travel_time_max_all = sorted_population[sorted_population.len() - 1].travel_time;
    } else {
        statistics.travel_time_min_all = sorted_population[0].travel_time;
        statistics.travel_time_avg_all = get_average_travel_time(&sorted_population);
        statistics.travel_time_max_all = sorted_population[sorted_population.len() - 1].travel_time;
    }

    (best_individual, statistics)
}

pub fn run_genetic_algorithm(
    problem_instance: &ProblemInstance,
    original_config: &mut Config
) -> (Individual, Statistics) {
    // Define some data and a reference to it

    let number_of_threads = num_cpus::get() - 1;

    let result = Arc::new(Mutex::new(Vec::new()));

     // Launch multiple threads in parallel
     thread::scope(|s| {
        for _ in 0..number_of_threads {
            let arc_struct1 = problem_instance.clone();
            let mut arc_struct2 = original_config.clone();
            let result = Arc::clone(&result);

            // Spawn a new thread
            s.spawn(move |_| {
                // Call the function with the cloned structs
                let tuple_result = run_genetic_algorithm_instance(&arc_struct1, &mut arc_struct2);

                // Lock the result vector and push the tuple result
                let mut result = result.lock().unwrap();
                result.push(tuple_result);
            });
        }
    })
    .unwrap(); // Wait for all threads to finish

    let final_result = result.lock().unwrap().clone();

    let mut sorted_results: Vec<(Individual, Statistics)> = Vec::with_capacity(number_of_threads);
    for result in final_result.iter() {
        sorted_results.push(result.clone());
    }
    sorted_results.sort_unstable_by(|a, b| b.0.fitness.partial_cmp(&a.0.fitness).unwrap());
    for result in sorted_results.iter() {
        let config_index = final_result.iter().position(|x| x.0.fitness == result.0.fitness).unwrap();
        println!("Config: {:?} Fitness: {:?}", config_index, result.0.fitness);
    }



    sorted_results[0].clone()
}

#[derive(Debug, Serialize, Clone)]
pub struct Statistics {
    pub travel_time_min_feasible: f64,
    pub travel_time_avg_feasible: f64,
    pub travel_time_max_feasible: f64,
    pub travel_time_min_all: f64,
    pub travel_time_avg_all: f64,
    pub travel_time_max_all: f64,
    pub fitness_min_feasible: f64,
    pub fitness_avg_feasible: f64,
    pub fitness_max_feasible: f64,
    pub fitness_min_all: f64,
    pub fitness_avg_all: f64,
    pub fitness_max_all: f64,
}
