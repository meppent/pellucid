use super::divergences::{compute_divergences, Divergences};
use crate::create_graph::node::Node;
use crate::detect_cycles::acyclic_graph::AcyclicGraph;
use crate::tools::utils::is_empty_iter;
use crate::{create_graph::block::Block, detect_functions::candidate::Candidate};
use itertools::Itertools;
use std::collections::HashSet;

pub fn detect_candidates<'a, 'b>(a_graph: &AcyclicGraph<'a, 'b>) -> HashSet<Candidate<'a>> {
    let mut candidates: HashSet<Candidate<'a>> = HashSet::new();
    let divergences: Divergences = compute_divergences(&a_graph.get_all_blocks());
    for block in a_graph.get_all_blocks() {
        for (node_1, node_0) in block.get_nodes().iter().tuple_combinations() {
            if have_different_origin(node_0, node_1) {
                let divergences_blocks: HashSet<Block> =
                    divergences.get_divergence_blocks(node_0, node_1).clone();
                if divergences_blocks.len() > 0 {
                    candidates.insert(Candidate {
                        start: block.clone(),
                        ends: divergences_blocks,
                    });
                }
            }
        }
    }
    return remove_candidate_starting_on_loops(a_graph, &candidates);
}

fn have_different_origin<'a>(node_0: &Node<'a>, node_1: &Node<'a>) -> bool {
    assert!(node_0.get_block() == node_1.get_block());
    return !is_empty_iter(
        node_0
            .get_block_parents()
            .symmetric_difference(&node_1.get_block_parents()),
    );
}

fn remove_candidate_starting_on_loops<'a, 'b>(
    a_graph: &AcyclicGraph<'a, 'b>,
    candidates: &HashSet<Candidate<'a>>,
) -> HashSet<Candidate<'a>> {
    // the current algo detects loops as candidates, we need to remove these candidates
    let mut cleaned_candidates: HashSet<Candidate<'a>> = HashSet::new();
    for candidate in candidates.clone() {
        if a_graph.loops.has_loop_starting_at(&candidate.start) {
            assert!(candidate_is_properly_included_in_loop(a_graph, &candidate));
        } else {
            cleaned_candidates.insert(candidate);
        }
    }
    return cleaned_candidates;
}

fn candidate_is_properly_included_in_loop<'a, 'b>(
    a_graph: &AcyclicGraph<'a, 'b>,
    candidates: &Candidate<'a>,
) -> bool {
    let loop_label: usize = a_graph.loops.get_label_of_entry(&candidates.start).unwrap();
    for ending_block in &candidates.ends {
        if ending_block.has_some_children()
            && ending_block
                .get_child_blocks()
                .iter()
                .all(|child: &Block| a_graph.loops.block_is_in_loop(&child, loop_label))
        {
            return false;
        }
    }
    return true;
}
