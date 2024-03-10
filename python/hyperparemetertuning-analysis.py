import json 
from utils import Config

benchmarks = { "0":  828, 
                "1":  828, 
                "2":  823,
                "3":  827,
                "4":  827,
                "5":  589,
                "6":  586,
                "7":  1049,
                "8":  1208,
                "9":  1262
}



# read hyperparameter_tuning.json
try: 
    with open('hyperparameter_tuning.json') as f:
        data = json.load(f)
except FileNotFoundError as e:
    print("The file hyperparameter_tuning.json does not exist. Please run the hyperparameter tuning script first and convert the output to json format.")
else: 
    # convert the config into Config objects
    configs = [Config.from_json(config["Config"]) for config in data]
    # place it into data 
    data = [{"Config": configs[i], "Statistics": data[i]["Statistics"]} for i in range(len(configs))]

    for config in data:
        for key, value in config["Statistics"].items():
            config["Statistics"][key]["travel_time_min_feasible_rel_dev"] = 100 / benchmarks[key] * (value["travel_time_min_feasible"] if value["travel_time_min_feasible"] != 0 else float("inf"))
            config["Statistics"][key]["travel_time_avg_feasible_rel_dev"] = 100 / benchmarks[key] * (value["travel_time_avg_feasible"] if value["travel_time_avg_feasible"] != 0 else float("inf"))
            config["Statistics"][key]["travel_time_max_feasible_rel_dev"] = 100 / benchmarks[key] * (value["travel_time_max_feasible"] if value["travel_time_max_feasible"] != 0 else float("inf"))
            config["Statistics"][key]["travel_time_min_all_rel_dev"] = 100 / benchmarks[key] * (value["travel_time_min_all"] if value["travel_time_min_all"] != 0 else float("inf"))
            config["Statistics"][key]["travel_time_avg_all_rel_dev"] = 100 / benchmarks[key] * (value["travel_time_avg_all"] if value["travel_time_avg_all"] != 0 else float("inf"))
            config["Statistics"][key]["travel_time_max_all_rel_dev"] = 100 / benchmarks[key] * (value["travel_time_max_all"] if value["travel_time_max_all"] != 0 else float("inf"))

        values = list(config["Statistics"].values())
        statistics_list_len = len(list(values))
        config["Statistics"]["travel_time_min_feasible_rel_dev_avg"] = sum([value["travel_time_min_feasible_rel_dev"] for value in values]) / statistics_list_len
        config["Statistics"]["travel_time_avg_feasible_rel_dev_avg"] = sum([value["travel_time_avg_feasible_rel_dev"] for value in values]) / statistics_list_len
        config["Statistics"]["travel_time_max_feasible_rel_dev_avg"] = sum([value["travel_time_max_feasible_rel_dev"] for value in values]) / statistics_list_len
        config["Statistics"]["travel_time_min_all_rel_dev_avg"] = sum([value["travel_time_min_all_rel_dev"] for value in values]) / statistics_list_len
        config["Statistics"]["travel_time_avg_all_rel_dev_avg"] = sum([value["travel_time_avg_all_rel_dev"] for value in values]) / statistics_list_len
        config["Statistics"]["travel_time_max_all_rel_dev_avg"] = sum([value["travel_time_max_all_rel_dev"] for value in values]) / statistics_list_len
    
    # sort the data by the travel_time_avg_feasible_rel_dev_avg
    data = sorted(data, key=lambda x: x["Statistics"]["travel_time_avg_feasible_rel_dev_avg"])
    # print the top 5 best configurations
    for i in range(5):
        print(f"Configuration {i+1}:")
        print(data[i]["Config"])
        # print avg relative deviation
        print("Aggregated Statistics:")
        print(f"travel_time_min_feasible_rel_dev_avg: {data[i]['Statistics']['travel_time_min_feasible_rel_dev_avg']}")
        print(f"travel_time_avg_feasible_rel_dev_avg: {data[i]['Statistics']['travel_time_avg_feasible_rel_dev_avg']}")
        print(f"travel_time_max_feasible_rel_dev_avg: {data[i]['Statistics']['travel_time_max_feasible_rel_dev_avg']}")
        print(f"travel_time_min_all_rel_dev_avg: {data[i]['Statistics']['travel_time_min_all_rel_dev_avg']}")
        print(f"travel_time_avg_all_rel_dev_avg: {data[i]['Statistics']['travel_time_avg_all_rel_dev_avg']}")
        print(f"travel_time_max_all_rel_dev_avg: {data[i]['Statistics']['travel_time_max_all_rel_dev_avg']}")
        print()
