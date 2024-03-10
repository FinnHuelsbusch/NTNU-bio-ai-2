use serde::{ Deserialize, Serialize };
use serde_json::Error;

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
    
    #[serde(default)]
    pub percentage_to_slice: Option<f64>,
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
    pub log_file: Option<String>,
    pub output_file: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaConfig {
    pub configs: Vec<Config>,
    pub output_file: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigType {
    MetaConfig(MetaConfig),
    Config( Config)
}

pub fn initialize_config(file_path: &str) -> ConfigType {
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let new_instance: Result<Config, Error> = serde_json::from_str(&data);
    match new_instance {
        Ok(config) => ConfigType::Config(config),
        Err(_) => {
            let new_instance: MetaConfig = serde_json::from_str(&data).expect("Config json was not well formatted! Did the format change?");
            ConfigType::MetaConfig(new_instance)
        }
    }
}
