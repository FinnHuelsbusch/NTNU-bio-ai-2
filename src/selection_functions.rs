use rand::Rng;

use crate::{ config::Config, population::{self, Population} };

fn roulette_wheel_selection(population: &Population, population_size: usize) -> Population {
    let mut new_population: Population = Vec::with_capacity(population_size);

    // Map fitness values
    let mut fitness_values: Vec<f64> = population
        .iter()
        .map(|individual| individual.fitness)
        .collect();

    // Get the minimal fitness
    let min_fitness: f64 = *fitness_values
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(&0.0);

    // Transform fitness to positive range
    fitness_values = fitness_values
        .iter_mut()
        .map(|fitness_value| *fitness_value - min_fitness + f64::EPSILON)
        .collect();

    let total_fitness: f64 = fitness_values.iter().sum();
    let mut rng = rand::thread_rng();

    for _ in 0..population_size {
        let selected_value = rng.gen::<f64>() * total_fitness;
        let mut sum = 0.0;

        for (index, individual) in population.iter().enumerate() {
            sum += fitness_values[index];
            if selected_value < sum {
                new_population.push(individual.clone());
                break;
            }
        }
    }

    new_population
}

fn full_replacement_selection(population: &Population, children: &Population, population_size: usize) -> Population {
    assert!(population.len() == population_size);
    assert!(children.len() == population_size);
    children.clone()
}

pub fn parent_selection(population: &Population, config: &Config) -> Population {
    match config.parent_selection.name.as_str() {
        // Match a single value
        "rouletteWheel" => roulette_wheel_selection(population, population.len()),
        // Handle the rest of cases
        _ =>
            panic!(
                "Didn't have an Implementation for selection function: {:?}",
                config.parent_selection.name.as_str()
            ),
    }
}

pub fn survivor_selection(parents: &Population, children: &Population, config: &Config) -> Population {
    let mut selection_population: Population; 
    if config.survivor_selection.combine_parents_and_offspring.unwrap_or(false) {
        let mut combined_population: Population = parents.clone();
        combined_population.extend(children.clone());
        selection_population = combined_population;
    }
    else {
        selection_population = children.clone();
    }
    selection_population.sort_unstable_by(|a, b| a.fitness.total_cmp(&b.fitness));
    
    let mut new_population: Population = Vec::with_capacity(config.population_size);
    let number_of_elites = (config.survivor_selection.elitism_percentage.unwrap_or(0.0) * config.population_size as f64).ceil() as usize;
    assert !(number_of_elites < config.population_size);
    if number_of_elites > 0 {
        new_population.extend(selection_population.iter().take(number_of_elites).cloned());
    }


    let selected_population: Population = match config.parent_selection.name.as_str() {
        // Match a single value
        "rouletteWheel" => roulette_wheel_selection(&selection_population, config.population_size - number_of_elites),
        "fullReplacement" => full_replacement_selection(parents, children, config.population_size - number_of_elites),
        _ =>
            panic!(
                "Didn't have an Implementation for selection function: {:?}",
                config.parent_selection.name.as_str()
            ),
    }; 
    new_population.extend(selected_population);
    return new_population;
}
