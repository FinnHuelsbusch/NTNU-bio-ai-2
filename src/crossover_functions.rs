use std::collections::{HashMap, HashSet};

use rand::Rng;

use crate::{
    config::Config,
    individual::{ calculate_fitness, unflattened_genome, Genome, Individual },
    population::Population,
    problem_instance::ProblemInstance,
};

fn order_one_crossover(genome_a: &Genome, genome_b: &Genome) -> (Genome, Option<Genome>) {
    let genome_flattened_a: Vec<&usize> = genome_a.iter().flatten().collect();
    let genome_flattened_b: Vec<&usize> = genome_b.iter().flatten().collect();

    // assert that the genomes have the same length
    assert_eq!(genome_flattened_a.len(), genome_flattened_b.len());
    let genome_length: usize = genome_flattened_a.len();

    let mut rng = rand::thread_rng();
    let start: usize = rng.gen_range(0..genome_length);
    let end: usize = rng.gen_range(0..genome_length);

    let (start, end) = if start > end { (end, start) } else { (start, end) };

    let mut child_a: Vec<usize> = vec![0; genome_length];
    let mut child_b: Vec<usize> = vec![0; genome_length];

    // copy the selected part from parent1 to child1 and the selected part from parent2 to child2
    for i in start..=end {
        child_a[i] = *genome_flattened_a[i];
        child_b[i] = *genome_flattened_b[i];
    }

    let number_of_non_selected_elements = genome_length - (end - start);
    for (child, other_parent) in &mut [
        (&mut child_a, &genome_flattened_b),
        (&mut child_b, &genome_flattened_a),
    ] {
        for i in 0..number_of_non_selected_elements-1 {
            let source_index = (end + i + 1) % genome_length;
            let mut target_index = source_index;
            while child.contains(other_parent.get(target_index).unwrap()) {
                target_index = (target_index + 1) % genome_length;
            }
            child[source_index] = *other_parent[target_index];
        }
    }

    (unflattened_genome(&child_a, genome_a), Some(unflattened_genome(&child_b, genome_b)))
}

fn partially_mapped_crossover(genome_a: &Genome, genome_b: &Genome) -> (Genome, Option<Genome>) {
    // Flatten genomes into 1d vector
    let genome_flattened_a: Vec<&usize> = genome_a.iter().flatten().collect();
    let genome_flattened_b: Vec<&usize> = genome_b.iter().flatten().collect();

    // assert that the genomes have the same length and save the length in a variable for later use
    assert_eq!(genome_flattened_a.len(), genome_flattened_b.len());
    let genome_length: usize = genome_flattened_a.len();

    // select a random start and end index which will be a direct copy from the parent to the child
    let mut rng = rand::thread_rng();
    let start: usize = rng.gen_range(0..genome_length);
    let end: usize = rng.gen_range(0..genome_length);
    // make sure that start is smaller than end
    let (start, end) = if start > end { (end, start) } else { (start, end) };

    // create two children with the same length as the genomes
    let mut child_a: Vec<usize> = vec![usize::MAX; genome_length];
    let mut child_b: Vec<usize> = vec![usize::MAX; genome_length];

    // copy the selected part from parent1 to child1 and the selected part from parent2 to child2
    for i in start..=end {
        child_a[i] = *genome_flattened_a[i];
        child_b[i] = *genome_flattened_b[i];
    }

    for (child, parent, other_parent) in &mut [
        (&mut child_a, &genome_flattened_a, &genome_flattened_b),
        (&mut child_b, &genome_flattened_b, &genome_flattened_a),
    ] {
        //
        for i in start..=end {
            if child.contains(other_parent.get(i).unwrap()) {
                continue;
            } else {
                let mut index_to_insert = i;
                let mut previous_indices: Vec<usize> = Vec::new();
                loop {
                    previous_indices.push(index_to_insert);
                    index_to_insert = other_parent
                        .iter()
                        .position(|&x| x == parent[index_to_insert])
                        .unwrap();
                    if
                        (index_to_insert < start || end < index_to_insert) && // index outside of selected range
                        !previous_indices.contains(&index_to_insert) && // index not already used -> no cycle
                        child[index_to_insert] == usize::MAX
                        // location is not already used in child
                    {
                        child[index_to_insert] = *other_parent[i];
                        break;
                    }
                }
            }
        }

        // fill the rest of the child with the elements from the other parent
        let mut i = 0;
        while i < genome_length {
            let insert_index = (i + end + 1) % genome_length;
            if child[insert_index] != usize::MAX {
                i += 1;
                continue;
            }
            let mut source_index = (i + end + 1) % genome_length;
            while child.contains(other_parent.get(source_index).unwrap()) {
                source_index = (source_index + 1) % genome_length;
            }
            child[insert_index] = **other_parent.get(source_index).unwrap();
        }
    }

    (unflattened_genome(&child_a, genome_a), Some(unflattened_genome(&child_b, genome_b)))
}

