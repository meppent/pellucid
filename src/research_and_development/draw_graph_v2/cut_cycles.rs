use crate::tools::utils::remove_value;
use std::collections::HashSet;

pub fn remove_cycles(adjacency_list: &mut Vec<Vec<usize>>) {
    // not optimal at all, but it's nbd because this code is only intended to R&D
    for index in 0..adjacency_list.len() {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut current_parents: HashSet<usize> = HashSet::new();
        explore_to_cut_cycles(adjacency_list, &mut visited, &mut current_parents, index);
    }
}

pub fn explore_to_cut_cycles(
    adjacency_list: &mut Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    current_parents: &mut HashSet<usize>,
    index: usize,
) {
    visited.insert(index);
    current_parents.insert(index);
    for child in adjacency_list[index].clone() {
        if visited.contains(&child) {
            if current_parents.contains(&child) {
                remove_value(adjacency_list.get_mut(index).unwrap(), &child);
            }
        } else {
            explore_to_cut_cycles(adjacency_list, visited, current_parents, child);
        }
    }
    current_parents.remove(&index);
}
