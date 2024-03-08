
from typing import List
from matplotlib.animation import FuncAnimation
import seaborn as sns
import matplotlib.pyplot as plt
import json

import pandas as pd
import matplotlib.pyplot as plt

import numpy as np
import matplotlib.patches as mpatches
import ast


class Individual:
    def __init__(self, filepath):
        with open(filepath) as f:
            self.data = json.load(f)
            self.fitness = self.data['fitness']
            self.genome = self.data['genome']


class Patient:
    def __init__(self, patient_id, demand, start_time, end_time, care_time, x_coord, y_coord):
        self.id = patient_id
        self.demand = demand
        self.startTime = start_time
        self.endTime = end_time
        self.careTime = care_time
        self.xCoord = x_coord
        self.yCoord = y_coord


class Depot:
    def __init__(self, x_coord, y_coord, return_time):
        self.xCoord = x_coord
        self.yCoord = y_coord
        self.returnTime = return_time


class ProblemInstance:
    def __init__(self, instance_name, number_of_nurses, nurse_capacity, benchmark, depot, patients, travel_time):
        self.instanceName = instance_name
        self.numberOfNurses = number_of_nurses
        self.nurseCapacity = nurse_capacity
        self.benchmark = benchmark
        self.depot = depot
        self.patients = patients
        self.travelTime = travel_time

    def load_instance(filename):
        with open(f"./train/{filename}", 'r') as file:
            data = json.load(file)

        instance_name = data["instance_name"]
        print(f"Loading instance: {instance_name}")

        # Load the depot
        depot = Depot(
            data["depot"]["x_coord"],
            data["depot"]["y_coord"],
            data["depot"]["return_time"]
        )

        # Load the patients
        number_of_patients = len(data["patients"])
        print(f"Number of patients: {number_of_patients}")

        patients = {}
        for patient_id, patient_data in data["patients"].items():
            patients[int(patient_id)] = Patient(
                int(patient_id),
                patient_data["demand"],
                patient_data["start_time"],
                patient_data["end_time"],
                patient_data["care_time"],
                patient_data["x_coord"],
                patient_data["y_coord"]
            )

        # Load the travel time matrix
        number_of_nurses = data["nbr_nurses"]
        nurse_capacity = data["capacity_nurse"]
        benchmark = data["benchmark"]

        travel_time_matrix = [
            [travel_time for travel_time in row] for row in data["travel_times"]
        ]

        problem_instance = ProblemInstance(
            instance_name,
            number_of_nurses,
            nurse_capacity,
            benchmark,
            depot,
            patients,
            travel_time_matrix
        )

        print(f"Done loading instance: {instance_name}")
        return problem_instance


def visualizeTripsOnMap(genome: List[List[int]], problem_instance: ProblemInstance, ax=None):

    if ax is None:
        # create a plot
        fig, ax = plt.subplots()
    # add the depot to the plot
    ax.scatter(problem_instance.depot.xCoord,
               problem_instance.depot.yCoord, color='red')
    # add the patients to the plot
    ax.scatter([patient.xCoord for patient in problem_instance.patients.values()], [
               patient.yCoord for patient in problem_instance.patients.values()], color='blue')
    # generate as many colors as there are nurses
    colors = sns.color_palette('hsv', problem_instance.numberOfNurses)
    # iterate through the different trips
    for trip, color in zip(genome, colors):
        if len(trip) == 0:
            continue
        # add the first trip to the plot as arrow
        ax.arrow(problem_instance.depot.xCoord, problem_instance.depot.yCoord, problem_instance.patients[trip[0]].xCoord - problem_instance.depot.xCoord,
                 problem_instance.patients[trip[0]].yCoord - problem_instance.depot.yCoord, head_width=0.5, head_length=0.5, fc=color, ec=color)
        # iterate through the trip
        for i in range(1, len(trip)):
            # add the trip to the plot as arrow
            ax.arrow(problem_instance.patients[trip[i-1]].xCoord, problem_instance.patients[trip[i-1]].yCoord, problem_instance.patients[trip[i]].xCoord - problem_instance.patients[trip[i-1]
                                                                                                                                                                                     ].xCoord, problem_instance.patients[trip[i]].yCoord - problem_instance.patients[trip[i-1]].yCoord, head_width=0.5, head_length=0.5, fc=color, ec=color)
        # add the return trip to the plot
        ax.arrow(problem_instance.patients[trip[-1]].xCoord, problem_instance.patients[trip[-1]].yCoord, problem_instance.depot.xCoord - problem_instance.patients[trip[-1]
                                                                                                                                                                   ].xCoord, problem_instance.depot.yCoord - problem_instance.patients[trip[-1]].yCoord, head_width=0.5, head_length=0.5, fc=color, ec=color)


