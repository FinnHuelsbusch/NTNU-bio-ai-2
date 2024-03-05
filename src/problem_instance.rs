
use std::collections::HashMap;
use serde::Deserialize;

use crate::{depot::Depot, patient::Patient};


#[derive(Debug, Deserialize)]
pub struct ProblemInstance{
    pub instance_name: String, 

    #[serde(rename = "nbr_nurses")]
    pub number_of_nurses: u8, 

    #[serde(rename = "capacity_nurse")]
    pub nurse_capacity: u32,
    pub benchmark: f64, 
    pub depot: Depot, 
    
    
    pub patients: HashMap<u8, Patient>, 

    #[serde(rename = "travel_times")]
    pub travel_time: Vec<Vec<f64>>
}

pub fn initialize_problem_instance(file_path: &str) -> ProblemInstance {
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let new_instance: ProblemInstance = serde_json::from_str(&data).expect("JSON was not well-formatted");
    new_instance
}