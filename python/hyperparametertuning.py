
import json
import multiprocessing
from multiprocessing.pool import ThreadPool
import random
from string import Template
import signal
import subprocess
import logging
import sys
import threading
# train 0-9


def get_congig(
    train_instance: int,
    log_file: str,
    output_file: str,
    init_heuristic: str,
    population_size: int,
    number_of_generations: int,
    parent_selection: str,
    tournament_size_parent_selection: int,
    tournament_probability_parent_selection: float,
    elite_percentage_parent_selection: float,
    cross_over_probability_partiallyMappedCrossover: float,
    cross_over_probability_orderOneCrossover: float,
    cross_over_probability_edgeRecombination: float,
    mutation_probability_reassignOnePatient: float,
    mutation_probability_moveWithinJourney: float,
    mutation_probability_swapBetweenJourneys: float,
    mutation_probability_swapWithinJourney: float,
    mutation_probability_insertionHeuristic: float, 
    percentage_to_slice: float, 
    survivor_selection: str, 
    elite_percentage_survivor_selection: float,
    combine_parents_and_offspring: str,
    tournament_size_survivor_selection: int,
    tournament_probability_survivor_selection: float 
    )-> str:





    config = Template('''
    {
    "problem_instance": "./train/train_$train_instance.json",
    "log_file": "$log_file",
    "output_file": "$output_file",
    "population_initialisation": "$init_heuristic",
    "population_size": $population_size,
    "number_of_generations": $number_of_generations,
    "parent_selection": {
        "name": "$parent_selection",
        "tournament_size": $tournament_size_parent_selection,
        "tournament_probability": $tournament_probability_parent_selection,
        "elitism_percentage": $elite_percentage_parent_selection
    },
    "crossovers": [
        {
        "name": "partiallyMappedCrossover",
        "probability": $cross_over_probability_partiallyMappedCrossover
        },
        {
        "name": "orderOneCrossover",
        "probability": $cross_over_probability_orderOneCrossover
        },
        {
        "name": "edgeRecombination",
        "probability": $cross_over_probability_edgeRecombination
        }
    ],
    "mutations": [
        {
        "name": "reassignOnePatient",
        "probability": $mutation_probability_reassignOnePatient
        },
        {
        "name": "moveWithinJourney",
        "probability": $mutation_probability_moveWithinJourney
        },
        {
        "name": "swapBetweenJourneys",
        "probability": $mutation_probability_swapBetweenJourneys
        },
        {
        "name": "swapWithinJourney",
        "probability": $mutation_probability_swapWithinJourney
        },
        {
        "name": "insertionHeuristic",
        "probability": $mutation_probability_insertionHeuristic,
        "percentage_to_slice": $percentage_to_slice
        }
    ],
    "survivor_selection": {
        "name": "$survivor_selection",
        "elitism_percentage": $elite_percentage_survivor_selection,
        "combine_parents_and_offspring": $combine_parents_and_offspring,
        "tournament_size": $tournament_size_survivor_selection,
        "tournament_probability": $tournament_probability_survivor_selection
    }
    }
    ''')
    return config.safe_substitute(
        train_instance = train_instance,
        log_file = log_file,
        output_file = output_file,
        init_heuristic = init_heuristic,
        population_size = population_size,
        number_of_generations = number_of_generations,
        parent_selection = parent_selection,
        tournament_size_parent_selection = tournament_size_parent_selection,
        tournament_probability_parent_selection = tournament_probability_parent_selection,
        elite_percentage_parent_selection = elite_percentage_parent_selection,
        cross_over_probability_partiallyMappedCrossover = cross_over_probability_partiallyMappedCrossover,
        cross_over_probability_orderOneCrossover = cross_over_probability_orderOneCrossover,
        cross_over_probability_edgeRecombination = cross_over_probability_edgeRecombination,
        mutation_probability_reassignOnePatient = mutation_probability_reassignOnePatient,
        mutation_probability_moveWithinJourney = mutation_probability_moveWithinJourney,
        mutation_probability_swapBetweenJourneys = mutation_probability_swapBetweenJourneys,
        mutation_probability_swapWithinJourney = mutation_probability_swapWithinJourney,
        mutation_probability_insertionHeuristic = mutation_probability_insertionHeuristic, 
        percentage_to_slice = percentage_to_slice, 
        survivor_selection = survivor_selection, 
        elite_percentage_survivor_selection = elite_percentage_survivor_selection,
        combine_parents_and_offspring = combine_parents_and_offspring,
        tournament_size_survivor_selection = tournament_size_survivor_selection,
        tournament_probability_survivor_selection = tournament_probability_survivor_selection
        
    )



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


