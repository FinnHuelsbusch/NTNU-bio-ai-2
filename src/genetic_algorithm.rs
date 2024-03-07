use crate::{
    config::Config, crossover_functions::crossover, mutation_functions::mutate, population::{ get_average_fitness, initialize_population, Population }, problem_instance::ProblemInstance, selection_functions::{parent_selection, survivor_selection}
};

use std::io;
use std::io::Write;

fn log_population_statistics(population: &Population) {
    let mut sorted_population: Population = population.clone();
    sorted_population.sort();

    println!(
        "Best: {:?} Avg: {:?} Worst: {:?}",
        sorted_population[0].fitness,
        get_average_fitness(population),
        sorted_population[sorted_population.len() - 1].fitness
    )
}

/**
 * Individual SGA(ProblemInstance problemInstance, Config config)
{


    for (int currentGeneration = 0; currentGeneration < config.numberOfGenerations; currentGeneration++)
    {

        // Parent selection
        std::cout << "SEL|";
        Population parents = config.parentSelection.first(pop, config.parentSelection.second, populationSize);

        // Crossover
        std::cout << "CROSS|";
        Population children = applyCrossover(parents, config.crossover, problemInstance);
        // Mutation
        std::cout << "MUT|";
        children = applyMutation(children, config.mutation, problemInstance);

        // Survivor selection
        std::cout << "SURV_SEL" << '\n';
        pop = config.survivorSelection.first(pop, children, config.survivorSelection.second, populationSize);

        // calculate percentage of valid solutions
        int validSolutions = std::count_if(pop.begin(), pop.end(), [&](const Individual &individual)
                                          { return isSolutionValid(individual.genome, problemInstance); });
        double percentageValid = (validSolutions / static_cast<double>(pop.size())) * 100;
        main_logger->info("Percentage of valid solutions: {}", percentageValid);
        statistics_logger->info("Percentage of valid solutions: {}", percentageValid);

        // Average fitness
        sortPopulationByTravelTime(pop, false, problemInstance);
        double averageTravelTime = std::accumulate(pop.begin(), pop.end(), 0.0, [problemInstance](double sum, const Individual &individual)
                                                   { return sum + getTotalTravelTime(individual.genome, problemInstance); }) /
                                   pop.size();
        main_logger->info("Travel Time Best: {} Avg: {} Worst: {}", getTotalTravelTime(pop[0].genome, problemInstance), averageTravelTime, getTotalTravelTime(pop[pop.size() - 1].genome, problemInstance));
        statistics_logger->info("Travel Time Best: {} Avg: {} Worst: {}", getTotalTravelTime(pop[0].genome, problemInstance), averageTravelTime, getTotalTravelTime(pop[pop.size() - 1].genome, problemInstance));
        std::cout << "Travel Time Best: " << getTotalTravelTime(pop[0].genome, problemInstance) << " Avg: " << averageTravelTime << " Worst: " << getTotalTravelTime(pop[pop.size() - 1].genome, problemInstance) << " Percentage of valid solutions: " << percentageValid << '\n';

        // Genome logging: 
        // Log the Genome of the fastest individual
        logGenome(pop[0].genome, "Fastest", currentGeneration);
        // Log the Genome of the fittest individual
        sortPopulationByFitness(pop, false);
        logGenome(pop[0].genome, "Fittest", currentGeneration);

        // Log the fitness of the best, average and worst individual
        double averageFitness = std::accumulate(pop.begin(), pop.end(), 0.0, [](double sum, const Individual &individual)
                                                { return sum + individual.fitness; }) /
                                pop.size();
        main_logger->info("Fitness Best: {} Avg: {} Worst: {}", pop[0].fitness, averageFitness, pop[pop.size() - 1].fitness);
        statistics_logger->info("Fitness Best: {} Avg: {} Worst: {}", pop[0].fitness, averageFitness, pop[pop.size() - 1].fitness);
        
        main_logger->flush();
    }
    sortPopulationByFitness(pop, false);
    valid = isSolutionValid(pop[0].genome, problemInstance);

    double totalTravelTime = getTotalTravelTime(pop[0].genome, problemInstance);
    if(valid)
    {
        main_logger->info("The solution is valid and fullfills {}% of the benchmark", (problemInstance.benchmark / totalTravelTime) * 100);
        std::cout << "The solution is valid and fullfills " << ((problemInstance.benchmark / totalTravelTime) * 100) << "% of the benchmark" << '\n';
    }
    else
    {
        main_logger->info("The solution is invalid and fullfills {}% of the benchmark", (problemInstance.benchmark / totalTravelTime) * 100);
        std::cout << "The solution is invalid and fullfills " << ((problemInstance.benchmark / totalTravelTime) * 100) << "% of the benchmark" << '\n';
    }
    

    return pop[0];
}
 */
pub fn run_genetic_algorithm_instance(problem_instance: &ProblemInstance, config: &Config) {
    let mut population: Population = initialize_population(problem_instance, config);

    for generation in 0..config.number_of_generations {
        println!("Calculating Generation: {:?}", generation);
        print!("SEL|");
        io::stdout().flush().unwrap();
        let mut parents = parent_selection(&population, config);

        print!("CROSS|");
        io::stdout().flush().unwrap();;
        let mut children = crossover(&mut parents, problem_instance, config);

        print!("MUT|");
        io::stdout().flush().unwrap();
        children = mutate(&mut children, problem_instance, config);

        println!("SURV_SEL");
        io::stdout().flush().unwrap();
        population = survivor_selection(&parents, &children, config);

        log_population_statistics(&population);
    }
}
