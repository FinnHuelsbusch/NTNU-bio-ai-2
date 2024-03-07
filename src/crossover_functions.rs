use std::collections::HashSet;

use rand::Rng;

use crate::{
    config::Config,
    individual::{ calculate_fitness, unflattened_genome, Genome, Individual },
    population::Population,
    problem_instance::ProblemInstance,
};

fn order_one_rossover (genome_a: &Genome, genome_b: &Genome) -> (Genome, Option<Genome>) {
    let genome_flattend_a: Vec<&usize> = genome_a.iter().flatten().collect();
    let genome_flattend_b: Vec<&usize> = genome_b.iter().flatten().collect();

    // assert that the genomes have the same length
    assert_eq!(genome_flattend_a.len(), genome_flattend_b.len());
    let genome_length: usize = genome_flattend_a.len();

    let mut rng = rand::thread_rng();
    let start: usize = rng.gen_range(0..genome_length);
    let end: usize = rng.gen_range(0..genome_length);

    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    let mut child_a: Vec<usize> = vec![0; genome_length];
    let mut child_b: Vec<usize> = vec![0; genome_length];

    // copy the selected part from parent1 to child1 and the selected part from parent2 to child2
    for i in start..=end {
        child_a[i] = *genome_flattend_a[i];
        child_b[i] = *genome_flattend_b[i];
    }

    let number_of_non_selected_elements = genome_length - (end - start);
    for (child, parent, other_parent) in &mut [
        (&mut child_a, genome_a, genome_b),
        (&mut child_b, genome_b, genome_a),
    ] {
        
        for i in 0..number_of_non_selected_elements {
            let source_index = (end + i) % genome_length;
            let mut target_index = source_index;
            while child.contains(&parent.iter().flatten().nth(target_index).unwrap()) {
                target_index = (target_index + 1) % genome_length;
            }
            child[source_index] = parent.iter().flatten().nth(target_index).unwrap().clone();
        }
    }

    return (
        unflattened_genome(&child_a, genome_a), 
        Some(unflattened_genome(&child_b, genome_b))
    );
}

/*
auto partiallyMappedCrossover(const Genome &parent1, const Genome &parent2) -> std::pair<Genome, std::optional<Genome>>
{
    std::vector<int> parent1Flat = flattenGenome(parent1);
    std::vector<int> parent2Flat = flattenGenome(parent2);

    RandomGenerator& rng = RandomGenerator::getInstance();
    std::size_t start = rng.generateRandomInt(0, parent1Flat.size() - 1);
    std::size_t end = rng.generateRandomInt(0, parent1Flat.size() - 1);
    if (start > end) {
        std::swap(start, end);
    }
    std::vector<int> child1Flat = std::vector<int>(parent1Flat.size(), -1);
    std::vector<int> child2Flat = std::vector<int>(parent2Flat.size(), -1);

    // copy the selected part from parent1 to child1 and the selected part from parent2 to child2
    std::vector<int> previousIndices;
    for (int i = start; i <= end; i++) {
        child1Flat[i] = parent1Flat[i];
        child2Flat[i] = parent2Flat[i];
    }

    for (int i = start; i <= end; i++) {
        int index = i; 
        previousIndices.clear();
        // check if the value is already in the selected part
        if (std::find(child1Flat.begin(), child1Flat.end(), parent2Flat[i]) != child1Flat.end()) {          
            // the value is already in the selected part
            continue;
        }
        do {
            previousIndices.push_back(index);
            auto iterator = std::find(parent2Flat.begin(), parent2Flat.end(), parent1Flat[index]);
            index = iterator - parent2Flat.begin();
        } while ((start <= index && index <= end && std::find(previousIndices.begin(), previousIndices.end(), index) == previousIndices.end()) || child1Flat[index] != -1);
        
        child1Flat[index] = parent2Flat[i];
    }

    for (int i = start; i <= end; i++) {
        
        int index = i;
        previousIndices.clear();
        if (std::find(child2Flat.begin(), child2Flat.end(), parent1Flat[i]) != child2Flat.end()) {
            continue;
        }
        do {
            previousIndices.push_back(index);
            auto iterator = std::find(parent1Flat.begin(), parent1Flat.end(), parent2Flat[index]);
            index = iterator - parent1Flat.begin();
        } while ((start <= index && index <= end && std::find(previousIndices.begin(), previousIndices.end(), index) == previousIndices.end()) || child2Flat[index] != -1);
        child2Flat[index] = parent1Flat[i];
        
    }

    // fill the rest of the child with the remaining genes from the other parent
    for (int i = 1; i < parent1Flat.size(); i++) {
        int index = (i + end) % parent1Flat.size();
        // check if the value is undefined
        if (child1Flat[index] == -1) {
            int targetIndex = index; 
            while (std::find(child1Flat.begin(), child1Flat.end(), parent2Flat[targetIndex]) != child1Flat.end()) {
                targetIndex = (targetIndex + 1) % parent1Flat.size();
            }
            child1Flat[index] = parent2Flat[targetIndex];
        }

        if (child2Flat[index] == -1) {
            int targetIndex = index; 
            while (std::find(child2Flat.begin(), child2Flat.end(), parent1Flat[targetIndex]) != child2Flat.end()) {
                targetIndex = (targetIndex + 1) % parent2Flat.size();
            }
            child2Flat[index] = parent1Flat[targetIndex];
        }
    }

    Genome child1 = unflattenGenome(child1Flat, parent1); 
    Genome child2 = unflattenGenome(child2Flat, parent2);
    return std::make_pair(child1, child2);
}

*/

