use itertools::Itertools;

use super::{candidate::Candidate, candidate_detection::detect_candidates, function::Function};
use crate::{
    create_graph::{block::Block, duplication::duplicate_block_connection},
    detect_cycles::acyclic_graph::AcyclicGraph,
    tools::graph_tools::pack_by_block,
};
use std::collections::{HashMap, HashSet};

fn pick_best_candidate<'a>(candidates: &HashSet<Candidate<'a>>) -> Option<Function<'a>> {
    for (candidate_0, candidate_1) in candidates.iter().tuple_combinations() {
        assert!(candidate_0.start == candidate_1.start);
    }
    let mut best_functions: Vec<Function<'a>> = Vec::new();
    let mut max_acceptable_inputs: usize = 0;
    for candidate in candidates {
        let function: Function<'a> = Function::from(candidate);
        let n_acceptable_inputs: usize = function.get_acceptable_inputs().len();

        if n_acceptable_inputs >= 2 && n_acceptable_inputs == max_acceptable_inputs {
            // TODO investigate here: the >= 2 is probably at the origin of the panic scopes
            best_functions.push(function);
        } else if n_acceptable_inputs > max_acceptable_inputs {
            max_acceptable_inputs = n_acceptable_inputs;
            best_functions = vec![function];
        }
    }
    let mut best_function: Option<Function<'a>> = None;
    let mut max_n_intermediate_blocks: usize = 0;
    for function in best_functions {
        let intermediate_blocks: HashSet<Block> = function.get_intermediate_blocks();
        if intermediate_blocks.len() > max_n_intermediate_blocks {
            max_n_intermediate_blocks = intermediate_blocks.len();
            best_function = Some(function);
        }
    }
    return best_function;
}

fn detect_functions<'a, 'b>(a_graph: &AcyclicGraph<'a, 'b>) -> HashMap<Block<'a>, Function<'a>> {
    let mut all_functions: HashMap<Block<'a>, Function<'a>> = HashMap::new(); // initial block =>function

    let candidates: HashSet<Candidate<'a>> = detect_candidates(&a_graph);
    let block_to_candidates: HashMap<Block, HashSet<Candidate>> =
        pack_by_block(&a_graph.get_all_blocks(), candidates, |s| s.start.clone());
    for (starting_block, candidates) in &block_to_candidates {
        let best_function = pick_best_candidate(&candidates);
        if let Some(resulting_function) = best_function {
            all_functions.insert(starting_block.clone(), resulting_function);
        }
    }

    return all_functions;
}

pub fn detect_functions_and_duplicate_oddities<'a, 'b>(
    a_graph: &mut AcyclicGraph<'a, 'b>,
) -> HashMap<Block<'a>, Function<'a>> {
    let initial_n_blocks: usize = a_graph.get_all_blocks().len();
    let functions: HashMap<Block, Function> = 'main_loop: loop {
        log::debug!("functions detection...");
        let functions: HashMap<Block<'a>, Function<'a>> = detect_functions(&a_graph);
        for starting_block in a_graph.get_all_blocks() {
            if let Some(function) = functions.get(&starting_block) {
                let mut dupplication_occured: bool = false;
                for input_block in function.start.get_parent_blocks() {
                    if !input_block.get_child_blocks().contains(&function.start) {
                        // when we duplicate, the parents of the starting block may change if it loops on itself
                        continue;
                    }
                    if !function.get_output(&input_block).is_deterministic() {
                        duplicate_block_connection(a_graph, &input_block, &function.start);
                        dupplication_occured = true;
                    }
                }
                if dupplication_occured {
                    log::debug!("dupplication_occured on {:?}", function.start);
                    continue 'main_loop; // TODO avoid recomputing all functions
                }
            }
        }

        break functions;
    };
    let final_n_blocks: usize = a_graph.get_all_blocks().len();

    log::debug!(
        "{} % blocks were duplicated",
        100.0 * (final_n_blocks - initial_n_blocks) as f32 / initial_n_blocks as f32
    );

    return functions;
}
