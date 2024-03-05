
pub type Journey = Vec<u64>;
pub type Genome = Vec<Journey>;


pub struct Individual {
    pub genome: Genome,
    pub fitness: f64,
    
    // penalty
    pub missing_care_time_penalty: f64,
    pub capacity_penalty: f64,
    pub to_late_to_depot_penality: f64,

}
