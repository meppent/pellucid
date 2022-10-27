use super::{
    incorporate_variables::{convert_vopcodes_to_lines, Line, Value, Variable, VariablesStack},
    scopes_with_vars::{
        FunctionCallWithVars, FunctionReturnWithVars, FunctionWithVars, InstructionsWithVars,
        VarScope,
    },
};
use crate::execution_flow::{
    execution_flow::{ExecutionFlow, FunctionLabel, GetFunctionLabel, LoopLabel, Scope},
    with_opcodes::{
        flow_with_opcodes::ExecutionFlowWithOpcodes,
        scopes_with_opcodes::{FunctionWithOpcodes, OpcodeScope},
    },
};
use std::collections::{HashMap, HashSet};

pub type ExecutionFlowWithVars = ExecutionFlow<
    InstructionsWithVars,
    FunctionCallWithVars,
    FunctionReturnWithVars,
    FunctionWithVars,
>;

pub fn convert_opcode_flow_to_var_flow(
    opcode_flow: &ExecutionFlowWithOpcodes,
) -> ExecutionFlowWithVars {
    let mut var_equivalences: Vec<(Variable, Variable)> = Vec::new();

    let mut functions_with_vars: HashMap<FunctionLabel, FunctionWithVars> = HashMap::new();
    for (label, function_with_opcodes) in &opcode_flow.functions {
        let mut initial_stack: VariablesStack = VariablesStack::new();
        let input_vars: Vec<Variable> =
            initial_stack.create_and_push_vars(function_with_opcodes.n_inputs);
        let (n_outputs, returns): (usize, bool) =
            if let Some(n_outputs) = &function_with_opcodes.n_outputs {
                (*n_outputs, true)
            } else {
                (0, false)
            };
        let (_, content): (VariablesStack, Vec<VarScope>) = convert_opcode_scopes_to_var_scopes(
            &initial_stack,
            &function_with_opcodes.content,
            &opcode_flow.functions,
            &mut HashMap::new(),
            &mut var_equivalences,
        );

        let converted_function: FunctionWithVars = FunctionWithVars {
            label: *label,
            input_vars,
            n_outputs,
            returns,
            content,
        };
        functions_with_vars.insert(*label, converted_function);
    }

    let mut execution_flow_with_vars: ExecutionFlowWithVars =
        ExecutionFlow::new(functions_with_vars);

    let var_mapping: HashMap<Variable, Variable> =
        convert_aquivalences_to_var_mapping(&var_equivalences);

    execution_flow_with_vars.rename_variables(&var_mapping);

    execution_flow_with_vars.simplify();
    return execution_flow_with_vars;
}

impl ExecutionFlowWithVars {
    pub fn simplify(&mut self) {
        self.simplify_vars();
        self.remove_empty_elements();
        self.remove_small_functions(); // must be done before variable renaming
        self.rename_variables_starting_from_zero();
        self.rename_functions_starting_from_zero();
    }
}