fn edge_recombination(genome_a: &Genome, genome_b: &Genome) -> (Genome, Option<Genome>) {
    // Flatten genomes into 1d vector
    let genome_flattened_a: Vec<&usize> = genome_a.iter().flatten().collect();
    let genome_flattened_b: Vec<&usize> = genome_b.iter().flatten().collect();

    assert_eq!(genome_flattened_a.len(), genome_flattened_b.len());
    let genome_length: usize = genome_flattened_a.len();
    
    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

    for index in 0..genome_length {
        let left = (index + genome_length - 1) % genome_length;
        let right = (index + 1) % genome_length;

        let entry_a = adjacency_list.entry(*genome_flattened_a[index]).or_default();
        entry_a.push(*genome_flattened_a[left]);
        entry_a.push(*genome_flattened_a[right]);

        let entry_b = adjacency_list.entry(*genome_flattened_b[index]).or_default();
        entry_b.push(*genome_flattened_b[left]);
        entry_b.push(*genome_flattened_b[right]);

        
    }

    let mut rng = rand::thread_rng();
    let mut child: Vec<usize> = Vec::with_capacity(genome_length);
    let mut current = *genome_flattened_a[rng.gen_range(0..genome_length)];

    for _ in 0..genome_length {
        child.push(current);

        // Remove current from all lists in adjacencyList to shorten the lists
        for (_, list) in adjacency_list.iter_mut() {
            list.retain(|&x| x != current);
        }

        //Examine list for current element:
        // – If there is a common edge, pick that to be next element
        // – Otherwise pick the entry in the list which itself has the shortest list
        // – Ties are split at random

        let mut new_current: usize = usize::MAX;
        let mut seen: HashSet<usize> = HashSet::new();

        // check if there is one edge in the neighbors of the current node which is present twice (sigend with a plus)
        if let Some(neighbors) = adjacency_list.get(&current) {
            for &value in neighbors.iter() {
                if seen.contains(&value) {
                    new_current = value;
                    break;
                }
                seen.insert(value);
            }
            
            // if there is no edge that is present twice, we have to choose the next node based on the length of the neighbors list
            if new_current == usize::MAX {
                // choice of new current is not random if there are two list of equal length.
                let mut min_size = usize::MAX;
                for value in &adjacency_list[&current] {
                    let value_set: HashSet<&usize> = HashSet::from_iter(adjacency_list[&current].iter());
                    if value_set.len() <= min_size {
                        min_size = value_set.len();
                        new_current = *value;
                    }
                }
            }
        }
        adjacency_list.remove(&current);
        if adjacency_list.is_empty() {
            break;
        }
        if new_current == usize::MAX {
            // there is no entry for the current node in the adjacency list, so we have to choose a random node
            new_current = *adjacency_list.keys().nth(rng.gen_range(0..adjacency_list.len())).unwrap();
        }

        

       
        current = new_current;
    }


    
    // assert that the child genome is valid by using a hashset to check for duplicates
    assert_eq!(child.len(), child.iter().collect::<HashSet<&usize>>().len());
    
    (unflattened_genome(&child, genome_a), None)
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

            let child_genomes: (Genome, Option<Genome>) = match crossover_config.name.as_str() {
                "edgeRecombination" =>
                    edge_recombination(
                        &children[individual_index_a].genome,
                        &children[individual_index_b].genome
                    ),
                "orderOneCrossover" =>
                    order_one_crossover(
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

            if let Some(genome) = child_genomes.1 {
                let mut child_b = Individual::new(genome);
                calculate_fitness(&mut child_b, problem_instance);
                children[individual_index_b] = child_b;
            }
        }
    }

    children
}
