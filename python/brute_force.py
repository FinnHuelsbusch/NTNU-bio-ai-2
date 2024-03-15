import argparse
import subprocess
import json

# add command line arguments
parser = argparse.ArgumentParser(description='Run the genetic algorithm in a brute force manner.')
parser.add_argument('--config-path', type=str, help='Path to the config file.', default='configs/config.json')
parser.add_argument('--start-index', type=int, help='Start index for the log files.', default=0)
arguments = parser.parse_args()

with open(arguments.config_path) as f:
    config = json.load(f)

for i in range(arguments.start_index, 300):
    # change log file
    config['log_file'] = "./logs/"+config['problem_instance'].split("/")[-1]+ '_' + str(i) + ".log"
    config['output_file'] = "./outputs/"+config['problem_instance'].split("/")[-1]+ '_' + str(i)

    # change early stopping
    config['early_stopping'] = False
    # write the new config to a file
    with open('configs/temp_config.json', 'w') as f:
        json.dump(config, f)
    # execute the training
    result = subprocess.run(['./target/release/bio-ai-2', './configs/temp_config.json'], stdout=subprocess.DEVNULL)
    if result.returncode != 0:
        print("Error in training.")
        break
    else:
        # open log file and read the result
        with open(config['log_file']) as f:
            content = f.readlines()
            # get all lines that contain Genome: Name: Fittest Generation:
            result = [x for x in content if "Genome: Name: Fittest Generation:" in x]
            generation = result[-1].split("Generation: ")[1].split(" ")[0]
            #fitness: -1404.7211518389558, travel_time: 1404.7211518389558, missin
            travel_time = content[-1].split("travel_time: ")[1].split(",")[0]
            print(f"File: {config['problem_instance']} finished at generation {generation} with travel time {travel_time}")