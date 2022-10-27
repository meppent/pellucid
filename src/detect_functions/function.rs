use super::candidate::Candidate;
use crate::create_graph::{block::Block, node::Node};
use crate::tools::graph_tools::node_dfs;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum FunctionOutput<'a> {
    MultiBlock, // weird behaviour, we don't like that
    SingleBlock(Block<'a>),
    NoOutput,
}

impl<'a> FunctionOutput<'a> {
    pub fn is_deterministic(&self) -> bool {
        match self {
            Self::MultiBlock => false,
            Self::SingleBlock(_) => true,
            Self::NoOutput => true,
        }
    }
}
#[derive(Debug)]
pub struct Function<'a> {
    candidate: Candidate<'a>,
    input_to_output: HashMap<Block<'a>, FunctionOutput<'a>>,
}
impl<'a> Deref for Function<'a> {
    type Target = Candidate<'a>;
    fn deref(&self) -> &Candidate<'a> {
        &self.candidate
    }
}
impl<'a> Function<'a> {
    fn get_output_blocks(from_block: &Block<'a>, candidate: &Candidate<'a>) -> HashSet<Block<'a>> {
        let mut output_blocks: HashSet<Block<'a>> = HashSet::new();
        for node in from_block.get_nodes() {
            let collisons: HashSet<Node<'a>> = node_dfs(
                &node,
                |n: &Node<'a>| candidate.ends.contains(&n.get_block()),
                &mut |_, _| (),
            )
            .stopped_nodes;
            for collision_node in &collisons {
                output_blocks.extend(collision_node.get_block_children());
            }
        }
        return output_blocks;
    }

    pub fn from(candidate: &Candidate<'a>) -> Self {
        let mut input_to_output: HashMap<Block<'a>, FunctionOutput<'a>> = HashMap::new();
        for parent_block in candidate.start.get_parent_blocks() {
            let output_blocks: HashSet<Block<'a>> =
                Self::get_output_blocks(&parent_block, candidate);
            match output_blocks.len() {
                0 => {
                    input_to_output.insert(parent_block.clone(), FunctionOutput::NoOutput);
                }
                1 => {
                    let output: Block<'a> = output_blocks.iter().collect_vec()[0].clone();
                    input_to_output
                        .insert(parent_block.clone(), FunctionOutput::SingleBlock(output));
                }
                _ => {
                    input_to_output.insert(parent_block.clone(), FunctionOutput::MultiBlock);
                }
            }
        }
        return Self {
            candidate: candidate.clone(),
            input_to_output,
        };
    }

    pub fn get_acceptable_inputs(&self) -> HashSet<Block<'a>> {
        let mut acceptable_inputs: HashSet<Block<'a>> = HashSet::new();
        for (block_input, output_type) in &self.input_to_output {
            if output_type.is_deterministic() {
                acceptable_inputs.insert(block_input.clone());
            }
        }
        return acceptable_inputs;
    }

    pub fn get_output(&self, input_block: &Block<'a>) -> FunctionOutput<'a> {
        return self.input_to_output[input_block].clone();
    }
}
