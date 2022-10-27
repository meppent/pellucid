use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::scopes_with_opcodes::{
    fill_n_inputs_and_outputs, FunctionCallWithOpcodes, FunctionReturnWithOpcodes,
    FunctionWithOpcodes, InstructionsWithOpcodes, OpcodeScope,
};
use crate::{
    bytecode_reader::vopcode::Vopcode,
    create_graph::block::Block,
    execution_flow::{
        execution_flow::{ExecutionFlow, FunctionLabel, Scope, MAIN_FUNCTION_LABEL},
        skeleton::{
            skeleton::Skeleton,
            skeleton_scopes::{SkeletonFunction, SkeletonJunction, SkeletonScope},
        },
    },
    tools::utils::{calculate_hash, concat_and_consume},
};

pub type ExecutionFlowWithOpcodes = ExecutionFlow<
    InstructionsWithOpcodes,
    FunctionCallWithOpcodes,
    FunctionReturnWithOpcodes,
    FunctionWithOpcodes,
>;

fn compute_function_label(starting_block: &Block) -> u64 {
    return calculate_hash(starting_block);
}

pub fn convert_skeleton_to_execution_flow<'a>(
    skeleton: &Skeleton<'a, '_>,
) -> ExecutionFlowWithOpcodes {
    let mut functions: HashMap<FunctionLabel, FunctionWithOpcodes> = HashMap::new();

    functions.insert(
        MAIN_FUNCTION_LABEL,
        FunctionWithOpcodes {
            label: MAIN_FUNCTION_LABEL,
            n_inputs: 0,
            n_outputs: None,
            content: convert_skeleton_scopes(
                &skeleton.main_instructions,
                &skeleton.returning_blocks,
            ),
        },
    );

    for (starting_block, skeleton_function) in &skeleton.functions {
        let label: FunctionLabel = compute_function_label(starting_block);
        assert!(!functions.contains_key(&label));
        functions.insert(
            label,
            convert_skeleton_function(skeleton_function.clone(), &skeleton.returning_blocks),
        );
    }

    for (starting_block, skeleton_junction) in &skeleton.junctions {
        let label: FunctionLabel = compute_function_label(starting_block);
        assert!(!functions.contains_key(&label));
        functions.insert(
            label,
            convert_skeleton_junction(skeleton_junction.clone(), &skeleton.returning_blocks),
        );
    }

    fill_n_inputs_and_outputs(&mut functions);

    let mut execution_flow_with_opcodes: ExecutionFlowWithOpcodes =
        ExecutionFlowWithOpcodes::new(functions);
    execution_flow_with_opcodes.remove_secondary_functions_containing_loops();
    return execution_flow_with_opcodes;
}

fn convert_skeleton_function<'a>(
    skeleton_function: Rc<RefCell<SkeletonFunction<'a>>>,
    returning_blocks: &HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>>,
) -> FunctionWithOpcodes {
    let inner_scopes: Vec<OpcodeScope> = convert_skeleton_scopes(
        &RefCell::borrow(&skeleton_function).instructions,
        returning_blocks,
    );
    let mut converted_function: FunctionWithOpcodes =
        FunctionWithOpcodes::new_empty(&RefCell::borrow(&skeleton_function).info.start);
    converted_function.content = inner_scopes;
    return converted_function;
}

fn convert_skeleton_junction<'a>(
    skeleton_junction: Rc<RefCell<SkeletonJunction<'a>>>,
    returning_blocks: &HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>>,
) -> FunctionWithOpcodes {
    let inner_scopes: Vec<OpcodeScope> = convert_skeleton_scopes(
        &RefCell::borrow(&skeleton_junction).instructions,
        returning_blocks,
    );
    let mut converted_function: FunctionWithOpcodes =
        FunctionWithOpcodes::new_empty(&RefCell::borrow(&skeleton_junction).starting_block);
    converted_function.content = inner_scopes;
    converted_function.n_outputs = None;
    return converted_function;
}

