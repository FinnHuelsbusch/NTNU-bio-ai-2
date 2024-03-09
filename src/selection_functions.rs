use rand::Rng;

use crate::{config::Config, population::Population};

fn roulette_wheel_selection(population: &Population, population_size: usize) -> Population {
    let mut new_population: Population = Vec::with_capacity(population_size);

    // Map fitness values
    let mut fitness_values: Vec<f64> = population
        .iter()
        .map(|individual| individual.fitness)
        .collect();

    // Get the minimal fitness
    let min_fitness: f64 = *fitness_values
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap_or(&0.0);

    // Transform fitness to positive range
    fitness_values = fitness_values
        .iter_mut()
        .map(|fitness_value| *fitness_value - min_fitness + f64::EPSILON)
        .collect();

    let total_fitness: f64 = fitness_values.iter().sum();
    let mut rng = rand::thread_rng();

    for _ in 0..population_size {
        let selected_value = rng.gen::<f64>() * total_fitness;
        let mut sum = 0.0;

        for (index, individual) in population.iter().enumerate() {
            sum += fitness_values[index];
            if selected_value < sum {
                new_population.push(individual.clone());
                break;
            }
        }
    }

    new_population
}

/*
auto tournamentSelection(const Population &population, const FunctionParameters &parameters, const int populationSize) -> Population
{
    auto mainLogger = spdlog::get("main_logger");
    // check that the parameters are present
    if (parameters.find("tournamentSize") == parameters.end() || parameters.find("tournamentProbability") == parameters.end())
    {
        throw std::invalid_argument("Tournament selection requires the parameters 'tournamentSize' and 'tournamentProbability'");
    }

    int tournamentSize = std::get<int>(parameters.at("tournamentSize"));
    double tournamentProbability = std::get<double>(parameters.at("tournamentProbability"));
    mainLogger->trace("TournamentSize {}", tournamentSize);
    mainLogger->trace("TournamentProbability {}", tournamentProbability);
    Population parents;
    RandomGenerator &rng = RandomGenerator::getInstance();
    for (int i = 0; i < populationSize; i++)
    {
        std::vector<int> tournament;
        tournament.reserve(tournamentSize);
        for (int j = 0; j < tournamentSize; j++)
        {
            int index = rng.generateRandomInt(0, population.size() - 1);
            tournament.push_back(index);
        }

        // get index of best individual according to fitness
        auto it = std::max_element(tournament.begin(), tournament.end(), [&population](int a, int b) {
            return population[a].fitness < population[b].fitness;
        });

        int bestIndex = std::distance(tournament.begin(), it);

        double randNumber = rng.generateRandomDouble(0, 1);
        if (randNumber <= tournamentProbability)
        {
            parents.push_back(population[tournament[bestIndex]]);
        }
        else
        {
            // remove the best individual from the tournament
            tournament.erase(tournament.begin() + bestIndex);
            // select a random individual from the remaining ones
            int randomIndex = rng.generateRandomInt(0, tournament.size() - 1);
            parents.push_back(population[tournament[randomIndex]]);
        }
    }
    return parents;
}
 */

fn tournament_selection(population: &Population, population_size: usize, tournament_size: usize, tournament_probability: f64) -> Population {
    let mut new_population: Population = Vec::with_capacity(population_size);
    let mut rng = rand::thread_rng();

    for _ in 0..population_size {
        let mut tournament: Vec<usize> = Vec::with_capacity(tournament_size);
        for _ in 0..tournament_size {
            let index = rng.gen_range(0..population.len());
            tournament.push(index);
        }

        let best_index = tournament
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                population[**a]
                    .fitness
                    .partial_cmp(&population[**b].fitness)
                    .unwrap()
            }).unwrap().0;

        let rand_number = rng.gen::<f64>();
        if rand_number <= tournament_probability {
            new_population.push(population[best_index].clone());
        } else {
            tournament.remove(best_index);
            let random_index = rng.gen_range(0..tournament.len());
            new_population.push(population[tournament[random_index]].clone());
        }
    }

    new_population
}

fn full_replacement_selection(
    population: &Population,
    children: &Population,
    population_size: usize,
) -> Population {
    assert_eq!(population.len(), population_size);
    assert_eq!(children.len(), population_size);
    children.clone()
}

pub fn parent_selection(population: &Population, config: &Config) -> Population {
    let number_of_elites = (config.parent_selection.elitism_percentage.unwrap_or(0.0)
        * (config.population_size as f64))
        .ceil() as usize;
    assert!(number_of_elites < config.population_size);
    let mut new_population: Population = Vec::with_capacity(config.population_size);
    if number_of_elites > 0 {
        let mut elite_population: Population = population.clone();
        elite_population.retain(|x| x.is_feasible());
        elite_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        // Add the elites to the new population
        for i in 0..number_of_elites {
            new_population.push(elite_population[i].clone());
        }

    }
    let selected_population: Population = match config.parent_selection.name.as_str() {
        // Match a single value
        "rouletteWheel" => {
            roulette_wheel_selection(&population, config.population_size - number_of_elites)
        },
        "tournament" => {
            tournament_selection(&population,config.population_size - number_of_elites, config.parent_selection.tournament_size.unwrap(), config.parent_selection.tournament_probability.unwrap())
        },
        // Handle the rest of cases
        _ => panic!(
            "Didn't have an Implementation for selection function: {:?}",
            config.parent_selection.name.as_str()
        ),
    };
    new_population.extend(selected_population);
    new_population
}

pub fn survivor_selection(
    parents: &Population,
    children: &Population,
    config: &Config,
) -> Population {
    let mut selection_population: Population;
    if config
        .survivor_selection
        .combine_parents_and_offspring
        .unwrap_or(false)
    {
        let mut combined_population: Population = parents.clone();
        combined_population.extend(children.clone());
        selection_population = combined_population;
    } else {
        selection_population = children.clone();
    }
    selection_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let mut new_population: Population = Vec::with_capacity(config.population_size);
    let number_of_elites = (config.survivor_selection.elitism_percentage.unwrap_or(0.0)
        * (config.population_size as f64))
        .ceil() as usize;
    assert!(number_of_elites < config.population_size);
    if number_of_elites > 0 {
        let mut  elite_population: Population  = selection_population.clone();
        elite_population.retain(|x| x.is_feasible());
        elite_population.sort_unstable_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        for i in 0..number_of_elites {
            new_population.push(elite_population[i].clone());
        }
    }

    let selected_population: Population = match config.parent_selection.name.as_str() {
        // Match a single value
        "rouletteWheel" => roulette_wheel_selection(
            &selection_population,
            config.population_size - number_of_elites,
        ),
        "fullReplacement" => {
            full_replacement_selection(parents, children, config.population_size - number_of_elites)
        },
        "tournament" => {
            tournament_selection(&selection_population,config.population_size - number_of_elites, 
                                 config.survivor_selection.tournament_size.unwrap_or_else(|| panic!("You need to specify the tournament size if you are using tournament selection for survivor selection.")),
                                 config.survivor_selection.tournament_probability.unwrap_or_else(|| panic!("You need to specify the tournament probability if you are using tournament selection for survivor selection.")))
        },
        _ => panic!(
            "Didn't have an Implementation for selection function: {:?}",
            config.parent_selection.name.as_str()
        ),
    };
    new_population.extend(selected_population);
    new_population
}
