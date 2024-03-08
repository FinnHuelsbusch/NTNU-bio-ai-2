use log::info;

use crate::{
    config::{ self, Config },
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

use std::{ f64::INFINITY, io };
use std::io::Write;

fn log_population_statistics(generation: usize, population: &Population) {
    let mut sorted_population: Population = population.clone();
    // filter sorted_population to only include individuals with a feasible solution
    sorted_population.retain(|individual| individual.is_feasible());
    sorted_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

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
) -> Individual {
    let mut population: Population = initialize_population(problem_instance, original_config);
    let mut best_individual: Individual = population[0].clone();
    let mut config = original_config.clone();

    let mut delta = f64::MAX;
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
        }

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
    }
    println!("Best Individual: {:?}", best_individual);

    best_individual
}
