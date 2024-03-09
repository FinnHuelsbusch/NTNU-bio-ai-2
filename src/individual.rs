use std::{ collections::HashMap, fs::File, io::Write };

use serde::Serialize;

use crate::problem_instance::ProblemInstance;

pub type Journey = Vec<usize>;
pub type Genome = Vec<Journey>;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Individual {
    pub genome: Genome,
    pub fitness: f64,
    pub travel_time: f64,

    // penalty
    pub missing_care_time_penalty: f64,
    pub capacity_penalty: f64,
    pub to_late_to_depot_penality: f64,
}


impl Individual {
    pub fn new(genome: Genome) -> Self {
        Individual {
            genome,
            fitness: 0.0,
            travel_time: 0.0,
            missing_care_time_penalty: 0.0,
            capacity_penalty: 0.0,
            to_late_to_depot_penality: 0.0,
        }
    }

    pub fn is_feasible(&self) -> bool {
        !(
            self.missing_care_time_penalty > 0.0 ||
            self.capacity_penalty > 0.0 ||
            self.to_late_to_depot_penality > 0.0
        )
    }

    pub fn export_to_file(&self, file_path: &str) {
        // Serialize the struct to JSON
        let json_string = serde_json::to_string(&self).unwrap();

        // Write JSON string to a file
        let mut file = File::create(file_path).expect("Unable to create file");
        file.write_all(json_string.as_bytes()).expect("Unable to write to file");
    }
}

pub fn is_journey_valid(journey: &Journey, problem_instance: &ProblemInstance) -> bool {
    if journey.is_empty() {
        return true;
    }

    let mut total_time_spent = 0.0;
    let mut total_fullfilled_demand = 0_u32;
    for (i, patient_id) in journey.iter().enumerate() {
        if i == 0 {
            total_time_spent += problem_instance.travel_time[0][*patient_id];
        } else {
            let previous_patient_id = journey[i - 1];
            total_time_spent += problem_instance.travel_time[previous_patient_id][*patient_id];
        }
        if total_time_spent < (problem_instance.patients[patient_id].start_time as f64) {
            total_time_spent = problem_instance.patients[patient_id].start_time as f64;
        }
        total_time_spent += problem_instance.patients[patient_id].care_time as f64;
        if total_time_spent > (problem_instance.patients[patient_id].end_time as f64) {
            return false;
        }
        total_fullfilled_demand += problem_instance.patients[patient_id].demand;
        if total_fullfilled_demand > problem_instance.nurse_capacity {
            return false;
        }
    }
    // add the driving time from the last patient to the depot if there is at least one patient
    if !journey.is_empty() {
        total_time_spent += problem_instance.travel_time[journey[journey.len() - 1]][0];
    }
    total_time_spent <= problem_instance.depot.return_time
}

pub fn is_genome_valid(genome: &Genome, problem_instance: &ProblemInstance) -> bool {
    let mut is_valid = true;
    // validate that each patient is visited exactly once
    let mut visited_patients = HashMap::<usize, bool>::new();
    for journey in genome {
        for patient_id in journey {
            if visited_patients.contains_key(patient_id) {
                is_valid = false;
                // TODO: log error message
            } else {
                visited_patients.insert(*patient_id, true);
            }
        }
        if !is_journey_valid(journey, problem_instance) {
            is_valid = false;
            // TODO: log error message
        }
    }
    // validate that all patients are visited
    for patient_id in problem_instance.patients.keys() {
        if !visited_patients.contains_key(patient_id) {
            is_valid = false;
            // TODO: log error message
        }
    }
    is_valid
}

pub fn calculate_fitness(individual: &mut Individual, problem_instance: &ProblemInstance) {
    let mut combined_trip_time = 0.0;
    let mut missing_care_time_penalty = 0.0;
    let mut capacity_penalty = 0.0;
    let mut to_late_to_depot_penality = 0.0;

    let travel_time = &problem_instance.travel_time;
    let mut combined_travel_time = 0.0;

    for journey in &individual.genome {
        let mut nurse_trip_time = 0.0;
        let mut nurse_travel_time = 0.0;
        let mut nurse_used_capacity = 0;

        for (i, patient_id) in journey.iter().enumerate() {
            if i == 0 {
                // if trip is from depot to patient
                nurse_trip_time += travel_time[0][*patient_id];
                nurse_travel_time += travel_time[0][*patient_id];
            } else {
                // if trip is from patient to patient
                nurse_trip_time += travel_time[journey[i - 1]][*patient_id];
                nurse_travel_time += travel_time[journey[i - 1]][*patient_id];
            }
            // If the nurse_trip_time is lower than the patient's start time, wait to the start of the time window
            nurse_trip_time = nurse_trip_time.max(
                problem_instance.patients[patient_id].start_time as f64
            );
            // Nurse is caring for the patient
            nurse_trip_time += problem_instance.patients[patient_id].care_time as f64;
            // If the nurse is leaving to late add the missed care time as a penalty
            if nurse_trip_time > (problem_instance.patients[patient_id].end_time as f64) {
                missing_care_time_penalty =
                    nurse_trip_time - (problem_instance.patients[patient_id].end_time as f64);
            }

            nurse_used_capacity += problem_instance.patients[patient_id].demand;
            if nurse_used_capacity > problem_instance.nurse_capacity {
                capacity_penalty = (nurse_used_capacity - problem_instance.nurse_capacity) as f64;
            }
        }
        // add the driving time from the last patient to the depot if there is at least one patient
        if !journey.is_empty() {
            nurse_trip_time += travel_time[journey[journey.len() - 1]][0];
            nurse_travel_time += travel_time[journey[journey.len() - 1]][0];
        }
        // add penalty if we are too late to the depot
        to_late_to_depot_penality = f64::max(
            0.0,
            nurse_trip_time - problem_instance.depot.return_time
        );
        combined_trip_time += nurse_travel_time;
        combined_travel_time += nurse_travel_time;
    }
    let fitness =
        -combined_trip_time -
        capacity_penalty * 100000.0 -
        missing_care_time_penalty * 10000.0 -
        to_late_to_depot_penality * 10000.0;

    individual.travel_time = combined_travel_time;
    individual.fitness = fitness;
    individual.missing_care_time_penalty = missing_care_time_penalty;
    individual.capacity_penalty = capacity_penalty;
    individual.to_late_to_depot_penality = to_late_to_depot_penality;
}

pub fn unflattened_genome(flattend_genome: &Journey, genome_original_structure: &Genome) -> Genome {
    let mut result: Genome = vec![Vec::new(); genome_original_structure.len()];

    let mut flattened_index: usize = 0;
    for (outer_index, original_journey) in genome_original_structure.iter().enumerate() {
        for _ in 0..original_journey.len() {
            result[outer_index].push(flattend_genome[flattened_index]);
            flattened_index += 1;
        }
    }

    result
}
