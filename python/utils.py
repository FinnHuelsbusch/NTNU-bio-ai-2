


import json
from string import Template
import logging
import subprocess


logpath = "./hyperparameter_tuning.log"
logger = logging.getLogger('log')
logger.setLevel(logging.INFO)
ch = logging.FileHandler(logpath)
ch.setFormatter(logging.Formatter('%(message)s'))
logger.addHandler(ch)   

class Config: 
    def __init__(self, 
                name: str,
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
                combine_parents_and_offspring: bool,
                tournament_size_survivor_selection: int,
                tournament_probability_survivor_selection: float
                    
                 ) -> None:
        self.name = name
        self.output_file = f"./outputs/{name}.json"
        self.log_file = f"./logs/{name}.log"
        # select random values from the list
        self.init_heuristic = init_heuristic
        self.population_size = population_size
        self.number_of_generations = number_of_generations
        self.parent_selection = parent_selection
        self.tournament_size_parent_selection = tournament_size_parent_selection
        self.tournament_probability_parent_selection = tournament_probability_parent_selection
        self.elite_percentage_parent_selection = elite_percentage_parent_selection
        self.cross_over_probability_partiallyMappedCrossover = cross_over_probability_partiallyMappedCrossover
        self.cross_over_probability_orderOneCrossover = cross_over_probability_orderOneCrossover
        self.cross_over_probability_edgeRecombination = cross_over_probability_edgeRecombination
        self.mutation_probability_reassignOnePatient = mutation_probability_reassignOnePatient
        self.mutation_probability_moveWithinJourney = mutation_probability_moveWithinJourney
        self.mutation_probability_swapBetweenJourneys = mutation_probability_swapBetweenJourneys
        self.mutation_probability_swapWithinJourney = mutation_probability_swapWithinJourney
        self.mutation_probability_insertionHeuristic = mutation_probability_insertionHeuristic
        self.percentage_to_slice = percentage_to_slice
        self.survivor_selection = survivor_selection
        self.elite_percentage_survivor_selection = elite_percentage_survivor_selection
        self.combine_parents_and_offspring = combine_parents_and_offspring
        self.tournament_size_survivor_selection = tournament_size_survivor_selection
        self.tournament_probability_survivor_selection = tournament_probability_survivor_selection

    def from_json(json: dict):
        return Config(
            name = json["output_file"].split("/")[-1].split(".")[0],
            init_heuristic = json["population_initialisation"],
            population_size = json["population_size"],
            number_of_generations = json["number_of_generations"],
            parent_selection = json["parent_selection"]["name"], 
            tournament_size_parent_selection = json["parent_selection"]["tournament_size"],
            tournament_probability_parent_selection = json["parent_selection"]["tournament_probability"],
            elite_percentage_parent_selection = json["parent_selection"]["elitism_percentage"],
            cross_over_probability_partiallyMappedCrossover = json["crossovers"][0]["probability"],
            cross_over_probability_orderOneCrossover = json["crossovers"][1]["probability"],
            cross_over_probability_edgeRecombination = json["crossovers"][2]["probability"],
            mutation_probability_reassignOnePatient = json["mutations"][0]["probability"],
            mutation_probability_moveWithinJourney = json["mutations"][1]["probability"],
            mutation_probability_swapBetweenJourneys = json["mutations"][2]["probability"],
            mutation_probability_swapWithinJourney = json["mutations"][3]["probability"],
            mutation_probability_insertionHeuristic = json["mutations"][4]["probability"],
            percentage_to_slice = json["mutations"][4]["percentage_to_slice"],
            survivor_selection = json["survivor_selection"]["name"],
            elite_percentage_survivor_selection = json["survivor_selection"]["elitism_percentage"],
            combine_parents_and_offspring = json["survivor_selection"]["combine_parents_and_offspring"],
            tournament_size_survivor_selection = json["survivor_selection"]["tournament_size"],
            tournament_probability_survivor_selection = json["survivor_selection"]["tournament_probability"]
        )

    def get_config_str(self, train_instance) -> str:
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
    
    def __str__(self) -> str:
        return self.get_config_str(train_instance=-1)

    def run_tests(self): 
        print(f"Running tests for config: {self.name}")
        statistics = {}
        for i in range(10): 
            config = self.get_config_str(train_instance=i)
            
            # write the config to a file
            with open(f"./configs/{self.name}.json", "w") as file: 
                file.write(config)

            result = subprocess.run(["./target/release/bio-ai-2", f"./configs/{self.name}.json"], stdout=subprocess.PIPE)
            # wait for the program to finish
            if result.returncode != 0:
                print(f"Error in config: {self.name}")
                return
            else: 
                with open(self.output_file, "r") as file: 
                    output_file_content = file.read()
                    output_json = json.loads(output_file_content)
                    statistics[f"{i}"] = output_json[1]
        # write the statistics to the log file
        logger.info('{"Config": %s, "Statistics": %s},', self, statistics)
        print(f"Finished tests for config: {self.name}")
