use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FunctionConfig {
    pub name: String,

    #[serde(default)]
    pub probability: Option<f64>,

    #[serde(default)]
    pub annealing_delta: Option<f64>,

    #[serde(default)]
    pub tournament_size: Option<usize>,

    #[serde(default)]
    pub tournament_probability: Option<f64>,

    #[serde(default)]
    pub elitism_percentage: Option<f64>,

    #[serde(default)]
    pub combine_parents_and_offspring: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub problem_instance: String,
    pub population_initialisation: String,
    pub population_size: usize,
    pub number_of_generations: usize,
    pub parent_selection: FunctionConfig,
    pub crossovers: Vec<FunctionConfig>,
    pub mutations: Vec<FunctionConfig>,
    pub survivor_selection: FunctionConfig,
}

pub fn initialize_config(file_path: &str) -> Config {
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("JSON was not well-formatted");
    config
}
