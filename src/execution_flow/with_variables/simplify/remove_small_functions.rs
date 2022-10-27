use std::collections::HashMap;

use crate::execution_flow::{
    execution_flow::{AccessContent, FunctionLabel, GetFunctionLabel, Scope},
    with_variables::{
        flow_with_vars::ExecutionFlowWithVars,
        incorporate_variables::{Line, Value, Variable},
        scopes_with_vars::{FunctionWithVars, InstructionsWithVars, VarScope},
        simplify::replace_var::replace_vars_until_second_assignment,
    },
};

fn replace_function_call_by_content_in_scopes(
    scopes: &mut Vec<VarScope>,
    label_to_replace: FunctionLabel,
    function_content: &FunctionWithVars,
) {
    for scope_index in (0..scopes.len()).rev() {
        match &mut scopes[scope_index] {
            Scope::FunctionCall(function_call) => {
                if label_to_replace == function_call.get_label() {
                    let mut converted_content: Vec<VarScope> =
                        function_content.get_content().clone();

                    replace_function_returns_by_var_assignment(
                        &mut converted_content,
                        label_to_replace,
                        &function_call.results,
                    );

                    assert!(function_call.arguments.len() == function_content.input_vars.len());

                    let mut to_replace: HashMap<Variable, Value> = HashMap::new();
                    for arg_index in 0..function_call.arguments.len() {
                        let input_arg: Variable = function_content.input_vars[arg_index];
                        let arg_value: Value = function_call.arguments[arg_index].clone();
                        to_replace.insert(input_arg, arg_value);
                    }
                    replace_vars_until_second_assignment(&mut converted_content, &to_replace);

                    // TODO try: scopes.splice(scope_index+1..scope_index, function_content.clone());
                    scopes.remove(scope_index);
                    scopes.splice(scope_index..scope_index, converted_content);
                }
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                replace_function_call_by_content_in_scopes(
                    instructions_if_true,
                    label_to_replace,
                    function_content,
                );
                replace_function_call_by_content_in_scopes(
                    instructions_if_false,
                    label_to_replace,
                    function_content,
                );
            }
            _ => (),
        }
    }
}

fn replace_function_returns_by_var_assignment(
    scopes: &mut Vec<VarScope>,
    return_label_to_remove: FunctionLabel,
    receiving_vars: &Vec<Variable>,
) {
    for scope_index in 0..scopes.len() {
        match &mut scopes[scope_index] {
            Scope::FunctionReturn(function_return) => {
                if function_return.get_label() != return_label_to_remove {
                    log::warn!("scopes should only contain 0 or 1 different labels for function returns: found {} instead of {}.", function_return.get_label(), return_label_to_remove);
                    continue;
                }
                assert!(receiving_vars.len() == function_return.returned_values.len());
                // TODO avoid replacing with empty lines and remove at scope_index instead when receiving_vars.len() == 0 ?
                let mut lines: Vec<Line> = Vec::new();
                for assign_index in 0..receiving_vars.len() {
                    lines.push(Line::Assignement {
                        receiving_var: Some(receiving_vars[assign_index]),
                        assigned_value: function_return.returned_values[assign_index].clone(),
                    })
                }
                let assignment_instructions: InstructionsWithVars = InstructionsWithVars { lines };
                scopes[scope_index] = Scope::Instructions(assignment_instructions);
                // removefunction return and replace by var assignment
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                replace_function_returns_by_var_assignment(
                    instructions_if_true,
                    return_label_to_remove,
                    receiving_vars,
                );
                replace_function_returns_by_var_assignment(
                    instructions_if_false,
                    return_label_to_remove,
                    receiving_vars,
                );
            }
            _ => (),
        }
    }
}

fn should_function_with_vars_exist(function: &FunctionWithVars, n_uses: usize) -> bool {
    if function.is_main() {
        return true;
    }
    if n_uses <= 1 {
        return false;
    }
    if function.get_content().is_empty() {
        return false;
    }
    let length: usize = ExecutionFlowWithVars::compute_size_of_scopes(function.get_content());
    if length <= 1 {
        return false;
    }
    return n_uses * length >= 6;

    // TODO do not duplicate recursivefunctions
}

impl ExecutionFlowWithVars {
    pub fn remove_small_functions(&mut self) {
        self.remove_functions(
            should_function_with_vars_exist,
            replace_function_call_by_content_in_scopes,
        );
    }
}