fn partially_mapped_crossover(genome_a: &Genome, genome_b: &Genome) -> (Genome, Option<Genome>) {
    let genome_flattend_a: Vec<&usize> = genome_a.iter().flatten().collect();
    let genome_flattend_b: Vec<&usize> = genome_b.iter().flatten().collect();


    assert_eq!(genome_flattend_a.len(), genome_flattend_b.len());
    let genome_length: usize = genome_flattend_a.len();

    let mut rng = rand::thread_rng();
    let start: usize = rng.gen_range(0..genome_length);
    let end: usize = rng.gen_range(0..genome_length);

    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    let mut child_a: Vec<usize> = vec![0; genome_length];
    let mut child_b: Vec<usize> = vec![0; genome_length];

    // copy the selected part from parent1 to child1 and the selected part from parent2 to child2
    for i in start..=end {
        child_a[i] = *genome_flattend_a[i];
        child_b[i] = *genome_flattend_b[i];
    }

    let number_of_non_selected_elements = genome_length - (end - start);
    for (child, parent, other_parent) in &mut [
        (&mut child_a, &genome_flattend_a, &genome_flattend_b),
        (&mut child_b, &genome_flattend_b, &genome_flattend_a)
    ] {
        for i in start..=end {
            if child.contains(&other_parent.iter().nth(i).unwrap()) {
                continue;
            }else {
                let mut index_to_insert = i;
                let mut previous_indices: Vec<usize> = Vec::new();
                loop {
                    previous_indices.push(index_to_insert);
                    index_to_insert = other_parent.iter().position(|&x| x == parent[index_to_insert]).unwrap();
                    if (index_to_insert < start && index_to_insert > end) // index outside of selected range
                        && !!!previous_indices.contains(&index_to_insert) // index not already used -> no cycle
                        && child[index_to_insert] == 0 // location is not already used in child
                    {
                        child[index_to_insert] = *other_parent[i];
                        break;
                    }
                }
                
            }
        }
    }

    return (
        unflattened_genome(&child_a, genome_a), 
        Some(unflattened_genome(&child_b, genome_b))
    );
}

/*
auto edgeRecombination(const Genome &parent1, const Genome &parent2) -> std::pair<Genome, std::optional<Genome>>
{
    std::vector<int> parent1Flat = flattenGenome(parent1);
    std::vector<int> parent2Flat = flattenGenome(parent2);
    // assert that the genomes have the same length
    assert(parent1Flat.size() == parent2Flat.size());

    std::map<int, std::vector<int>> adjacencyList;
    for (int patientID : parent1Flat)
    {
        adjacencyList[patientID] = std::vector<int>();
    }
    for (int i = 0; i < parent1Flat.size(); i++) {
        int left = (i - 1 + parent1Flat.size()) % parent1Flat.size();
        int right = (i + 1) % parent1Flat.size();
        adjacencyList[parent1Flat[i]].push_back(parent1Flat[left]);
        adjacencyList[parent1Flat[i]].push_back(parent1Flat[right]);
        adjacencyList[parent2Flat[i]].push_back(parent2Flat[left]);
        adjacencyList[parent2Flat[i]].push_back(parent2Flat[right]);
    }

    RandomGenerator& rng = RandomGenerator::getInstance();
    std::vector<int> child;
    int current = parent1Flat[rng.generateRandomInt(0, parent1Flat.size() - 1)];
    for (int i = 0; i < parent1Flat.size(); i++) {
        child.push_back(current);
        for (auto& [key, value] : adjacencyList) {
            value.erase(std::remove(value.begin(), value.end(), current), value.end());
        }
        
        //Examine list for current element:
            // – If there is a common edge, pick that to be next element
            // – Otherwise pick the entry in the list which itself has the shortest list
            // – Ties are split at random
        int newCurrent = INT_MAX;
        std::set<int> seen = std::set<int>();
        for (int value : adjacencyList[current]) {
            if (seen.contains(value)) {
                newCurrent = value;
                break;
            }
            seen.insert(value);
        }

        // choice of new current is not random if there are two list of equal length. 
        if (newCurrent == INT_MAX) {
            int minSize = INT_MAX;
            for (int key : adjacencyList[current]) {
                std::set<int> valueSet = std::set<int>(adjacencyList[key].begin(), adjacencyList[key].end());
                if (valueSet.size() <= minSize) {
                    minSize = valueSet.size();
                    newCurrent = key;
                }
            }
        }
        adjacencyList.erase(current);
        if (adjacencyList.empty()) {
            break;
        }
        if (newCurrent == INT_MAX) {
            do{
                newCurrent = parent1Flat[rng.generateRandomInt(0, parent1Flat.size() - 1)];
            } while (adjacencyList.find(newCurrent) == adjacencyList.end());
        }
        current = newCurrent;
    }
    return std::make_pair(unflattenGenome(child, parent1), std::nullopt);
}
 */