class Config: 
    def __init__(self, name) -> None:
        self.name = name
        self.output_file = f"./outputs/{name}.json"
        self.log_file = f"./logs/{name}.log"
        # select random values from the list
        self.init_heuristic = init_heuristic[random.randint(0, init_heuristic.__len__()-1)]
        self.population_size = population_size[random.randint(0, population_size.__len__()-1)]
        self.number_of_generations = number_of_generations[random.randint(0, number_of_generations.__len__()-1)]
        self.parent_selection = parent_selection[random.randint(0, parent_selection.__len__()-1)]
        self.tournament_size_parent_selection = tournament_size_parent_selection[random.randint(0, tournament_size_parent_selection.__len__()-1)]
        self.tournament_probability_parent_selection = tournament_probability_parent_selection[random.randint(0, tournament_probability_parent_selection.__len__()-1)]
        self.elite_percentage_parent_selection = elite_percentage_parent_selection[random.randint(0, elite_percentage_parent_selection.__len__()-1)]
        self.cross_over_probability_partiallyMappedCrossover = cross_over_probability[random.randint(0, cross_over_probability.__len__()-1)]
        self.cross_over_probability_orderOneCrossover = cross_over_probability[random.randint(0, cross_over_probability.__len__()-1)]
        self.cross_over_probability_edgeRecombination = cross_over_probability[random.randint(0, cross_over_probability.__len__()-1)]
        self.mutation_probability_reassignOnePatient = mutation_probability[random.randint(0, mutation_probability.__len__()-1)]
        self.mutation_probability_moveWithinJourney = mutation_probability[random.randint(0, mutation_probability.__len__()-1)]
        self.mutation_probability_swapBetweenJourneys = mutation_probability[random.randint(0, mutation_probability.__len__()-1)]
        self.mutation_probability_swapWithinJourney = mutation_probability[random.randint(0, mutation_probability.__len__()-1)]
        self.mutation_probability_insertionHeuristic = mutation_probability[random.randint(0, mutation_probability.__len__()-1)]
        self.percentage_to_slice = percentage_to_slice[random.randint(0, percentage_to_slice.__len__()-1)]
        self.survivor_selection = survivor_selection[random.randint(0, survivor_selection.__len__()-1)]
        self.elite_percentage_survivor_selection = elite_percentage_survivor_selection[random.randint(0, elite_percentage_survivor_selection.__len__()-1)]
        self.combine_parents_and_offspring = combine_parents_and_offspring[random.randint(0, combine_parents_and_offspring.__len__()-1)]
        self.tournament_size_survivor_selection = tournament_size_survivor_selection[random.randint(0, tournament_size_survivor_selection.__len__()-1)]
        self.tournament_probability_survivor_selection = tournament_probability_survivor_selection[random.randint(0, tournament_probability_survivor_selection.__len__()-1)]

    def __str__(self) -> str:
        return get_congig(
            train_instance = -1,
            log_file = self.log_file,
            output_file = self.output_file,
            init_heuristic = self.init_heuristic,
            population_size = self.population_size,
            number_of_generations = self.number_of_generations,
            parent_selection = self.parent_selection,
            tournament_size_parent_selection = self.tournament_size_parent_selection,
            tournament_probability_parent_selection = self.tournament_probability_parent_selection,
            elite_percentage_parent_selection = self.elite_percentage_parent_selection,
            cross_over_probability_partiallyMappedCrossover = self.cross_over_probability_partiallyMappedCrossover,
            cross_over_probability_orderOneCrossover = self.cross_over_probability_orderOneCrossover,
            cross_over_probability_edgeRecombination = self.cross_over_probability_edgeRecombination,
            mutation_probability_reassignOnePatient = self.mutation_probability_reassignOnePatient,
            mutation_probability_moveWithinJourney = self.mutation_probability_moveWithinJourney,
            mutation_probability_swapBetweenJourneys = self.mutation_probability_swapBetweenJourneys,
            mutation_probability_swapWithinJourney = self.mutation_probability_swapWithinJourney,
            mutation_probability_insertionHeuristic = self.mutation_probability_insertionHeuristic, 
            percentage_to_slice = self.percentage_to_slice, 
            survivor_selection = self.survivor_selection, 
            elite_percentage_survivor_selection = self.elite_percentage_survivor_selection,
            combine_parents_and_offspring = self.combine_parents_and_offspring,
            tournament_size_survivor_selection = self.tournament_size_survivor_selection,
            tournament_probability_survivor_selection = self.tournament_probability_survivor_selection
        )

    def run_tests(self): 
        statistics = {}
        for i in train_instance: 
            config = get_congig(
                train_instance = i,
                log_file = self.log_file,
                output_file = self.output_file,
                init_heuristic = self.init_heuristic,
                population_size = self.population_size,
                number_of_generations = self.number_of_generations,
                parent_selection = self.parent_selection,
                tournament_size_parent_selection = self.tournament_size_parent_selection,
                tournament_probability_parent_selection = self.tournament_probability_parent_selection,
                elite_percentage_parent_selection = self.elite_percentage_parent_selection,
                cross_over_probability_partiallyMappedCrossover = self.cross_over_probability_partiallyMappedCrossover,
                cross_over_probability_orderOneCrossover = self.cross_over_probability_orderOneCrossover,
                cross_over_probability_edgeRecombination = self.cross_over_probability_edgeRecombination,
                mutation_probability_reassignOnePatient = self.mutation_probability_reassignOnePatient,
                mutation_probability_moveWithinJourney = self.mutation_probability_moveWithinJourney,
                mutation_probability_swapBetweenJourneys = self.mutation_probability_swapBetweenJourneys,
                mutation_probability_swapWithinJourney = self.mutation_probability_swapWithinJourney,
                mutation_probability_insertionHeuristic = self.mutation_probability_insertionHeuristic, 
                percentage_to_slice = self.percentage_to_slice, 
                survivor_selection = self.survivor_selection, 
                elite_percentage_survivor_selection = self.elite_percentage_survivor_selection,
                combine_parents_and_offspring = self.combine_parents_and_offspring,
                tournament_size_survivor_selection = self.tournament_size_survivor_selection,
                tournament_probability_survivor_selection = self.tournament_probability_survivor_selection
            )
            
            # write the config to a file
            with open(f"./configs/{self.name}.json", "w") as file: 
                file.write(config)

            result = subprocess.run(["./target/release/bio-ai-2", f"./configs/{self.name}.json"], stdout=subprocess.PIPE)
            # wait for the program to finish
            if result.returncode != 0:
                print(f"Error in config: {self.name}")
                return
            else: 
                print(f"Config: {self.name} finished iteration {i}")
                with open(self.output_file, "r") as file: 
                    output_file_content = file.read()
                    output_json = json.loads(output_file_content)
                    statistics[f"{i}"] = output_json[1]
        # write the statistics to the log file
        logger.info('{"Config": %s, "Statistics": %s},', self, statistics)
        # remove the config file
        subprocess.run(["rm", f"./configs/{self.name}.json"])


logpath = "./hyperparameter_tuning.log"
logger = logging.getLogger('log')
logger.setLevel(logging.INFO)
ch = logging.FileHandler(logpath)
ch.setFormatter(logging.Formatter('%(message)s'))
logger.addHandler(ch)     
logger.info("[")      

print("Number of cpu : ", multiprocessing.cpu_count())
# create thread pool with as many threads as there are in the pc
thread_pool = ThreadPool(multiprocessing.cpu_count() - 1)

# while not interupted by the user run number of threads configs in parrallel

def signal_handler(signal, frame):
    print('You pressed Ctrl+C! Waiting for threads to finish...')
    thread_pool.terminate()
    logger.info("]")
    print('All threads finished')
    sys.exit(0)

signal.signal(signal.SIGINT, signal_handler)

# create a list of configs
configs = [Config(f"config{i}") for i in range(150000)]
# run the tests
thread_pool.map(Config.run_tests, configs)


