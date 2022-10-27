use crate::{create_graph::block::Block, tools::utils::hash_hashset};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Clone)]
pub struct Candidate<'a> {
    pub start: Block<'a>,
    pub ends: HashSet<Block<'a>>,
}

impl<'a> Hash for Candidate<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        hash_hashset(&self.ends).hash(state);
    }
}

impl<'a> Debug for Candidate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("function candidate")
            .field("start", &format!("{:x}", self.start.get_pc_start()))
            .field(
                "end",
                &self
                    .ends
                    .iter()
                    .map(|end| format!("{:x}", end.get_pc_start()))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl<'a> Candidate<'a> {
    pub fn get_intermediate_blocks(&self) -> HashSet<Block<'a>> {
        let mut visited: HashSet<Block<'a>> = HashSet::new();
        let mut intermediate_blocks: HashSet<Block<'a>> = self.ends.clone();
        let mut parent_of: HashMap<Block<'a>, Block<'a>> = HashMap::new();

        self._explore_dfs(
            &self.start,
            &mut visited,
            &mut intermediate_blocks,
            &mut parent_of,
        );

        return intermediate_blocks;
    }

    fn _explore_dfs(
        &self,
        current: &Block<'a>,
        visited: &mut HashSet<Block<'a>>,
        intermediate_blocks: &mut HashSet<Block<'a>>,
        parent_of: &mut HashMap<Block<'a>, Block<'a>>,
    ) {
        if intermediate_blocks.contains(current) {
            // self.ends are included in intermediary_blocks
            let mut moving_up: &Block = current;
            while let Some(parent) = parent_of.get(&moving_up) {
                moving_up = parent;
                if intermediate_blocks.contains(moving_up) {
                    break;
                } else {
                    intermediate_blocks.insert(moving_up.clone());
                }
            }
        }
        if visited.contains(&current) {
            return;
        }
        visited.insert(current.clone());

        if !intermediate_blocks.contains(current) {
            for child in current.get_child_blocks() {
                parent_of.insert(child.clone(), current.clone());
                self._explore_dfs(&child, visited, intermediate_blocks, parent_of);
            }
        }
    }
}
