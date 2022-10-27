use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;

use crate::{
    create_graph::block::Block,
    detect_cycles::acyclic_graph::AcyclicGraph,
    detect_functions::{
        function::{Function, FunctionOutput},
        function_detection::detect_functions_and_duplicate_oddities,
    },
};

use super::skeleton_scopes::{SkeletonFunction, SkeletonIf, SkeletonJunction, SkeletonScope};

#[derive(PartialEq, Eq, Debug)]
enum Destination<'a> {
    Block(Block<'a>),
    StartLoop {
        entry_block: Block<'a>,
        label: usize,
    },
    ContinueLoop {
        label: usize,
    },
}

#[derive(PartialEq, Eq, Debug)]
enum BlockOutput<'a> {
    // where to go after the end of a given block ?
    SingleDestination(Destination<'a>),
    DualDestination {
        true_dest: Destination<'a>,
        false_dest: Destination<'a>,
    },
    NonDeterministic,
    Over,
}

pub struct Skeleton<'a, 'b> {
    pub a_graph: &'b mut AcyclicGraph<'a, 'b>,
    pub functions: HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>>,
    pub junctions: HashMap<Block<'a>, Rc<RefCell<SkeletonJunction<'a>>>>,
    pub main_instructions: Vec<SkeletonScope<'a>>,
    pub returning_blocks: HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>>, // blocks that end anfunction
}

impl<'a, 'b> Skeleton<'a, 'b> {
    pub fn build(a_graph: &'b mut AcyclicGraph<'a, 'b>) -> Self {
        let mut functions: HashMap<Block, Function> =
            detect_functions_and_duplicate_oddities(a_graph);

        let mut skeleton_functions: HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>> =
            HashMap::new();
        let mut skeleton_junctions: HashMap<Block<'a>, Rc<RefCell<SkeletonJunction<'a>>>> =
            HashMap::new();

        for block in a_graph.get_all_blocks() {
            if let Some(function) = functions.remove(&block) {
                skeleton_functions.insert(
                    block.clone(),
                    Rc::new(RefCell::new(SkeletonFunction {
                        info: function,
                        instructions: Vec::new(),
                    })),
                );
                assert!(a_graph.loops.get_label_of_entry(&block).is_none());
            } else if block.get_parent_blocks().len() >= 2 {
                skeleton_junctions.insert(
                    block.clone(),
                    Rc::new(RefCell::new(SkeletonJunction {
                        starting_block: block,
                        instructions: Vec::new(),
                    })),
                );
            }
        }
        drop(functions);

        let mut returning_blocks: HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>> =
            HashMap::new();
        for skeleton_function in skeleton_functions.values() {
            let ending_blocks: HashSet<Block> =
                RefCell::borrow(skeleton_function).info.ends.clone();
            for ending_block in ending_blocks {
                returning_blocks.insert(ending_block, Rc::clone(skeleton_function));
            }
        }

        let mut skeleton: Skeleton = Skeleton {
            a_graph,
            functions: skeleton_functions,
            junctions: skeleton_junctions,
            main_instructions: Vec::new(),
            returning_blocks,
        };

        for (block, skeleton_function) in skeleton.functions.clone() {
            let stop_at: HashSet<Block<'a>> = RefCell::borrow(&skeleton_function).info.ends.clone();
            let instructions = skeleton.get_instructions_from(block.clone(), stop_at);
            RefCell::borrow_mut(&skeleton_function).instructions = instructions;
        }

        for (block, skeleton_junction) in skeleton.junctions.clone() {
            let instructions = skeleton.get_instructions_from(block.clone(), HashSet::new());
            RefCell::borrow_mut(&skeleton_junction).instructions = instructions;
        }

        // TODO handle case where first block in graph has multiple parents
        skeleton.main_instructions =
            skeleton.get_instructions_from(skeleton.a_graph.get_block(0), HashSet::new());

        return skeleton;
    }

    fn get_instructions_on_dest(
        &mut self,
        stop_at: HashSet<Block<'a>>,
        current_block: &Block<'a>,
        dest: Destination<'a>,
    ) -> Vec<SkeletonScope<'a>> {
        let mut following_instructions: Vec<SkeletonScope<'a>>;