def read_log_file_genome_development(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    logfile_data = {}
    for i, line in enumerate(lines):
        if 'INFO   Genome: Name: ' in line:

            thread_id = line.split('(')[1].split(')')[0]
            genome_name = line.split('Name: ')[1].split(
                'Generation: ')[0].strip()
            generation = line.split('Generation: ')[
                1].split('Genome: ')[0].strip()
            genome_string = line.split('Genome: ')[2].strip()
            # genome string to 2D list
            genome = ast.literal_eval(genome_string)
            thread_data = logfile_data.get(thread_id, {})
            genome_data = thread_data.get(genome_name, {})
            genome_data[generation] = genome
            thread_data[genome_name] = genome_data
            logfile_data[thread_id] = thread_data
    return logfile_data


def visualizeAsGantChart(individual: Individual, problem_instance: ProblemInstance):

    data = {
        'Task': [],
        'Start Time': [],
        'End Time': [],
        'Type': [],
        'Patient': [],
        'Demand': []
    }
    # iterate through the different trips
    for i, trip in enumerate(individual.genome):
        if len(trip) == 0:
            continue
        # iterate through the different patients in the trip
        for j in range(len(trip)):
            if j == 0:
                data['Task'].append(f'Nurse {i}')
                data['Start Time'].append(0)
                data['End Time'].append(
                    problem_instance.travelTime[0][trip[j]])
                data['Type'].append('Travel')
                data['Patient'].append(f'Depot->Patient{trip[j]}')
                data['Demand'].append(0)
            else:
                data['Task'].append(f'Nurse {i}')
                data['Start Time'].append(data['End Time'][-1])
                data['End Time'].append(
                    data['Start Time'][-1] + problem_instance.travelTime[trip[j-1]][trip[j]])
                data['Type'].append('Travel')
                data['Patient'].append(f'Patient{trip[j-1]}->Patient{trip[j]}')
                data['Demand'].append(0)
            # care for the patient
            data['Task'].append(f'Nurse {i}')
            data['Start Time'].append(
                max(problem_instance.patients[trip[j]].startTime, data['End Time'][-1]))
            data['End Time'].append(
                data['Start Time'][-1] + problem_instance.patients[trip[j]].careTime)
            data['Type'].append('Care')
            data['Patient'].append(f'Patient{trip[j]}')
            data['Demand'].append(problem_instance.patients[trip[j]].demand)

        data['Task'].append(f'Nurse {i}')
        data['Start Time'].append(data['End Time'][-1])
        data['End Time'].append(
            data['Start Time'][-1] + problem_instance.travelTime[trip[-1]][0])
        data['Type'].append('Travel')
        data['Patient'].append(f'Patient{trip[-1]}->Depot')
        data['Demand'].append(0)
    # create a dataframe
    df = pd.DataFrame(data)
    # Declaring a figure "gnt" with size20 x 15
    fig, gnt = plt.subplots(figsize=(20, 15))

    # Setting Y-axis limits
    gnt.set_ylim(0, df['Task'].unique().shape[0] * 10)

    # Setting X-axis limits
    gnt.set_xlim(0, df['End Time'].max())

    # Setting labels for x-axis and y-axis
    gnt.set_xlabel('Time units')
    gnt.set_ylabel('Nurse')

    # Setting ticks on y-axis
    gnt.set_yticks(np.arange(5, df['Task'].unique().shape[0] * 10, 10))
    # Labelling tickes of y-axis
    gnt.set_yticklabels(df['Task'].unique())

    # Setting graph attribute
    gnt.grid(True)

    for i, nurse in enumerate(df['Task'].unique()):
        # get travel times
        help = df[(df['Task'] == nurse) & (df['Type'] == 'Travel')]
        start_times = df[(df['Task'] == nurse) & (
            df['Type'] == 'Travel')]['Start Time'].to_list()
        durations = (df[(df['Task'] == nurse) & (df['Type'] == 'Travel')]['End Time'] -
                     df[(df['Task'] == nurse) & (df['Type'] == 'Travel')]['Start Time']).to_list()

        xranges = [(start, duration)
                   for start, duration in zip(start_times, durations)]
        yranges = (i * 10, 10)

        # Declaring a bar in schedule
        gnt.broken_barh(
            xranges, yranges, facecolors='tab:blue'
        )
        # annotate the travel times with the duration
        for xr in xranges:
            # limit to two decimals
            gnt.text(xr[0] + xr[1] / 2, yranges[0] + yranges[1] / 2,
                     f'{round(xr[1], 2)}', ha='center', va='center', color='white')

        # get care times
        start_times = df[(df['Task'] == nurse) & (
            df['Type'] == 'Care')]['Start Time'].to_list()
        durations = (df[(df['Task'] == nurse) & (df['Type'] == 'Care')]['End Time'] -
                     df[(df['Task'] == nurse) & (df['Type'] == 'Care')]['Start Time']).to_list()

        xranges = [(start, duration)
                   for start, duration in zip(start_times, durations)]
        yranges = (i * 10, 10)

        # Declaring a bar in schedule
        gnt.broken_barh(
            xranges, yranges, facecolors='tab:red'
        )
        # annotate the care times with the patient id
        for xr in xranges:
            trip = df[(df['Task'] == nurse) & (df['Type'] == 'Care')
                      & (df['Start Time'] == xr[0])]['Patient'].values[0]

            gnt.text(xr[0] + xr[1] / 2, yranges[0] + yranges[1] / 2,
                     f'{trip}', ha='center', va='center', color='white')

        # add sum of demand to the right y-axis
        care_times = df[(df['Task'] == nurse) & (
            df['Type'] == 'Care')]['Demand'].sum()
        gnt.text(df['End Time'].max() + 10, yranges[0] + yranges[1] / 2,
                 f'{care_times}', ha='center', va='center', color='black')

    # add lable to the right y-axis
    x_pos = df['End Time'].max() + 50
    y_pos = df['Task'].unique().shape[0] * 10 / 2
    gnt.text(x_pos, y_pos, 'Satisfied Demand', ha='center',
             va='center', color='black', rotation=-90)

    # add a legend
    red_patch = mpatches.Patch(color='tab:red', label='Care')
    blue_patch = mpatches.Patch(color='tab:blue', label='Travel')
    plt.legend(handles=[red_patch, blue_patch], bbox_to_anchor=(
        1.05, 1), loc='upper left', borderaxespad=0.)

    # display the plot
    # save df to csv ordered by nurse and start time
    df = df.sort_values(by=['Task', 'Start Time'])
    df.to_csv('./python/gantt.csv', index=False)


def animateTripsOnMap(genomes, problem_instance, filename='animation.gif'):

    # Create a figure and axis
    fig, ax = plt.subplots()

    # Initialize empty plot
    pos = {}
    edges = []

    # Function to update the plot for each version
    def update(version):
        ax.clear()
        for artist in ax.lines + ax.collections:
            artist.remove()
        ax.set_title(f"Version {version+1}")

        genome = genomes[version]
        visualizeTripsOnMap(genome, problem_instance, ax)

    # Create an animation
    animation = FuncAnimation(fig, update, frames=len(
        genomes), repeat=False, interval=250)

    # Save the animation to gif
    animation.save(filename, writer='imagemagick', fps=1)


def read_log_file_individual_statistics(filepath):
    with open(filepath, 'r') as file:
        lines = file.readlines()
    data = {}
    for line in lines:
        if 'Travel Time Best: ' in line:

            thread_id = line.split('(')[1].split(')')[0]
            best = float(line.split('Best: ')[1].split(' Avg:')[0])
            avg = float(line.split('Avg: ')[1].split(' Worst:')[0])
            worst = float(line.split('Worst: ')[1])
            thread_data = data.get(thread_id, {"Best Travel Time": [], "Avg Travel Time": [
            ], "Worst Travel Time": [], "Best Fitness": [], "Avg Fitness": [], "Worst Fitness": []})
            thread_data["Best Travel Time"].append(best)
            thread_data["Avg Travel Time"].append(avg)
            thread_data["Worst Travel Time"].append(worst)
            data[thread_id] = thread_data
        elif 'Fitness Best: ' in line:
            thread_id = line.split('(')[1].split(')')[0]
            best = float(line.split('Best: ')[1].split(' Avg:')[0])
            avg = float(line.split('Avg: ')[1].split(' Worst:')[0])
            worst = float(line.split('Worst: ')[1])
            thread_data = data.get(thread_id, {"Best Travel Time": [], "Avg Travel Time": [
            ], "Worst Travel Time": [], "Best Fitness": [], "Avg Fitness": [], "Worst Fitness": []})
            thread_data["Best Fitness"].append(best)
            thread_data["Avg Fitness"].append(avg)
            thread_data["Worst Fitness"].append(worst)
            data[thread_id] = thread_data
    return data


def visualize_thread_data(thread_id, thread_data):
    df = pd.DataFrame(thread_data)
    fig, ax = plt.subplots()
    sns.lineplot(data=df, x=df.index, y="Best Travel Time",
                 label="Best", color="green", ax=ax)
    sns.lineplot(data=df, x=df.index, y="Avg Travel Time",
                 label="Avg", color="blue", ax=ax)
    sns.lineplot(data=df, x=df.index, y="Worst Travel Time",
                 label="Worst", color="red", ax=ax)
    # label the axes
    ax.set_ylabel("Travel Time")
    # second y-axis
    ax2 = ax.twinx()
    sns.lineplot(data=df, x=df.index, y="Best Fitness",
                 label="Best", color="green", ax=ax2, linestyle='--')
    sns.lineplot(data=df, x=df.index, y="Avg Fitness",
                 label="Avg", color="blue", ax=ax2, linestyle='--')
    sns.lineplot(data=df, x=df.index, y="Worst Fitness",
                 label="Worst", color="red", ax=ax2, linestyle='--')
    ax2.set_ylabel("Fitness")
    plt.title(f"Thread {thread_id} - Best, Avg, Worst Fitness")
    plt.xlabel("Generation")
    plt.savefig('./metrics/fitness_' + thread_id + '.png')

    # fig, ax = plt.subplots()
    # sns.lineplot(data=df, x=df.index, y="Best Travel Time", label="Best", color="green", ax=ax)
    # sns.lineplot(data=df, x=df.index, y="Avg Travel Time", label="Avg", color="blue", ax=ax)
    # sns.lineplot(data=df, x=df.index, y="Worst Travel Time", label="Worst", color="red", ax=ax)
    # ax.set_ylabel("Travel Time")
    #     # second y-axis
    # ax2 = ax.twinx()
    # sns.lineplot(data=df, x=df.index, y="Diversity", label="Diversity", color="orange", ax=ax2, linestyle='--')
    # ax2.set_ylabel("Diversity")
    # plt.title(f"Thread {thread_id} - Best, Avg, Worst Travel Time and Diversity")
    # plt.xlabel("Generation")
    # plt.show()


if __name__ == "__main__":
    # load the problem instance
    problem_instance = ProblemInstance.load_instance("train_9.json")
    # load the individual
    individual = Individual("./python/solution.json")
    # visualize the individual
    visualizeAsGantChart(individual, problem_instance)
    plt.savefig('./metrics/trip_gant.png')
    visualizeTripsOnMap(individual.genome, problem_instance)
    plt.savefig('./metrics/trip_map.png')
    # read the log file
    logfile_data_genome_development = read_log_file_genome_development(
        './python/statistics_rust.txt')
    logfile_data_individual_statistics = read_log_file_individual_statistics(
        './python/statistics_rust.txt')
    # visualize the log file
    keys = list(logfile_data_genome_development.keys())
    for thread_id, thread_data in logfile_data_individual_statistics.items():
        df = pd.DataFrame(thread_data)
        fig, ax = plt.subplots()
        sns.lineplot(data=df, x=df.index, y="Best Travel Time",
                     label="Best", color="green", ax=ax)
        sns.lineplot(data=df, x=df.index, y="Avg Travel Time",
                     label="Avg", color="blue", ax=ax)
        sns.lineplot(data=df, x=df.index, y="Worst Travel Time",
                     label="Worst", color="red", ax=ax)
        # label the axes
        ax.set_ylabel("Travel Time")
        # second y-axis
        ax2 = ax.twinx()
        sns.lineplot(data=df, x=df.index, y="Best Fitness",
                     label="Best", color="green", ax=ax2, linestyle='--')
        sns.lineplot(data=df, x=df.index, y="Avg Fitness",
                     label="Avg", color="blue", ax=ax2, linestyle='--')
        sns.lineplot(data=df, x=df.index, y="Worst Fitness",
                     label="Worst", color="red", ax=ax2, linestyle='--')
        ax2.set_ylabel("Fitness")
        plt.title(f"Thread {thread_id} - Best, Avg, Worst Fitness")
        plt.xlabel("Generation")

        plt.show()
    # for thread_id, thread_data in logfile_data_genome_development.items():
    #     for genome_name, generations in thread_data.items():
    #         genomes = []
    #         for generation, genome in generations.items():
    #             genomes.append(genome)
    #         animateTripsOnMap(genomes, problem_instance, f'animation_{keys.index(thread_id)}_{genome_name}.gif')
