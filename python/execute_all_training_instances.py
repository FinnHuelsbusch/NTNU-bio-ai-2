import os
import subprocess
import json

# read config 
with open('configs/config.json') as f:
    config = json.load(f)

    for filename in os.listdir('train'):
        if filename.endswith(".json"):
            # replace   "problem_instance":  in config with "problem_instance": "./train/"+filename,
            config['problem_instance'] = "./train/"+filename
            config['log_file'] = "./logs/"+filename+".log"
            config['output_file'] = "./outputs/"+filename
            config['early_stopping'] = True

            # write the new config to a file
            with open('configs/temp_config.json', 'w') as f:
                json.dump(config, f)
            # execute the training
            result = subprocess.run(['./target/release/bio-ai-2', './configs/temp_config.json'], stdout=subprocess.DEVNULL)
            if result.returncode != 0:
                print("Error in training: ", filename)
                break
            else:
                # open log file and read the result
                with open("./logs/"+filename+".log") as f:
                    content = f.readlines()
                    # get all lines that contain Genome: Name: Fittest Generation:
                    result = [x for x in content if "Genome: Name: Fittest Generation:" in x]
                    generation = result[-1].split("Generation: ")[1].split(" ")[0]
                    #fitness: -1404.7211518389558, travel_time: 1404.7211518389558, missin
                    travel_time = content[-1].split("travel_time: ")[1].split(",")[0]
                    print(f"File: {filename} finished at generation {generation} with travel time {travel_time}")

            # remove temp config file
            os.remove('configs/temp_config.json')
            


