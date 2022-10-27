use super::{
    count_vars::get_vars_ordered_by_depth, initialization_of_vars::enumerate_var_initializations,
    replace_var::replace_vars_in_value,
};
use crate::execution_flow::{
    execution_flow::{AccessContent, Scope},
    with_variables::{
        flow_with_vars::ExecutionFlowWithVars,
        incorporate_variables::{Line, Value, Variable},
        scopes_with_vars::VarScope,
        simplify::rename::rename_variables_in_scopes,
    },
};
use std::collections::{HashMap, HashSet};

impl ExecutionFlowWithVars {
    pub fn remove_empty_elements(&mut self) {
        for (_, function) in &mut self.functions {
            remove_empty_elements_in_scopes(function.get_content_mut());
        }
    }
    pub fn rename_variables_starting_from_zero(&mut self) {
        for (_, function) in &mut self.functions {
            rename_variables_starting_from_zero_in_scopes(function.get_content_mut());
        }
    }
    pub fn simplify_vars(&mut self) {
        for (_, function) in &mut self.functions {
            simplify_vars_in_scopes(function.get_content_mut());
        }
    }
}

pub fn simplify_vars_in_scopes(scopes: &mut Vec<VarScope>) {
    let n_initializations_per_var: HashMap<Variable, usize> = enumerate_var_initializations(scopes);

    let duplicable_vars: HashSet<Variable> = n_initializations_per_var
        .iter()
        .filter(|(_, n_init)| **n_init <= 1)
        .map(|(var, _)| *var)
        .collect();
    _simplify_vars_in_scopes(scopes, &duplicable_vars, &mut HashMap::new());
}

fn _simplify_vars_in_scopes(
    scopes: &mut Vec<VarScope>,
    duplicable_vars: &HashSet<Variable>,
    to_replace: &mut HashMap<Variable, Value>,
) {
    for scope in scopes {
        match scope {
            Scope::Instructions(instructions) => {
                for line in &mut instructions.lines {
                    match line {
                        Line::Assignement {
                            receiving_var,
                            assigned_value,
                        } => {
                            replace_vars_in_value(assigned_value, to_replace);

                            if let Some(_receiving_var) = receiving_var {
                                assert!(!to_replace.contains_key(_receiving_var));
                                if duplicable_vars.contains(_receiving_var)
                                    && should_value_be_duplicated(assigned_value, duplicable_vars)
                                {
                                    to_replace.insert(*_receiving_var, assigned_value.clone());
                                    *line = Line::Empty;
                                }
                            }
                        }
                        Line::If { condition } => {
                            replace_vars_in_value(condition, to_replace);
                        }
                        Line::Empty => (),
                    }
                }
            }
            Scope::FunctionCall(internal_function_call) => {
                for arg_value in &mut internal_function_call.arguments {
                    replace_vars_in_value(arg_value, to_replace);
                }
            }
            Scope::FunctionReturn(internal_function_return) => {
                for ret_value in &mut internal_function_return.returned_values {
                    replace_vars_in_value(ret_value, to_replace);
                }
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                _simplify_vars_in_scopes(instructions_if_true, duplicable_vars, to_replace);
                _simplify_vars_in_scopes(instructions_if_false, duplicable_vars, to_replace);
            }
            Scope::LoopContinue { label: _ }
            | Scope::Loop { label: _ }
            | Scope::Empty
            | Scope::Panic => (),
        }
    }
}
fn should_value_be_duplicated(value: &Value, duplicable_vars: &HashSet<Variable>) -> bool {
    match value {
        Value::Calculation { opcode, args } => {
            !opcode.has_effect()
                && value.size() <= 12
                && args
                    .iter()
                    .all(|arg_value| should_value_be_duplicated(arg_value, duplicable_vars))
        }
        Value::Existing(variable) => duplicable_vars.contains(variable),
        Value::Bytes(_) => true,
        Value::FunctionReturnedValue {
            label: _,
            arguments: _,
            return_index: _,
        } => false,
    }
}

fn remove_empty_elements_in_scopes(scopes: &mut Vec<VarScope>) {
    for scope in scopes.iter_mut() {
        match scope {
            Scope::Instructions(instructions) => instructions
                .lines
                .retain(|line: &Line| *line != Line::Empty),
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                remove_empty_elements_in_scopes(instructions_if_true);
                remove_empty_elements_in_scopes(instructions_if_false);
            }
            _ => (),
        }
    }
    scopes.retain(|scope: &VarScope| !scope.is_empty());
}

fn rename_variables_starting_from_zero_in_scopes(scopes: &mut Vec<VarScope>) {
    let vars_ordered_by_depth: Vec<Variable> = get_vars_ordered_by_depth(scopes);
    let mut var_mapping: HashMap<Variable, Variable> = HashMap::new();
    for (index, var) in vars_ordered_by_depth.iter().enumerate() {
        var_mapping.insert(*var, Variable { alias: index });
    }
    rename_variables_in_scopes(scopes, &var_mapping);
}
