use super::flow_with_opcodes::{aggregate_n_stack_inputs, aggregate_n_stack_inputs_and_outputs};
use crate::execution_flow::execution_flow::{
    AccessContent, FunctionLabel, GetFunctionLabel, Length, Scope,
};
use crate::{
    bytecode_reader::vopcode::Vopcode, create_graph::block::Block, tools::utils::calculate_hash,
};
use std::collections::HashMap;
use std::{collections::HashSet, fmt::Debug};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FunctionCallWithOpcodes {
    pub label: FunctionLabel,
}

impl GetFunctionLabel for FunctionCallWithOpcodes {
    fn get_label(&self) -> FunctionLabel {
        return self.label;
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FunctionReturnWithOpcodes {
    pub label: FunctionLabel,
}

impl GetFunctionLabel for FunctionReturnWithOpcodes {
    fn get_label(&self) -> FunctionLabel {
        return self.label;
    }
}

pub type OpcodeScope =
    Scope<InstructionsWithOpcodes, FunctionCallWithOpcodes, FunctionReturnWithOpcodes>;

#[derive(Clone, Eq, Debug)]
pub struct FunctionWithOpcodes {
    // Starting from this point in the program, 'functions' may no longer return.
    pub label: FunctionLabel,
    pub n_inputs: usize,
    pub n_outputs: Option<usize>, // 'None' in case it's a junction (cf JunctionSkeletonScope) = anfunction from which we never get out. Here we only consider the output offunctions, that is to say we do not look at the RETURN opcode (but JUMP / JUMPI ...)
    pub content: Vec<OpcodeScope>,
}

impl PartialEq for FunctionWithOpcodes {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl GetFunctionLabel for FunctionWithOpcodes {
    fn get_label(&self) -> FunctionLabel {
        return self.label;
    }
}

impl AccessContent<InstructionsWithOpcodes, FunctionCallWithOpcodes, FunctionReturnWithOpcodes>
    for FunctionWithOpcodes
{
    fn get_content(&self) -> &Vec<OpcodeScope> {
        return &self.content;
    }

    fn get_content_mut(&mut self) -> &mut Vec<OpcodeScope> {
        return &mut self.content;
    }
}

impl FunctionWithOpcodes {
    pub fn new_empty(starting_block: &Block) -> Self {
        return FunctionWithOpcodes {
            label: calculate_hash(starting_block),
            n_inputs: 0,
            n_outputs: None,
            content: Vec::new(),
        };
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct InstructionsWithOpcodes {
    pub code: Vec<Vopcode>,
    pub n_stack_inputs: usize,
    pub n_stack_outputs: usize,
}

impl Length for InstructionsWithOpcodes {
    fn len(&self) -> usize {
        return self.code.len();
    }
}

impl InstructionsWithOpcodes {
    pub fn get_last_vopcode(&self) -> Vopcode {
        return self.code[self.len() - 1];
    }
    pub fn len(&self) -> usize {
        return self.code.len();
    }
}

pub fn fill_n_inputs_and_outputs(
    function_scopes: &mut HashMap<FunctionLabel, FunctionWithOpcodes>,
) {
    let mut function_alias_already_filled: HashSet<u64> = HashSet::new();
    let all_labels: Vec<FunctionLabel> = function_scopes.keys().cloned().collect();
    for label in all_labels {
        if !function_alias_already_filled.contains(&label) {
            _fill_n_inputs_and_outputs(label, &mut function_alias_already_filled, function_scopes);
        }
    }
}

fn _fill_n_inputs_and_outputs(
    label: FunctionLabel,
    function_alias_already_filled: &mut HashSet<u64>,
    function_scopes: &mut HashMap<FunctionLabel, FunctionWithOpcodes>,
) {
    let content: Vec<OpcodeScope> = function_scopes[&label].content.clone();
    let (n_inputs, n_outputs) = get_n_inputs_and_outputs_until_end(
        &content,
        function_alias_already_filled,
        function_scopes,
    );
    function_alias_already_filled.insert(label);
    function_scopes.get_mut(&label).unwrap().n_inputs = n_inputs;
    function_scopes.get_mut(&label).unwrap().n_outputs = n_outputs;
}

fn get_n_inputs_and_outputs_until_end(
    instructions: &[OpcodeScope],
    function_alias_already_filled: &mut HashSet<u64>,
    function_scopes: &mut HashMap<FunctionLabel, FunctionWithOpcodes>,
) -> (usize, Option<usize>) // n_inputs, possible n_outputs
{
    if instructions.is_empty() {
        return (0, Some(0));
    }

    let n_inputs_at_first_scope: usize;
    let n_outputs_at_first_scope: Option<usize>;

    match &instructions[0] {
        Scope::Instructions(code_scope) => {
            n_inputs_at_first_scope = code_scope.n_stack_inputs;
            if code_scope.get_last_vopcode().is_exiting() {
                // do not consider the size of the ending stack on opcodes RETURN, REVERT, STOP ... For now, we are only interested inflow
                n_outputs_at_first_scope = None;
            } else {
                n_outputs_at_first_scope = Some(code_scope.n_stack_outputs);
            }
        }
        Scope::FunctionCall(call) => {
            if !function_alias_already_filled.contains(&call.label) {
                _fill_n_inputs_and_outputs(
                    call.label,
                    function_alias_already_filled,
                    function_scopes,
                );
            }
            n_inputs_at_first_scope = function_scopes[&call.label].n_inputs;
            n_outputs_at_first_scope = function_scopes[&call.label].n_outputs;
        }

        Scope::Condition {
            instructions_if_true,
            instructions_if_false,
        } => {
            let (true_n_inputs, true_n_outputs): (usize, Option<usize>) =
                get_n_inputs_and_outputs_until_end(
                    &instructions_if_true,
                    function_alias_already_filled,
                    function_scopes,
                );
            let (false_n_inputs, false_n_outputs): (usize, Option<usize>) =
                get_n_inputs_and_outputs_until_end(
                    &instructions_if_false,
                    function_alias_already_filled,
                    function_scopes,
                );
            n_inputs_at_first_scope = true_n_inputs.max(false_n_inputs);
            n_outputs_at_first_scope = match (true_n_outputs, false_n_outputs) {
                (None, None) => None,
                (Some(_true_n_outputs), None) => Some(_true_n_outputs),
                (None, Some(_false_n_outputs)) => Some(_false_n_outputs),
                (Some(_true_n_outputs), Some(_false_n_outputs)) => {
                    let ajusted_true_n_outputs: usize =
                        _true_n_outputs + false_n_inputs.saturating_sub(true_n_inputs);

                    let ajusted_false_n_outputs: usize =
                        _false_n_outputs + true_n_inputs.saturating_sub(false_n_inputs);
                    // If-Else ends with diverging stack size, probably because the 2 branches never join again
                    Some(ajusted_true_n_outputs.max(ajusted_false_n_outputs))
                }
            }
        }
        Scope::Panic => {
            assert!(instructions.len() == 1);
            n_inputs_at_first_scope = 0;
            n_outputs_at_first_scope = None;
        }
        Scope::FunctionReturn(_) => {
            n_inputs_at_first_scope = 0;
            n_outputs_at_first_scope = Some(0);
        }
        Scope::Loop { label: _ } => {
            n_inputs_at_first_scope = 0;
            n_outputs_at_first_scope = Some(0);
        }
        Scope::LoopContinue { label: _ } => {
            n_inputs_at_first_scope = 0;
            n_outputs_at_first_scope = Some(0);
        }
        Scope::Empty => {
            n_inputs_at_first_scope = 0;
            n_outputs_at_first_scope = Some(0);
        }
    }

    let (following_n_inputs, following_n_outputs): (usize, Option<usize>) =
        get_n_inputs_and_outputs_until_end(
            &instructions[1..],
            function_alias_already_filled,
            function_scopes,
        );

    if let Some(_n_outputs_at_first_scope) = n_outputs_at_first_scope {
        if let Some(_following_n_outputs) = following_n_outputs {
            let (n_stack_inputs, n_stack_outputs) = aggregate_n_stack_inputs_and_outputs(
                n_inputs_at_first_scope,
                _n_outputs_at_first_scope,
                following_n_inputs,
                _following_n_outputs,
            );
            return (n_stack_inputs, Some(n_stack_outputs));
        } else {
            return (
                aggregate_n_stack_inputs(
                    n_inputs_at_first_scope,
                    _n_outputs_at_first_scope,
                    following_n_inputs,
                ),
                None,
            );
        }
    } else {
        return (n_inputs_at_first_scope, None);
    }
}