fn edge_recombination(genome_a: &Genome, genome_b: &Genome) -> (Genome, Option<Genome>) {
    // Flatten genomes into 1d vector
    let genome_flattend_a: Vec<&usize> = genome_a.iter().flatten().collect();
    let genome_flattend_b: Vec<&usize> = genome_b.iter().flatten().collect();

    let genome_length: usize = genome_flattend_a.len();

    assert_eq!(genome_flattend_a.len(), genome_flattend_b.len());

    let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); genome_length];

    for index in 0..genome_length {
        let left = (index - 1 + genome_length) % genome_length;
        let right = (index + 1) % genome_length;
        adjacency_list[*genome_flattend_a[index]].push(*genome_flattend_a[left]);
        adjacency_list[*genome_flattend_a[index]].push(*genome_flattend_a[right]);
        adjacency_list[*genome_flattend_b[index]].push(*genome_flattend_b[left]);
        adjacency_list[*genome_flattend_b[index]].push(*genome_flattend_b[right]);
    }

    let mut rng = rand::thread_rng();
    let mut child: Vec<usize> = Vec::with_capacity(genome_length);
    let mut current = *genome_flattend_a[rng.gen_range(0..genome_length)];

    for _ in 0..genome_length {
        child.push(current);

        for value in &mut adjacency_list {
            value.retain(|&x| x != current);
        }

        //Examine list for current element:
        // – If there is a common edge, pick that to be next element
        // – Otherwise pick the entry in the list which itself has the shortest list
        // – Ties are split at random

        let mut new_current: usize = usize::MAX;
        let mut seen: HashSet<usize> = HashSet::new();

        for value in &mut adjacency_list[current] {
            if seen.contains(&value) {
                new_current = *value;
                break;
            }
            seen.insert(*value);
        }

        // choice of new current is not random if there are two list of equal length.
        if new_current == usize::MAX {
            for value in &adjacency_list[current] {
                let mut min_size = usize::MAX;
                let value_set: HashSet<&usize> = HashSet::from_iter(adjacency_list[current].iter());
                if value_set.len() <= min_size {
                    min_size = value_set.len();
                    new_current = *value;
                }
            }
        }

        adjacency_list.remove(current);

        if adjacency_list.is_empty() {
            break;
        }

        if new_current == usize::MAX {
            loop {
                new_current = *genome_flattend_a[rng.gen_range(0..genome_length)];
                if new_current == adjacency_list.len() - 1 {
                    break;
                }
            }
        }
        current = new_current;
    }

    return (unflattened_genome(&child, genome_a), None);
}

pub fn crossover(
    population: &mut Population,
    problem_instance: &ProblemInstance,
    config: &Config
) -> Population {
    let mut rng = rand::thread_rng();
    let mut children: Population = population.clone();
    for crossover_config in config.crossovers.iter() {
        // Calculate the number of crossovers which should happen for the specific config
        let number_of_crossovers: u64 = (
            (config.population_size as f64) * crossover_config.probability.unwrap_or(0.0)
        ).ceil() as u64;

        for _ in 0..number_of_crossovers {
            let individual_index_a: usize = rng.gen_range(0..config.population_size);
            let mut individual_index_b: usize = rng.gen_range(0..config.population_size);

            while individual_index_a == individual_index_b {
                individual_index_b = rng.gen_range(0..config.population_size);
            }

            let child_genomes: (Genome, Option<Genome>) = match
                config.parent_selection.name.as_str()
            {
                "edgeRecombination" =>
                    edge_recombination(
                        &children[individual_index_a].genome,
                        &children[individual_index_b].genome
                    ),
                "orderOneCrossover" =>
                    order_one_rossover(
                        &children[individual_index_a].genome,
                        &children[individual_index_b].genome
                    ),
                "partiallyMappedCrossover" =>
                    partially_mapped_crossover(
                        &children[individual_index_a].genome,
                        &children[individual_index_b].genome
                    ),

                // Handle the rest of cases
                _ =>
                    panic!(
                        "Didn't have an Implementation for selection function: {:?}",
                        config.parent_selection.name.as_str()
                    ),
            };

            let mut child_a = Individual::new(child_genomes.0);
            calculate_fitness(&mut child_a, problem_instance);
            children[individual_index_a] = child_a;

            match child_genomes.1 {
                Some(genome) => {
                    let mut child_b = Individual::new(genome);
                    calculate_fitness(&mut child_b, problem_instance);
                    children[individual_index_b] = child_b;
                }
                None => {}
            }
        }
    }

    children
}
