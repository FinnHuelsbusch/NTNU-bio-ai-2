use std::collections::HashMap;

use crate::problem_instance::ProblemInstance;


pub type Journey = Vec<u8>;
pub type Genome = Vec<Journey>;


pub struct Individual {
    pub genome: Genome,
    pub fitness: f64,
    
    // penalty
    pub missing_care_time_penalty: f64,
    pub capacity_penalty: f64,
    pub to_late_to_depot_penality: f64,

}

pub fn is_journey_valid(journey: &Journey, problem_instance: &ProblemInstance) -> bool {
    if journey.is_empty(){
        return true; 
    }

    let mut total_time_spent = 0.0; 
    let mut total_fullfilled_demand = 0 as u32;
    for (i, patient_id) in journey.iter().enumerate() {
        let previous_patient_id = journey[i - 1];
        if i == 0 {
            total_time_spent += problem_instance.travel_time[0][*patient_id as usize];
        } else {
            total_time_spent += problem_instance.travel_time[previous_patient_id as usize][*patient_id as usize];
        }
        if total_time_spent < problem_instance.patients[patient_id].start_time as f64{
            total_time_spent = problem_instance.patients[patient_id].start_time as f64;
        }
        total_time_spent += problem_instance.patients[patient_id].care_time as f64; 
        if total_time_spent > problem_instance.patients[patient_id].end_time as f64{
            return false;
        }
        total_fullfilled_demand += problem_instance.patients[patient_id].demand;
        if total_fullfilled_demand > problem_instance.nurse_capacity {
            return false;
        }
    }
    // add the driving time from the last patient to the depot if there is at least one patient
    if !journey.is_empty() {
        total_time_spent += problem_instance.travel_time[journey[journey.len() - 1] as usize][0];
    }
    if total_time_spent > problem_instance.depot.return_time {
        return false;
    }
    true
}


pub fn is_genome_valid(genome: &Genome, problem_instance: &ProblemInstance) -> bool {
    let mut is_valid = true;
    // validate that each patient is visited exactly once
    let mut visited_patients = HashMap::<u8, bool>::new();
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

pub fn calculate_fitness(individual: & mut Individual, problem_instance: &ProblemInstance){

    let mut combined_trip_time = 0.0;
    let mut missing_care_time_penalty = 0.0;
    let mut capacity_penalty = 0.0;
    let mut to_late_to_depot_penality = 0.0;

    let travel_time = &problem_instance.travel_time;

    for journey in &individual.genome {
        let mut nurse_trip_time = 0.0;
        let mut nurse_travel_time = 0.0;
        let mut nurse_used_capacity = 0;

        for (i, patient_id) in journey.iter().enumerate() {
            
            if i == 0 { // if trip is from depot to patient
                nurse_trip_time += travel_time[0][*patient_id as usize];
                nurse_travel_time += travel_time[0][*patient_id as usize];
            } else { // if trip is from patient to patient
                nurse_trip_time += travel_time[journey[i - 1] as usize][*patient_id as usize];
                nurse_travel_time += travel_time[journey[i - 1] as usize][*patient_id as usize];
            }
            // If the nurse_trip_time is lower than the patient's start time, wait to the start of the time window
            nurse_trip_time = nurse_trip_time.max(problem_instance.patients[patient_id].start_time as f64);
            // Nurse is caring for the patient
            nurse_trip_time += problem_instance.patients[patient_id].care_time as f64;
            // If the nurse is leaving to late add the missed care time as a penalty
            if nurse_trip_time > problem_instance.patients[patient_id].end_time as f64 {
                missing_care_time_penalty = nurse_trip_time - problem_instance.patients[patient_id].end_time as f64;
            }
            
            nurse_used_capacity += problem_instance.patients[patient_id].demand;
            if nurse_used_capacity > problem_instance.nurse_capacity {
                capacity_penalty = (nurse_used_capacity - problem_instance.nurse_capacity) as f64;
            }
        }
        // add the driving time from the last patient to the depot if there is at least one patient
        if !journey.is_empty() {
            nurse_trip_time += travel_time[journey[journey.len() - 1] as usize][0];
            nurse_travel_time += travel_time[journey[journey.len() - 1] as usize][0];
        }
        // add penalty if we are too late to the depot
        to_late_to_depot_penality = f64::max(0.0, nurse_trip_time - problem_instance.depot.return_time as f64);
        combined_trip_time += nurse_travel_time;
    }
    let fitness = -combined_trip_time - capacity_penalty * 100000.0 - missing_care_time_penalty * 10000.0 - to_late_to_depot_penality * 10000.0;
    individual.fitness = fitness;
    individual.missing_care_time_penalty = missing_care_time_penalty;
    individual.capacity_penalty = capacity_penalty;
    individual.to_late_to_depot_penality = to_late_to_depot_penality;
}


    

