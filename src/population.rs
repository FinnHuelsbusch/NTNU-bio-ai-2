use std::collections::HashMap;
use rand::{ Rng, thread_rng };
use rand::seq::SliceRandom;
use crate::{
    config::Config,
    individual::{ calculate_fitness, is_journey_valid, Genome, Individual },
    patient::Patient,
    problem_instance::ProblemInstance,
};

pub type Population = Vec<Individual>;

pub fn get_average_fitness(population: &Population) -> f64 {
    let sum: f64 = population
        .iter()
        .map(|individual| individual.fitness)
        .sum();

    // Calculate the average
    let average = if population.is_empty() {
        0.0 // return 0 if the vector is empty to avoid division by zero
    } else {
        sum / (population.len() as f64)
    };

    average
}

fn initialize_random_population(problem_instance: &ProblemInstance, config: &Config) -> Population {
    let mut population = Vec::with_capacity(config.population_size);

    for _ in 0..config.population_size {
        let patient_ids: Vec<usize> = problem_instance.patients.keys().copied().collect();
        let mut genome: Genome = vec![Vec::new(); problem_instance.number_of_nurses];

        for &patient_id in patient_ids.iter() {
            let random_index = thread_rng().gen_range(0..problem_instance.number_of_nurses);
            genome[random_index].push(patient_id);
        }

        let mut individual = Individual {
            genome,
            fitness: 0.0,
            missing_care_time_penalty: 0.0,
            capacity_penalty: 0.0,
            to_late_to_depot_penality: 0.0,
        };
        calculate_fitness(&mut individual, &problem_instance);
        population.push(individual);
    }

    population
}

// TODO debattieren, ob wir das so machen wollen
// ist der rekursive aufruf in der funktion initialize_valid_population sinnvoll?
// sollten wir diese appendHeuristic verwenden oder eine bessere Heuristic verwenden? (vorteil ist, dass das hier schneller ist. Ein insert braucht O(number of nurses). Bessere hätten O(number of patients * 0.5)
fn initialize_append_heuristic_population(
    problem_instance: &ProblemInstance,
    config: &Config
) -> Population {
    let mut patients_by_end_time: HashMap<u32, Vec<Patient>> = HashMap::new();
    // fill patients_by_end_time
    for patient in problem_instance.patients.values() {
        let end_time = patient.end_time;
        if patients_by_end_time.contains_key(&end_time) {
            patients_by_end_time.get_mut(&end_time).unwrap().push(patient.clone());
        } else {
            patients_by_end_time.insert(end_time, vec![patient.clone()]);
        }
    }

    // get the keys in ascending order
    let mut end_times: Vec<u32> = patients_by_end_time.keys().cloned().collect();
    end_times.sort();

    let mut population: Population = Vec::with_capacity(config.population_size);
    while population.len() < config.population_size {
        let mut genome: Genome = vec![Vec::new(); problem_instance.number_of_nurses];
        let mut broken = false;
        'outer: for end_time in &end_times {
            let mut patients = patients_by_end_time.get(&end_time).unwrap().clone();
            let mut rng = rand::thread_rng();
            patients.shuffle(&mut rng);
            for patient in patients {
                let mut smallest_detour = f64::INFINITY;
                let mut best_position = 0;
                for (i, journey) in genome.iter().enumerate() {
                    let detour: f64; // Declare the detour variable
                    if journey.is_empty() {
                        detour =
                            problem_instance.travel_time[0][patient.id] +
                            problem_instance.travel_time[patient.id][0];
                    } else {
                        detour =
                            problem_instance.travel_time[journey[journey.len() - 1]][patient.id] +
                            problem_instance.travel_time[patient.id][0] -
                            problem_instance.travel_time[journey[journey.len() - 1]][0];
                    }
                    if detour < smallest_detour {
                        let mut updated_journey = journey.clone();
                        updated_journey.push(patient.id);
                        if is_journey_valid(&updated_journey, &problem_instance) {
                            smallest_detour = detour;
                            best_position = i;
                        }
                    }
                }
                if smallest_detour <= f64::INFINITY {
                    genome[best_position].push(patient.id);
                } else {
                    broken = true;
                    break 'outer;
                }
            }
        }
        if broken {
            continue;
        } else {
            let mut individual = Individual {
                genome,
                fitness: 0.0,
                missing_care_time_penalty: 0.0,
                capacity_penalty: 0.0,
                to_late_to_depot_penality: 0.0,
            };
            calculate_fitness(&mut individual, &problem_instance);
            population.push(individual);
        }
    }

    return population;
}

pub fn initialize_population(problem_instance: &ProblemInstance, config: &Config) -> Population {
    match config.population_initialisation.as_str() {
        "random" => initialize_random_population(problem_instance, config),
        "appendHeuristic" => initialize_append_heuristic_population(problem_instance, config),

        _ =>
            panic!(
                "Didn't have an Implementation for population intialisation: {:?}",
                config.population_initialisation
            ),
    }
}
