use log::warn;
use crate::{
    config::Config,
    individual::{calculate_fitness, is_journey_valid, Genome, Individual, Journey},
    population::Population,
    problem_instance::ProblemInstance,
};
use rand::Rng;

fn reassign_one_patient(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut source_nurse: usize;
    loop {
        source_nurse = rng.gen_range(0..problem_instance.number_of_nurses);
        if target_genome[source_nurse].len() > 0 {
            break;
        }
    }
    let source_patient_index: usize = rng.gen_range(0..target_genome[source_nurse].len());
    let patient = target_genome[source_nurse].remove(source_patient_index);
    // patient is removed from the source nurse before the target nurse and index is selected to avoid the case where the source and target nurse are the same and the patient is inserted at an index that is out of bounds
    let target_nurse: usize = rng.gen_range(0..problem_instance.number_of_nurses);
    let target_patient_index: usize = rng.gen_range(0..=target_genome[target_nurse].len());
    target_genome[target_nurse].insert(target_patient_index, patient);
    return target_genome;
}

fn swap_within_journey(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut source_nurse: usize;
    loop {
        source_nurse = rng.gen_range(0..problem_instance.number_of_nurses);
        if target_genome[source_nurse].len() > 1 {
            break;
        }
    }
    let patient_index_a: usize = rng.gen_range(0..target_genome[source_nurse].len());
    let patient_index_b: usize = rng.gen_range(0..target_genome[source_nurse].len());
    let patient_a = target_genome[source_nurse][patient_index_a];
    let patient_b = target_genome[source_nurse][patient_index_b];
    target_genome[source_nurse][patient_index_a] = patient_b;
    target_genome[source_nurse][patient_index_b] = patient_a;
    return target_genome;
}

fn swap_between_journeys(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut nurse_a: usize;
    loop {
        nurse_a = rng.gen_range(0..problem_instance.number_of_nurses);
        if target_genome[nurse_a].len() > 0 {
            break;
        }
    }
    let mut nurse_b: usize = usize::MAX;
    for (i, journey) in genome.iter().enumerate() {
        if i != nurse_a && journey.len() > 0 {
            nurse_b = i;
            break;
        }
    }
    if nurse_b == usize::MAX {
        warn!("No nurse with patients found to swap with nurse {}", nurse_a);
        return genome.clone();
    }

    let patient_index_a: usize = rng.gen_range(0..target_genome[nurse_a].len());
    let patient_index_b: usize = rng.gen_range(0..target_genome[nurse_b].len());
    let patient_a = target_genome[nurse_a][patient_index_a];
    let patient_b = target_genome[nurse_b][patient_index_b];
    target_genome[nurse_a][patient_index_a] = patient_b;
    target_genome[nurse_b][patient_index_b] = patient_a;
    return target_genome;
}

fn move_within_journey(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut source_nurse: usize;
    loop {
        source_nurse = rng.gen_range(0..problem_instance.number_of_nurses);
        if target_genome[source_nurse].len() > 1 {
            break;
        }
    }
    let source_patient_index: usize = rng.gen_range(0..target_genome[source_nurse].len());
    let patient = target_genome[source_nurse].remove(source_patient_index);
    let target_patient_index: usize = rng.gen_range(0..=target_genome[source_nurse].len());
    target_genome[source_nurse].insert(target_patient_index, patient);
    return target_genome;
}

fn inverse_journey(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut nurese: usize;
    loop {
        nurese = rng.gen_range(0..problem_instance.number_of_nurses);
        if target_genome[nurese].len() > 1 {
            break;
        }
    }
    target_genome[nurese].reverse();
    return target_genome;
}

