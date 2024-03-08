use log::info;

use crate::{
    config::Config,
    crossover_functions::crossover,
    individual::{ self, Individual },
    mutation_functions::mutate,
    population::{
        self,
        get_average_fitness,
        get_average_travel_time,
        initialize_population,
        Population,
    },
    problem_instance::ProblemInstance,
    selection_functions::{ parent_selection, survivor_selection },
};

use std::io;
use std::io::Write;

fn log_population_statistics(generation: usize, population: &Population) {
    let mut sorted_population: Population = population.clone();
    // filter sorted_population to only include individuals with a feasible solution
    sorted_population = sorted_population
        .iter()
        .filter(|individual| individual.is_feasible())
        .cloned()
        .collect();
    sorted_population.sort();

    println!(
        "Best: {:?} Avg: {:?} Worst: {:?}",
        sorted_population[0].travel_time,
        get_average_travel_time(population),
        sorted_population[sorted_population.len() - 1].travel_time
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

pub fn run_genetic_algorithm_instance(
    problem_instance: &ProblemInstance,
    config: &Config
) -> Individual {
    let mut population: Population = initialize_population(problem_instance, config);
    let mut best_individual: Individual = population[0].clone();

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

        population.sort();
        if population[0].travel_time < best_individual.travel_time  && population[0].is_feasible(){
            best_individual = population[0].clone();
        }

        log_population_statistics(generation, &population);
    }
    println!("Best Individual: {:?}", best_individual);

    best_individual
}
