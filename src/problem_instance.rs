
use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

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
    
    #[serde(deserialize_with = "deserialize_patients")]
    pub patients: HashMap<u8, Patient>, 

    #[serde(rename = "travel_times")]
    pub travel_time: Vec<Vec<f64>>
}

fn deserialize_patients<'de, D>(deserializer: D) -> Result<HashMap<u8, Patient>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PatientsVisitor;

    impl<'de> serde::de::Visitor<'de> for PatientsVisitor {
        type Value = HashMap<u8, Patient>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with u8 keys and Patient values")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut patients = HashMap::new();

            while let Some((key, value)) = map.next_entry::<u8, Patient>()? {
                let id: u8 = key as u8;
                let patient_with_id = Patient { id, ..value };
                patients.insert(key, patient_with_id);
            }

            Ok(patients)
        }
    }

    deserializer.deserialize_map(PatientsVisitor)
}

pub fn initialize_problem_instance(file_path: &str) -> ProblemInstance {
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let new_instance: ProblemInstance = serde_json::from_str(&data).expect("JSON was not well-formatted");
    new_instance
}