fn convert_skeleton_scopes<'a>(
    skeleton_instructions: &[SkeletonScope<'a>],
    returning_blocks: &HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>>,
) -> Vec<OpcodeScope> {
    if skeleton_instructions.is_empty() {
        return Vec::new();
    }

    let (converted_scopes, next_skeleton_index): (Vec<OpcodeScope>, usize) =
        match &skeleton_instructions[0] {
            SkeletonScope::LoopContinue { label } => {
                (vec![Scope::LoopContinue { label: *label }], 1)
            }
            SkeletonScope::Loop { label } => (vec![Scope::Loop { label: *label }], 1),
            SkeletonScope::Panic => {
                assert!(skeleton_instructions.len() == 1);
                (vec![Scope::Panic], 1)
            }
            SkeletonScope::Block(_) => {
                let mut consecutive_blocks: Vec<Block> = Vec::new();
                let mut first_index_without_block: usize = skeleton_instructions.len();
                for (scope_index, scope) in skeleton_instructions.iter().enumerate() {
                    if let SkeletonScope::Block(block) = scope {
                        consecutive_blocks.push(block.clone());
                    } else {
                        first_index_without_block = scope_index;
                        break;
                    }
                }

                let next_skeleton_scope: Option<&SkeletonScope> =
                    if first_index_without_block < skeleton_instructions.len() {
                        Some(&skeleton_instructions[first_index_without_block])
                    } else {
                        None
                    };

                (
                    consecutive_blocks_to_scopes(
                        &consecutive_blocks,
                        returning_blocks,
                        next_skeleton_scope,
                    ),
                    first_index_without_block,
                )
            }

            SkeletonScope::Function(skeleton_function) => (
                vec![Scope::FunctionCall(FunctionCallWithOpcodes {
                    label: compute_function_label(&RefCell::borrow(skeleton_function).info.start),
                })],
                1,
            ),
            SkeletonScope::Junction(skeleton_junction) => (
                vec![Scope::FunctionCall(FunctionCallWithOpcodes {
                    label: compute_function_label(
                        &RefCell::borrow(skeleton_junction).starting_block,
                    ),
                })],
                1,
            ),
            SkeletonScope::If(skeleton_if) => {
                let mut instructions_if_true: Vec<OpcodeScope> =
                    convert_skeleton_scopes(&skeleton_if.true_instructions, returning_blocks);
                let mut instructions_if_false: Vec<OpcodeScope> =
                    convert_skeleton_scopes(&skeleton_if.false_instructions, returning_blocks);
                /*
                    if:                                 if:
                            ----                                ----
                            ----                                ----
                            f()          ┌───┘╲         else:
                    else:                └───┐╱                 ----
                            ----                                ----
                            ----                        f()
                            f()
                */
                let mut then_scopes: Vec<OpcodeScope> = Vec::new();
                while let (Some(last_true_scope), Some(last_false_scope)) =
                    (instructions_if_true.last(), instructions_if_false.last())
                {
                    if last_true_scope == last_false_scope {
                        instructions_if_true.pop();
                        then_scopes.push(instructions_if_false.pop().unwrap());
                    } else {
                        break;
                    }
                }
                let if_scope = Scope::Condition {
                    instructions_if_true,
                    instructions_if_false,
                };
                (concat_and_consume(vec![if_scope], then_scopes), 1)
            }
        };

    let remaining_skeleton_instructions: &[SkeletonScope] =
        &skeleton_instructions[next_skeleton_index..];
    let remaining_scopes: Vec<OpcodeScope> =
        convert_skeleton_scopes(remaining_skeleton_instructions, returning_blocks);
    return concat_and_consume(converted_scopes, remaining_scopes);
}

fn consecutive_blocks_to_scopes<'a>(
    consecutive_blocks: &[Block<'a>],
    returning_blocks: &HashMap<Block<'a>, Rc<RefCell<SkeletonFunction<'a>>>>,
    next_skeleton_scope: Option<&SkeletonScope>,
) -> Vec<OpcodeScope> {
    // 'consecutive' doesnt mean that these block have adjacent pc starts, but rather that they are executed consecutively (with jumps from the previous to the next one)
    let mut resulting_scopes: Vec<OpcodeScope> = vec![Scope::Instructions(aggregate_blocks_code(
        consecutive_blocks,
    ))];

    for block in &consecutive_blocks[0..consecutive_blocks.len() - 1] {
        assert!(!returning_blocks.contains_key(block));
    }

    let last_block: &Block = consecutive_blocks.last().unwrap();
    if let Some(skeleton_function) = returning_blocks.get(last_block) {
        let execution_terminates: bool = last_block.get_code().last().unwrap().is_exiting(); // the last ocpode ends the execution of the contract
        if !execution_terminates {
            // REVERT, STOP ... are not considered as return points of the current function, because we do not go back to the previous scope
            if let Some(_next_skeleton_scope) = next_skeleton_scope {
                match _next_skeleton_scope {
                    SkeletonScope::LoopContinue { label: _ } => (), // The last block had been considered to be one end on the currentfunction because its next step is to continue a loop
                    SkeletonScope::Panic => (), // the problem comes from the existence of Panic ...
                    _ => panic!(
                        "Non trivial skeleton scope after the end of anfunction: {}",
                        _next_skeleton_scope.to_string()
                    ),
                }
            } else {
                let label: FunctionLabel =
                    compute_function_label(&RefCell::borrow(&skeleton_function).info.start);
                resulting_scopes.push(Scope::FunctionReturn(FunctionReturnWithOpcodes { label }));
            }
        }
    }
    return resulting_scopes;
}