        match dest {
            Destination::ContinueLoop { label } => {
                // continue existing loop
                following_instructions = vec![SkeletonScope::LoopContinue { label }];
            }

            Destination::Block(next_block) => {
                if next_block.get_parent_blocks().len() >= 2 {
                    following_instructions = self.continue_instructions_at_multi_parents_block(
                        stop_at.clone(),
                        current_block,
                        &next_block,
                    );
                } else {
                    following_instructions = self.get_instructions_from(next_block, stop_at);
                }
            }

            Destination::StartLoop { entry_block, label } => {
                following_instructions = vec![SkeletonScope::Loop { label }];
                following_instructions
                    .extend(self.get_instructions_from(entry_block.clone(), stop_at));
            }
        }
        return following_instructions;
    }

    fn get_instructions_from(
        &mut self,
        from_block: Block<'a>,
        stop_at: HashSet<Block<'a>>,
    ) -> Vec<SkeletonScope<'a>> {
        // TODO check if 2functions can share some ending blocks

        let mut instructions: Vec<SkeletonScope<'a>> = Vec::new();

        let mut current_block: Block<'a> = from_block.clone();
        instructions.push(SkeletonScope::Block(current_block.clone()));

        let mut block_output: BlockOutput;

        loop {
            block_output = self.get_block_output(&current_block);
            if let BlockOutput::SingleDestination(Destination::Block(next_block)) = &block_output {
                if next_block.get_parent_blocks().len() == 1 {
                    current_block = next_block.clone();
                    instructions.push(SkeletonScope::Block(current_block.clone()));
                    continue;
                }
            }
            break;
        }

        match block_output {
            BlockOutput::Over => (), // nothing to do

            BlockOutput::SingleDestination(dest) => {
                let following_instructions: Vec<SkeletonScope<'a>> =
                    self.get_instructions_on_dest(stop_at, &current_block, dest);
                instructions.extend(following_instructions);
            }

            BlockOutput::DualDestination {
                true_dest,
                false_dest,
            } => {
                let true_instructions: Vec<SkeletonScope<'a>> =
                    self.get_instructions_on_dest(stop_at.clone(), &current_block, true_dest);
                let false_instructions: Vec<SkeletonScope<'a>> =
                    self.get_instructions_on_dest(stop_at.clone(), &current_block, false_dest);
                let skeleton_if = SkeletonIf {
                    true_instructions,
                    false_instructions,
                };
                instructions.push(SkeletonScope::If(skeleton_if));
            }
            BlockOutput::NonDeterministic => {
                if stop_at.contains(&current_block) {
                    // we just reach an ending block of anfunction, we do nothing
                } else {
                    instructions.push(SkeletonScope::Panic);
                }
            }
        }

        return instructions;
    }

    fn continue_instructions_at_multi_parents_block(
        &mut self,
        stop_at: HashSet<Block<'a>>,
        input_block: &Block<'a>,
        multi_parent_block: &Block<'a>,
    ) -> Vec<SkeletonScope<'a>> {
        let mut following_instructions: Vec<SkeletonScope<'a>> = Vec::new();

        if let Some(skeleton_functions) = self.functions.get(&multi_parent_block) {
            // beginning of anfunction
            following_instructions.push(SkeletonScope::Function(
                self.functions[&multi_parent_block].clone(),
            ));
            let function_output: FunctionOutput = RefCell::borrow(skeleton_functions)
                .info
                .get_output(input_block);
            match function_output {
                FunctionOutput::MultiBlock => {
                    panic!("A dupplication should have occured here.")
                }
                FunctionOutput::SingleBlock(output_block) => {
                    following_instructions.extend(self.get_instructions_from(output_block, stop_at))
                }
                FunctionOutput::NoOutput => (),
            }
        } else {
            // beginning of a junction
            following_instructions.push(SkeletonScope::Junction(
                self.junctions[&multi_parent_block].clone(),
            ));
        }
        return following_instructions;
    }

    fn get_block_output(&mut self, block: &Block<'a>) -> BlockOutput<'a> {
        let continue_loop_label: Option<usize> = self
            .a_graph
            .disconnected_connections
            .get(&block.get_pc_start())
            .cloned();
        let loop_start_label: Option<usize> = self.a_graph.loops.get_label_of_entry(block);
        assert!(!(loop_start_label.is_some() && continue_loop_label.is_some()));

        let try_to_convert_to_loop_entry = |block: &Block<'a>| -> Destination {
            if let Some(loop_label) = self.a_graph.loops.get_label_of_entry(block) {
                return Destination::StartLoop {
                    entry_block: block.clone(),
                    label: loop_label,
                };
            } else {
                return Destination::Block(block.clone());
            }
        };

        let block_output: BlockOutput;

        if block.get_child_blocks().len() == 0 {
            if let Some(loop_label) = continue_loop_label {
                block_output =
                    BlockOutput::SingleDestination(Destination::ContinueLoop { label: loop_label });
            } else {
                block_output = BlockOutput::Over;
            }
        } else if block.get_child_blocks().len() == 1 {
            let next_block: Block = block.get_child_blocks().iter().collect_vec()[0].clone();

            if let Some(loop_label) = continue_loop_label {
                block_output = if next_block.get_pc_start() == block.get_next_pc_start() {
                    BlockOutput::DualDestination {
                        true_dest: Destination::ContinueLoop { label: loop_label },
                        false_dest: Destination::Block(next_block),
                    }
                } else {
                    BlockOutput::DualDestination {
                        true_dest: Destination::Block(next_block),
                        false_dest: Destination::ContinueLoop { label: loop_label },
                    }
                };
            } else {
                block_output =
                    BlockOutput::SingleDestination(try_to_convert_to_loop_entry(&next_block));
            }
        } else if let Some((true_block, false_block)) = block.get_next_conditional_dests() {
            block_output = BlockOutput::DualDestination {
                true_dest: try_to_convert_to_loop_entry(&true_block),
                false_dest: try_to_convert_to_loop_entry(&false_block),
            };
            assert!(!self
                .a_graph
                .disconnected_connections
                .contains_key(&block.get_pc_start()));
        } else {
            block_output = BlockOutput::NonDeterministic;
        }
        return block_output;
    }
}
