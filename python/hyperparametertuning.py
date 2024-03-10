
import multiprocessing
from multiprocessing.pool import ThreadPool
import random
import signal
import sys
import utils
from utils import Config
import concurrent.futures

# train 0-9




train_instance = [i for i in range(10)]
init_heuristic = ["appendHeuristic", "random"]
population_size = [100, 500, 1000, 5000]
number_of_generations = [100, 200 ,500, 1000]
parent_selection = ["tournament", "rouletteWheel"]
tournament_size_parent_selection = [2, 5, 10, 20, 50]
tournament_probability_parent_selection = [0.0, 0.05, 0.1, 0.2, 0.5]
elite_percentage_parent_selection = [0.0, 0.05, 0.1, 0.2, 0.5]
cross_over_probability = [0.0, 0.01, 0.05, 0.1, 0.2, 0.5]
mutation_probability = [0.0, 0.01, 0.05, 0.1, 0.2, 0.5]
percentage_to_slice = [0.1, 0.2, 0.3, 0.4, 0.5]
survivor_selection = ["tournament", "rouletteWheel", "fullReplacement"]
elite_percentage_survivor_selection = [0.0, 0.05, 0.1, 0.2, 0.5]
combine_parents_and_offspring = ["true", "false"]
tournament_size_survivor_selection = [2, 5, 10, 20, 50]
tournament_probability_survivor_selection = [0.0, 0.05, 0.1, 0.2, 0.5]

def get_random_config(name): 
    return Config(
        name = name,
        init_heuristic = random.choice(init_heuristic),
        population_size = random.choice(population_size),
        number_of_generations = random.choice(number_of_generations),
        parent_selection = random.choice(parent_selection),
        tournament_size_parent_selection = random.choice(tournament_size_parent_selection),
        tournament_probability_parent_selection = random.choice(tournament_probability_parent_selection),
        elite_percentage_parent_selection = random.choice(elite_percentage_parent_selection),
        cross_over_probability_partiallyMappedCrossover = random.choice(cross_over_probability),
        cross_over_probability_orderOneCrossover = random.choice(cross_over_probability),
        cross_over_probability_edgeRecombination = random.choice(cross_over_probability),
        mutation_probability_reassignOnePatient = random.choice(mutation_probability),
        mutation_probability_moveWithinJourney = random.choice(mutation_probability),
        mutation_probability_swapBetweenJourneys = random.choice(mutation_probability),
        mutation_probability_swapWithinJourney = random.choice(mutation_probability),
        mutation_probability_insertionHeuristic = random.choice(mutation_probability), 
        percentage_to_slice = random.choice(percentage_to_slice), 
        survivor_selection = random.choice(survivor_selection), 
        elite_percentage_survivor_selection = random.choice(elite_percentage_survivor_selection),
        combine_parents_and_offspring = random.choice(combine_parents_and_offspring),
        tournament_size_survivor_selection = random.choice(tournament_size_survivor_selection),
        tournament_probability_survivor_selection = random.choice(tournament_probability_survivor_selection)
    )
  







# Number of threads to use
num_threads = multiprocessing.cpu_count() - 1

# Create a ThreadPoolExecutor
with concurrent.futures.ThreadPoolExecutor(max_workers=num_threads) as executor:
    try: 
        # Create a list of configs
        configs = [get_random_config(f"config_{i}") for i in range(30)]

        # Submit the Config.run_tests method for each config
        futures = [executor.submit(Config.run_tests, config) for config in configs]

        # Wait for all threads to complete
        concurrent.futures.wait(futures)
    except KeyboardInterrupt:
        print("Caught KeyboardInterrupt, terminating threads")
        executor.shutdown(wait=True, cancel_futures=True)
        print("All threads terminated")
        sys.exit(0)



