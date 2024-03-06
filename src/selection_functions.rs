use rand::Rng;

use crate::{ config::Config, population::Population };

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

pub fn parent_selection(population: &Population, config: &Config) -> Population {
    match config.parent_selection.name.as_str() {
        // Match a single value
        "roulette" => roulette_wheel_selection(population, population.len()),
        // Handle the rest of cases
        _ =>
            panic!(
                "Didn't have an Implementation for selection function: {:?}",
                config.parent_selection.name.as_str()
            ),
    }
}

pub fn survivor_selection(parents: &Population, config: &Config) -> Population {
    match config.parent_selection.name.as_str() {
        // Match a single value
        // "roulette" => roulette_wheel_selection(population, population.len()),
        // Handle the rest of cases
        _ =>
            panic!(
                "Didn't have an Implementation for selection function: {:?}",
                config.parent_selection.name.as_str()
            ),
    }
}
