use std::collections::HashMap;
use rand::seq::SliceRandom;
use serde::de;
use crate::{config::{self, Config}, individual::{calculate_fitness, is_journey_valid, Individual}, patient::{self, Patient}, problem_instance::{self, ProblemInstance}};


pub type Population = Vec<Individual>;

// auto initializeFeasiblePopulation(const ProblemInstance &problemInstance, const Config &config) -> Population
// {
    
// }

// pub fn initialize_random_population() -> Population {

// }




// TODO debattieren, ob wir das so machen wollen
// ist der rekursive aufruf in der funktion initialize_valid_population sinnvoll?
// sollten wir diese appendHeuristic verwenden oder eine bessere Heuristic verwenden? (vorteil ist, dass das hier schneller ist. Ein insert braucht O(number of nurses). Bessere hätten O(number of patients * 0.5)
pub fn initialize_valid_population(problem_instance: ProblemInstance, config: Config) -> Population {

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


   let mut population: Population = vec![];
   while population.len() < config.population_size as usize {
        let mut genome: Vec<Vec<u8>> = vec![];
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
                        detour = problem_instance.travel_time[0][patient.id as usize] + problem_instance.travel_time[patient.id as usize][0];
                    } else {
                        detour = problem_instance.travel_time[journey[journey.len() - 1] as usize][patient.id as usize] + problem_instance.travel_time[patient.id as usize][0] - problem_instance.travel_time[journey[journey.len() - 1] as usize][0];
                    }
                    if detour < smallest_detour{
                        let mut updated_journey = journey.clone();
                        updated_journey.push(patient.id);
                        if is_journey_valid(&updated_journey, &problem_instance) {
                            smallest_detour = detour;
                            best_position = i;
                        }
                    }
                }
                if smallest_detour < f64::INFINITY {
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
            let mut individual = Individual{genome, fitness: 0.0, missing_care_time_penalty: 0.0, capacity_penalty: 0.0, to_late_to_depot_penality: 0.0}; 
            calculate_fitness(& mut individual, &problem_instance);
            population.push(individual);
        }


   }

    return population;

}
    
    
// }