fn convert_opcode_scopes_to_var_scopes(
    initial_stack: &VariablesStack,
    opcode_scopes: &[OpcodeScope],
    functions_with_opcodes: &HashMap<FunctionLabel, FunctionWithOpcodes>,
    stack_at_loop_starts: &mut HashMap<LoopLabel, VariablesStack>,
    var_equivalences: &mut Vec<(Variable, Variable)>,
) -> (VariablesStack, Vec<VarScope>) {
    let mut var_scopes: Vec<VarScope> = Vec::new();
    let mut current_stack: VariablesStack = initial_stack.clone();
    for (opcode_scope_index, opcode_scope) in opcode_scopes.iter().enumerate() {
        match opcode_scope {
            Scope::Loop { label } => {
                var_scopes.push(Scope::Loop { label: *label });
                //assert!(!stack_at_loop_starts.contains_key(label)); // assert fails due to duplications... TODO ?
                stack_at_loop_starts.insert(*label, current_stack.clone());
            }
            Scope::LoopContinue { label } => {
                if let Some(stack_at_loop_start) = stack_at_loop_starts.get(label) {
                    assert!(stack_at_loop_start.len() == current_stack.len());
                    for stack_depth in 0..stack_at_loop_start.len() {
                        var_equivalences.push((
                            *stack_at_loop_start.peek_at(stack_depth),
                            *current_stack.peek_at(stack_depth),
                        ));
                    }
                } else {
                    log::warn!("Continue before loop ??");
                }

                var_scopes.push(Scope::LoopContinue { label: *label })
            }
            Scope::Panic => var_scopes.push(Scope::Panic),
            Scope::Empty => (), // ne need to keep it
            Scope::FunctionReturn(function_return_with_opcodes) => {
                assert!(opcode_scope_index == opcode_scopes.len() - 1);
                let n_returned_vars: usize = functions_with_opcodes
                    [&function_return_with_opcodes.get_label()]
                    .n_outputs
                    .unwrap();
                var_scopes.push(Scope::FunctionReturn(FunctionReturnWithVars {
                    label: function_return_with_opcodes.get_label(),
                    returned_values: Value::from_vars(&current_stack.multi_pop(n_returned_vars)),
                }));
            }
            Scope::Instructions(opcode_instructions) => {
                let instructions: Vec<Line>;
                (current_stack, instructions) =
                    convert_vopcodes_to_lines(&current_stack, &opcode_instructions.code);
                var_scopes.push(VarScope::Instructions(InstructionsWithVars {
                    lines: instructions,
                }));
            }
            Scope::FunctionCall(call) => {
                let n_arguments: usize = functions_with_opcodes[&call.get_label()].n_inputs;
                let n_results: usize = functions_with_opcodes[&call.get_label()]
                    .n_outputs
                    .unwrap_or(0);

                let mut arguments: Vec<Value> =
                    Value::from_vars(&current_stack.multi_pop(n_arguments));
                arguments.reverse();
                let results: Vec<Variable> = current_stack.create_and_push_vars(n_results);
                var_scopes.push(Scope::FunctionCall(FunctionCallWithVars {
                    label: call.get_label(),
                    arguments,
                    results,
                }));
            }
            Scope::Condition {
                instructions_if_true: opcode_instructions_if_true,
                instructions_if_false: opcode_instructions_if_false,
            } => {
                let (resulting_stack_if_true, converted_instructions_if_true): (
                    VariablesStack,
                    Vec<VarScope>,
                ) = convert_opcode_scopes_to_var_scopes(
                    &current_stack,
                    &opcode_instructions_if_true,
                    functions_with_opcodes,
                    stack_at_loop_starts,
                    var_equivalences,
                );
                let (resulting_stack_if_false, converted_instructions_if_false): (
                    VariablesStack,
                    Vec<VarScope>,
                ) = convert_opcode_scopes_to_var_scopes(
                    &current_stack,
                    &opcode_instructions_if_false,
                    functions_with_opcodes,
                    stack_at_loop_starts,
                    var_equivalences,
                );

                let (stack_to_change, target_stack): (VariablesStack, VariablesStack) =
                    if resulting_stack_if_true.len() < resulting_stack_if_false.len() {
                        (resulting_stack_if_true, resulting_stack_if_false)
                    } else {
                        (resulting_stack_if_false, resulting_stack_if_true)
                    };

                current_stack = target_stack.clone();
                var_scopes.push(Scope::Condition {
                    instructions_if_true: converted_instructions_if_true,
                    instructions_if_false: converted_instructions_if_false,
                });

                if opcode_scope_index != opcode_scopes.len() - 1 {
                    // TODO handle case if {if{}else{}}else{}
                    // In case the execution continues after both conditions (if else THEN):
                    // We rename the variables in the condition ending with smallest ending stack such that the latter
                    // matches the ending stack of the other condition:
                    for stack_depth in 0..stack_to_change.len() {
                        var_equivalences.push((
                            *stack_to_change.peek_at(stack_depth),
                            *target_stack.peek_at(stack_depth),
                        ));
                    }
                }
            }
        }
    }
    return (current_stack, var_scopes);
}

fn convert_aquivalences_to_var_mapping(
    var_equivalences: &Vec<(Variable, Variable)>,
) -> HashMap<Variable, Variable> {
    let mut var_to_equivalent_vars: HashMap<Variable, HashSet<Variable>> = HashMap::new();
    for (var0, var1) in var_equivalences {
        var_to_equivalent_vars
            .entry(*var0)
            .or_insert(HashSet::new())
            .insert(*var1);
        var_to_equivalent_vars
            .entry(*var1)
            .or_insert(HashSet::new())
            .insert(*var0);
    }
    let mut var_mapping: HashMap<Variable, Variable> = HashMap::new();
    let mut seen_vars: HashSet<Variable> = HashSet::new();
    for (var, eq_vars) in var_to_equivalent_vars {
        if !seen_vars.contains(&var) {
            seen_vars.insert(var);
            seen_vars.extend(&eq_vars);
            for eq_var in eq_vars {
                var_mapping.insert(eq_var, var);
            }
        }
    }
    return var_mapping;
}