pub fn aggregate_n_stack_inputs(
    n_stack_inputs_0: usize,
    n_stack_outputs_0: usize,
    n_stack_inputs_1: usize,
) -> usize {
    return n_stack_inputs_0
        .max((n_stack_inputs_0 + n_stack_inputs_1).saturating_sub(n_stack_outputs_0));
}

pub fn aggregate_n_stack_outputs(
    n_stack_inputs_0: usize,
    n_stack_outputs_0: usize,
    n_stack_inputs_1: usize,
    n_stack_outputs_1: usize,
) -> usize {
    let first_stack_delta: isize = n_stack_outputs_0 as isize - n_stack_inputs_0 as isize;
    let second_stack_delta: isize = n_stack_outputs_1 as isize - n_stack_inputs_1 as isize;
    let aggregated_stack_delta = first_stack_delta + second_stack_delta;
    let new_n_stack_inputs: usize =
        aggregate_n_stack_inputs(n_stack_inputs_0, n_stack_outputs_0, n_stack_inputs_1);
    let new_n_stack_outputs = (aggregated_stack_delta + new_n_stack_inputs as isize) as usize;
    return new_n_stack_outputs;
}

pub fn aggregate_n_stack_inputs_and_outputs(
    n_stack_inputs_0: usize,
    n_stack_outputs_0: usize,
    n_stack_inputs_1: usize,
    n_stack_outputs_1: usize,
) -> (usize, usize) {
    return (
        aggregate_n_stack_inputs(n_stack_inputs_0, n_stack_outputs_0, n_stack_inputs_1),
        aggregate_n_stack_outputs(
            n_stack_inputs_0,
            n_stack_outputs_0,
            n_stack_inputs_1,
            n_stack_outputs_1,
        ),
    );
}

fn aggregate_blocks_code(consecutive_blocks: &[Block]) -> InstructionsWithOpcodes {
    let mut code: Vec<Vopcode> = Vec::new();
    let mut n_stack_inputs: usize = 0;
    let mut n_stack_outputs: usize = 0;
    for block in consecutive_blocks {
        code.extend(block.get_code());
        let block_n_stack_inputs: usize = block.clone_symbolic_block().n_args;
        let block_n_stack_outputs: usize = block.clone_symbolic_block().n_outputs();
        (n_stack_inputs, n_stack_outputs) = aggregate_n_stack_inputs_and_outputs(
            n_stack_inputs,
            n_stack_outputs,
            block_n_stack_inputs,
            block_n_stack_outputs,
        );
    }
    return InstructionsWithOpcodes {
        code,
        n_stack_inputs,
        n_stack_outputs,
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        bytecode_reader::bytecode::Bytecode,
        create_graph::graph::Graph,
        detect_cycles::acyclic_graph::AcyclicGraph,
        execution_flow::{
            skeleton::skeleton::Skeleton,
            with_opcodes::flow_with_opcodes::{
                convert_skeleton_to_execution_flow, ExecutionFlowWithOpcodes,
            },
        },
    };
    use std::fs;

    use super::aggregate_n_stack_inputs_and_outputs;

    #[test]
    pub fn test_convert_skeleton_to_execution_flow() {
        let path: String = String::from("./contracts/complex/bytecode.txt");
        // let path: String =
        //     String::from("./contracts/simple/contract_0/bytecode.txt");
        println!("Loading {}", path);
        let bytecode_string: String = fs::read_to_string(path).expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let mut graph: Graph = Graph::from(&bytecode);
        let mut a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);

        let skeleton: Skeleton = Skeleton::build(&mut a_graph);
        let _: ExecutionFlowWithOpcodes = convert_skeleton_to_execution_flow(&skeleton);
    }

    #[test]
    pub fn test_aggregate_n_stack_inputs_and_outputs() {
        for (
            n_stack_inputs_0,
            n_stack_outputs_0,
            n_stack_inputs_1,
            n_stack_outputs_1,
            new_n_stack_inputs,
            new_n_stack_outputs,
        ) in [
            (10, 8, 7, 4, 10, 5),
            (10, 15, 6, 7, 10, 16),
            (10, 5, 15, 1, 20, 1),
            (10, 5, 15, 12, 20, 12),
            (10, 5, 15, 100, 20, 100),
        ] {
            assert!(
                (new_n_stack_inputs, new_n_stack_outputs)
                    == aggregate_n_stack_inputs_and_outputs(
                        n_stack_inputs_0,
                        n_stack_outputs_0,
                        n_stack_inputs_1,
                        n_stack_outputs_1,
                    )
            );
        }
    }
}
