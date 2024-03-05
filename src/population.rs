use crate::{config::{self, Config}, individual::Individual, problem_instance::{self, ProblemInstance}};


pub type Population = Vec<Individual>;

auto initializeFeasiblePopulation(const ProblemInstance &problemInstance, const Config &config) -> Population
{
    
}

pub fn initialize_random_population() -> Population {

}

pub fn initialize_valid_population(problem_instance: ProblemInstance, config: Config) -> Population {
    let population: Population = vec![];
    
    // Seed the random number generator
    RandomGenerator &rng = RandomGenerator::getInstance();

    std::map<int, std::vector<const Patient *>> PatientsByEndTime;
    for (const auto &[id, patient] : problemInstance.patients)
    {
        PatientsByEndTime[patient.endTime].push_back(&patient);
    }
    std::vector<int> startTimes;
    startTimes.reserve(PatientsByEndTime.size());
    for (const auto &[startTime, patients] : PatientsByEndTime)
    {
        startTimes.push_back(startTime);
    }
    // sort the start times
    std::sort(startTimes.begin(), startTimes.end());

    for (int i = 0; i < config.populationSize; i++)
    {
        // copy the  PatientsByEndTime
        std::map<int, std::vector<const Patient *>> PatientsByEndTimeCopy(PatientsByEndTime.begin(), PatientsByEndTime.end());
        // init genome with number of nurses x empty vector
        Genome genome = std::vector<std::vector<int>>(problemInstance.numberOfNurses);
        int currentStartTimeIndex = 0;
        int currentStartTime = startTimes[currentStartTimeIndex];
        int index;

        for (int j = 0; j < problemInstance.patients.size(); j++)
        {

            index = rng.generateRandomInt(0, PatientsByEndTimeCopy[currentStartTime].size() - 1);
            const Patient *patient = PatientsByEndTimeCopy[currentStartTime][index];
            PatientsByEndTimeCopy[currentStartTime].erase(PatientsByEndTimeCopy[currentStartTime].begin() + index);
            // insert patient in genome
            int minDetour = INT_MAX;
            int minDetourIndex = -1;
            for (int k = 0; k < genome.size(); k++)
            {
                if (genome[k].empty())
                {
                    minDetourIndex = k;
                    minDetour = problemInstance.travelTime[k][patient->id] + problemInstance.travelTime[patient->id][0];
                }
                else
                {
                    genome[k].push_back(patient->id);
                    if (isJourneyValid(genome[k], problemInstance))
                    {
                        int detour = problemInstance.travelTime[k - 1][patient->id] + problemInstance.travelTime[patient->id][0] - problemInstance.travelTime[k - 1][0];
                        if (detour < minDetour)
                        {
                            minDetour = detour;
                            minDetourIndex = k;
                        }
                    }
                    genome[k].pop_back();
                }
            }
            if (minDetourIndex != -1)
            {
                genome[minDetourIndex].push_back(patient->id);
            }
            else
            {
                break;
            }

            // check if the current start time is empty
            if (PatientsByEndTimeCopy[currentStartTime].empty())
            {
                // remove the current start time from the map
                PatientsByEndTimeCopy.erase(currentStartTime);
                // check if the map is empty
                if (PatientsByEndTimeCopy.empty())
                {
                    break;
                }
                currentStartTimeIndex++;
                currentStartTime = startTimes[currentStartTimeIndex];
            }
        }
        // Create the Individual
        if (isSolutionValid(genome, problemInstance))
        {
            Individual individual = {genome};
            evaluateIndividual(&individual, problemInstance);
            pop.push_back(individual);
        }
        else
        {
            i--;
        }
    }
    return pop;
}