fn two_opt(genome: &Genome, problem_instance: &ProblemInstance, _config: &Config) -> Genome {
    let target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut nurses_with_three_or_more_patients: Vec<usize> = Vec::new();
    for (nurese, _) in target_genome.iter().enumerate() {
        if target_genome[nurese].len() >= 3 {
            nurses_with_three_or_more_patients.push(nurese);
        }
    }
    // nurse who's journey we are going to optimize
    let nurese: usize = nurses_with_three_or_more_patients
        [rng.gen_range(0..nurses_with_three_or_more_patients.len())];
    let mut journey: Journey = target_genome[nurese].clone();

    let mut found_improvement = true;
    while found_improvement {
        found_improvement = false;
        for i in 0..journey.len() - 1 {
            for j in i..journey.len() {
                let old_distance: f64;
                let new_distance: f64;
                if i == 0 {
                    if j == journey.len() - 1 {
                        old_distance = problem_instance.travel_time[0][journey[0]]
                            + problem_instance.travel_time[journey[j]][0];
                        new_distance = problem_instance.travel_time[0][journey[j]]
                            + problem_instance.travel_time[journey[0]][0];
                    } else {
                        old_distance = problem_instance.travel_time[0][journey[0]]
                            + problem_instance.travel_time[journey[j]][journey[j + 1]];
                        new_distance = problem_instance.travel_time[0][journey[j]]
                            + problem_instance.travel_time[journey[0]][journey[j + 1]];
                    }
                } else if j == journey.len() - 1 {
                    old_distance = problem_instance.travel_time[journey[i - 1]][journey[i]]
                        + problem_instance.travel_time[journey[j]][0];
                    new_distance = problem_instance.travel_time[journey[i - 1]][journey[j]]
                        + problem_instance.travel_time[journey[i]][0];
                } else {
                    old_distance = problem_instance.travel_time[journey[i - 1]][journey[i]]
                        + problem_instance.travel_time[journey[j]][journey[j + 1]];
                    new_distance = problem_instance.travel_time[journey[i - 1]][journey[j]]
                        + problem_instance.travel_time[journey[i]][journey[j + 1]];
                }
                if new_distance < old_distance {
                    found_improvement = true;
                    journey[i..j + 1].reverse();
                }
            }
        }
    }
    return target_genome;
}

fn split_journey(genome: &Genome, problem_instance: &ProblemInstance, _config: &Config) -> Genome {
    let mut target_genome: Genome = Vec::new();
    let mut rng = rand::thread_rng();
    let mut destination_nurse = usize::MAX;

    for (nurse, journey) in genome.iter().enumerate() {
        if journey.is_empty() {
            destination_nurse = nurse;
            break;
        }
    }

    if destination_nurse == usize::MAX {
        return genome.clone();
    }

    let source_nurse: usize = loop {
        let nurse = rng.gen_range(0..problem_instance.number_of_nurses);
        if genome[nurse].len() > 1 {
            break nurse;
        }
    };

    let mut longest_travel_time = -1.0;
    let mut split_index = 0;

    for (i, &location) in genome[source_nurse]
        .iter()
        .enumerate()
        .take(genome[source_nurse].len() - 1)
    {
        let travel_time = problem_instance.travel_time[location][genome[source_nurse][i + 1]];
        if travel_time > longest_travel_time {
            longest_travel_time = travel_time;
            split_index = i;
        }
    }

    let new_journey: Journey = genome[source_nurse][split_index + 1..].to_vec();
    target_genome[destination_nurse] = new_journey;
    target_genome[source_nurse] = genome[source_nurse][..split_index + 1].to_vec();
    return target_genome;
}

fn validate_journey_if_patient_is_inserted(
    journey: &Journey,
    patient_id: usize,
    insertion_point: usize,
    problem_instance: &ProblemInstance,
) -> bool {
    if journey.is_empty() {
        return true;
    }
    let mut journey_copy = journey.clone();
    journey_copy.insert(insertion_point, patient_id);
    return is_journey_valid(&journey_copy, problem_instance);
}

