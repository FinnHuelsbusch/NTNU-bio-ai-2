use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use crate::{depot::Depot, patient::Patient};

#[derive(Debug, Deserialize)]
pub struct ProblemInstance {
    pub instance_name: String,

    #[serde(rename = "nbr_nurses")]
    pub number_of_nurses: usize,

    #[serde(rename = "capacity_nurse")]
    pub nurse_capacity: u32,
    pub benchmark: f64,
    pub depot: Depot,

    #[serde(deserialize_with = "deserialize_patients")]
    pub patients: HashMap<usize, Patient>,

    #[serde(rename = "travel_times")]
    pub travel_time: Vec<Vec<f64>>,
}

fn deserialize_patients<'de, D>(deserializer: D) -> Result<HashMap<usize, Patient>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PatientsVisitor;

    impl<'de> serde::de::Visitor<'de> for PatientsVisitor {
        type Value = HashMap<usize, Patient>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with u8 keys and Patient values")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut patients = HashMap::new();

            while let Some((key, value)) = map.next_entry::<usize, Patient>()? {
                let patient_with_id = Patient { id: key, ..value };
                patients.insert(key, patient_with_id);
            }

            Ok(patients)
        }
    }

    deserializer.deserialize_map(PatientsVisitor)
}

pub fn initialize_problem_instance(file_path: &str) -> ProblemInstance {
    let data = std::fs::read_to_string(file_path)
        .expect("Unable to read problem instance. Is the file path correct?");
    let new_instance: ProblemInstance = serde_json::from_str(&data)
        .expect("Problem instance json was not well formatted! Did the format change?");
    new_instance
}
