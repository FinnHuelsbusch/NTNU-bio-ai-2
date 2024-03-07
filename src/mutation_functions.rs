use crate::{
    config::Config,
    individual::{calculate_fitness, unflattened_genome, Genome, Individual, Journey},
    population::Population,
    problem_instance::ProblemInstance,
};
use rand::Rng;

fn reassign_one_patient(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    config: &Config,
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
    let target_nurse: usize = rng.gen_range(0..problem_instance.number_of_nurses);
    let target_patient_index: usize = rng.gen_range(0..=target_genome[target_nurse].len());
    let patient = target_genome[source_nurse].remove(source_patient_index);
    target_genome[target_nurse].insert(target_patient_index, patient);
    return target_genome;
}

fn swap_within_journey(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    config: &Config,
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
    let target_patient_index: usize = rng.gen_range(0..target_genome[source_nurse].len());
    let patient = target_genome[source_nurse].remove(source_patient_index);
    target_genome[source_nurse].insert(target_patient_index, patient);
    return target_genome;
}

fn swap_between_journeys(
    genome: &Genome,
    problem_instance: &ProblemInstance,
    config: &Config,
) -> Genome {
    let mut target_genome: Genome = genome.clone();
    let mut rng = rand::thread_rng();
    let mut nurse_a: usize;
    let mut nurse_b: usize;
    loop {
        nurse_a = rng.gen_range(0..problem_instance.number_of_nurses);
        if target_genome[nurse_a].len() > 0 {
            break;
        }
    }
    loop {
        nurse_b = rng.gen_range(0..problem_instance.number_of_nurses);
        if nurse_b != nurse_a && target_genome[nurse_b].len() > 0 {
            break;
        }
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
    config: &Config,
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

fn inverse_journey(genome: &Genome, problem_instance: &ProblemInstance, config: &Config) -> Genome {
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

// fn two_opt(genome: &Genome, problem_instance: &ProblemInstance, config: &Config) -> Genome {
//     let mut target_genome: Genome = genome.clone();
//     let mut rng = rand::thread_rng();
//     let mut nurses_with_three_or_more_patients: Vec<usize> = Vec::new();
//     for (nurese, journey) in target_genome.iter().enumerate() {
//         if target_genome[nurese].len() >= 3 {
//             nurses_with_three_or_more_patients.push(nurese);
//         }
//     }
//     // nurse who's journey we are going to optimize
//     let nurese: usize = nurses_with_three_or_more_patients
//         [rng.gen_range(0..nurses_with_three_or_more_patients.len())];
//     let mut journey: Journey = target_genome[nurese].clone();

//     let mut found_improvement = true;
//     while found_improvement {
//         found_improvement = false;
//         for i in -1..journey.len() - 2 {
//             for j in i + 1..journey.len() {
//                 let old_distance: f64;
//                 let new_distance: f64;
//                 if i == -1 {
//                     if j == journey.len() - 1 {
//                         old_distance = problem_instance.travel_time[0][journey[0]]
//                             + problem_instance.travel_time[journey[j]][0];
//                         new_distance = problem_instance.travel_time[0][journey[j]]
//                             + problem_instance.travel_time[journey[0]][0];
//                     } else {
//                         old_distance = problem_instance.travel_time[0][journey[0]]
//                             + problem_instance.travel_time[journey[j]][journey[j + 1]];
//                         new_distance = problem_instance.travel_time[0][journey[j]]
//                             + problem_instance.travel_time[journey[0]][journey[j + 1]];
//                     }
//                 } else if j == journey.len() - 1 {
//                     old_distance = problem_instance.travel_time[journey[i]][journey[i + 1]]
//                         + problem_instance.travel_time[journey[j]][0];
//                     new_distance = problem_instance.travel_time[journey[i]][journey[j]]
//                         + problem_instance.travel_time[journey[i + 1]][0];
//                 } else {
//                     old_distance = problem_instance.travel_time[journey[i]][journey[i + 1]]
//                         + problem_instance.travel_time[journey[j]][journey[j + 1]];
//                     new_distance = problem_instance.travel_time[journey[i]][journey[j]]
//                         + problem_instance.travel_time[journey[i + 1]][journey[j + 1]];
//                 }
//                 if new_distance < old_distance {
//                     found_improvement = true;
//                     journey[i + 1..j + 1].reverse();
//                 }
//             }
//         }
//     }
//     return target_genome;
// }

pub fn mutate(
    population: &mut Population,
    problem_instance: &ProblemInstance,
    config: &Config,
) -> Population {
    let mut rng = rand::thread_rng();
    let mut children: Population = population.clone();
    for muatation_config in config.mutations.iter() {
        // Calculate the number of crossovers which should happen for the specific config
        let number_of_mutations: u64 = ((config.population_size as f64)
            * muatation_config.probability.unwrap_or(0.0))
        .ceil() as u64;

        for _ in 0..number_of_mutations {
            let individual_index: usize = rng.gen_range(0..config.population_size);

            let child_genome: (Genome) = match config.parent_selection.name.as_str() {
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
                //"twoOpt" => two_opt(&children[individual_index].genome, problem_instance, config),

                // Handle the rest of cases
                _ => panic!(
                    "Didn't have an Implementation for selection function: {:?}",
                    config.parent_selection.name.as_str()
                ),
            };
            let mut child = Individual::new(child_genome);
            calculate_fitness(&mut child, problem_instance);
            children[individual_index] = child;
        }
    }

    return children;
}