fn insertion_heuristic(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = vec![vec![]; problem_instance.number_of_nurses];

    let unflattened_genome: Vec<&usize> = genome.iter().flatten().collect();
    for patient_id in unflattened_genome.iter() {
        let mut min_detour = f64::MAX;
        let mut min_detour_index = usize::MAX;
        let mut nurse_id = usize::MAX;
        for (nurse, _) in target_genome.iter().enumerate() {
            let current_journey: &Journey = &target_genome[nurse];
            if current_journey.is_empty() {
                min_detour = problem_instance.travel_time[0][**patient_id]
                    + problem_instance.travel_time[**patient_id][0];
                min_detour_index = 0;
                nurse_id = nurse;
            } else {
                // calculate detour if patient is inserted between first patient and depot
                let detour: f64 = problem_instance.travel_time[0][**patient_id]
                    + problem_instance.travel_time[**patient_id][current_journey[0]]
                    - problem_instance.travel_time[0][current_journey[0]];
                if detour < min_detour
                    && validate_journey_if_patient_is_inserted(
                        current_journey,
                        **patient_id,
                        0,
                        problem_instance,
                    )
                {
                    min_detour = detour;
                    min_detour_index = 0;
                    nurse_id = nurse;
                }
                // calculate detour between patients the trip back to the depot is not considered
                for i in 0..current_journey.len() - 1 {
                    let detour: f64 = problem_instance.travel_time[current_journey[i]]
                        [**patient_id]
                        + problem_instance.travel_time[**patient_id][current_journey[i + 1]]
                        - problem_instance.travel_time[current_journey[i]][current_journey[i + 1]];
                    if detour < min_detour
                        && validate_journey_if_patient_is_inserted(
                            current_journey,
                            **patient_id,
                            i + 1,
                            problem_instance,
                        )
                    {
                        min_detour = detour;
                        min_detour_index = i + 1;
                        nurse_id = nurse;
                    }
                }
                // calculate detour if patient is inserted between last patient and depot
                let detour: f64 = problem_instance.travel_time
                    [current_journey[current_journey.len() - 1]][**patient_id]
                    + problem_instance.travel_time[**patient_id][0]
                    - problem_instance.travel_time[current_journey[current_journey.len() - 1]][0];
                if detour < min_detour
                    && validate_journey_if_patient_is_inserted(
                        current_journey,
                        **patient_id,
                        current_journey.len(),
                        problem_instance,
                    )
                {
                    min_detour = detour;
                    min_detour_index = current_journey.len();
                    nurse_id = nurse;
                }
            }
        }
        if min_detour == f64::MAX || min_detour_index == usize::MAX || nurse_id == usize::MAX {
            panic!("No valid insertion point found for patient {}", patient_id);
            return genome.clone();
        }
        target_genome[nurse_id].insert(min_detour_index, **patient_id);
    }

    target_genome
}
/*
fn lin_kernighan(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    _config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();


    for journey in target_genome.iter_mut() {
        if journey.len() < 2 {
            continue;
        }
        let mut improvement = true;
        let mut old_distance = is_journey_valid(&journey, problem_instance);
        while improvement {
            improvement = false;
            for i in 0..journey.len() - 1 {
                for j in i + 1..journey.len() {
                    let mut candidate_journey = journey.clone();
                    candidate_journey[i..=j].reverse();
                    let new_distance = is_journey_valid(&candidate_journey, problem_instance);
                    // if only one is valid, we take it otherwise we take the one with the lowest distance
                    if new_distance && !old_distance || 
                        (new_distance && old_distance && new_distance.1 < old_distance.1) ||
                        (!new_distance && !old_distance && new_distance.1 < old_distance.1)
                        {
                        improvement = true;
                        old_distance = new_distance;
                    }
                }
            }

        }
    }

    target_genome
}*/

pub fn mutate(
    population: &mut Population,
    problem_instance: &ProblemInstance,
    config: &Config,
) -> Population {
    let mut rng = rand::thread_rng();
    let mut children: Population = population.clone();
    for mutation_config in config.mutations.iter() {
        // Calculate the number of crossovers which should happen for the specific config
        let number_of_mutations: u64 = ((config.population_size as f64)
            * mutation_config.probability.unwrap_or(0.0))
        .ceil() as u64;

        for _ in 0..number_of_mutations {
            let individual_index: usize = rng.gen_range(0..config.population_size);

            let child_genome: Genome = match mutation_config.name.as_str() {
                "reassignOnePatient" => reassign_one_patient(
                    &children[individual_index].genome,
                    problem_instance,
                    config,
                ),
                "swapWithinJourney" => swap_within_journey(
                    &children[individual_index].genome,
                    problem_instance,
                    config,
                ),
                "swapBetweenJourneys" => swap_between_journeys(
                    &children[individual_index].genome,
                    problem_instance,
                    config,
                ),
                "moveWithinJourney" => move_within_journey(
                    &children[individual_index].genome,
                    problem_instance,
                    config,
                ),
                "inverseJourney" => {
                    inverse_journey(&children[individual_index].genome, problem_instance, config)
                }
                "twoOpt" => two_opt(&children[individual_index].genome, problem_instance, config),
                "splitJourney" => {
                    split_journey(&children[individual_index].genome, problem_instance, config)
                }
                "insertionHeuristic" => insertion_heuristic(
                    &children[individual_index].genome,
                    problem_instance,
                    config,
                ),
                /*
                "linKernighan" => lin_kernighan(
                    &children[individual_index].genome,
                    problem_instance,
                    config,
                ),*/
                _ => panic!(
                    "Didn't have an Implementation for mutation function: {:?}",
                    mutation_config.name.as_str()
                ),
            };

            let mut child = Individual::new(child_genome);
            calculate_fitness(&mut child, problem_instance);
            children[individual_index] = child;
        }
    }

    return children;
}
