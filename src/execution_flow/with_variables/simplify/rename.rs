use crate::{
    execution_flow::{
        execution_flow::{
            AccessContent, FunctionLabel, GetFunctionLabel, Scope, MAIN_FUNCTION_LABEL,
        },
        with_variables::{
            flow_with_vars::ExecutionFlowWithVars,
            incorporate_variables::{Line, Value, Variable},
            scopes_with_vars::VarScope,
        },
    },
    tools::utils::{dedup_all, rename_keys},
};
use std::collections::HashMap;

impl ExecutionFlowWithVars {
    pub fn rename_variables(&mut self, var_mapping: &HashMap<Variable, Variable>) {
        for (_, function) in &mut self.functions {
            rename_variables_in_scopes(function.get_content_mut(), var_mapping)
        }
    }

    pub fn rename_function_labels(
        &mut self,
        label_mapping: &HashMap<FunctionLabel, FunctionLabel>,
    ) {
        for (_, function) in &mut self.functions {
            rename_function_labels_in_scopes(&mut function.content, label_mapping);
            if let Some(new_label) = label_mapping.get(&function.label) {
                function.label = *new_label;
            }
        }
        // don't forget that labels are used as keys in `flow_with_vars.functions`:
        let previous_n_functions: usize = self.functions.len();
        self.functions = rename_keys(&self.functions, label_mapping, true);
        let new_n_functions: usize = self.functions.len();
        let n_deletions: usize = previous_n_functions - new_n_functions;
        if n_deletions > 0 {
            log::info!("{} unused functions were deleted, probably because we did'nt reach the moment they are called due to panics in the execution flow.",n_deletions);
        }
    }

    pub fn rename_functions_starting_from_zero(&mut self) {
        let function_labels_ordered_by_depth: Vec<FunctionLabel> =
            self.get_functions_ordered_by_depth();
        let mut label_mapping: HashMap<FunctionLabel, FunctionLabel> = HashMap::new();
        for (index, label) in function_labels_ordered_by_depth.iter().enumerate() {
            label_mapping.insert(*label, index as u64);
        }
        label_mapping.insert(MAIN_FUNCTION_LABEL, MAIN_FUNCTION_LABEL);
        self.rename_function_labels(&label_mapping);
    }

    fn get_functions_ordered_by_depth(&mut self) -> Vec<FunctionLabel> {
        let mut labels: Vec<FunctionLabel> = Vec::new();
        labels.extend(get_functions_ordered_by_depth_in_scopes(
            self.get_main_function().get_content(),
        ));

        dedup_all(&mut labels);
        return labels;
    }
}

fn rename_variable(current_variable: &mut Variable, var_mapping: &HashMap<Variable, Variable>) {
    if let Some(new_variable) = var_mapping.get(&current_variable) {
        *current_variable = *new_variable;
    }
}

fn rename_list_variables(
    current_variables: &mut Vec<Variable>,
    var_mapping: &HashMap<Variable, Variable>,
) {
    for i in 0..current_variables.len() {
        rename_variable(&mut current_variables[i], var_mapping);
    }
}

fn rename_variables_in_line(line: &mut Line, var_mapping: &HashMap<Variable, Variable>) {
    match line {
        Line::Assignement {
            receiving_var,
            assigned_value,
        } => {
            if let Some(_receiving_var) = receiving_var {
                rename_variable(_receiving_var, var_mapping);
            }
            rename_variables_in_value(assigned_value, var_mapping);
        }
        Line::If { condition } => rename_variables_in_value(condition, var_mapping),
        Line::Empty => (),
    }
}

fn rename_variables_in_value(value: &mut Value, var_mapping: &HashMap<Variable, Variable>) {
    match value {
        Value::Calculation { opcode: _, args } => {
            for arg in args {
                rename_variables_in_value(arg, var_mapping);
            }
        }
        Value::Existing(var) => rename_variable(var, var_mapping),
        Value::Bytes(_) => (),
        Value::FunctionReturnedValue {
            label: _,
            arguments,
            return_index: _,
        } => {
            for arg_value in arguments {
                rename_variables_in_value(arg_value, var_mapping);
            }
        }
    }
}

pub fn rename_variables_in_scopes(
    scopes: &mut Vec<VarScope>,
    var_mapping: &HashMap<Variable, Variable>,
) {
    for scope in scopes {
        match scope {
            Scope::Instructions(instructions) => {
                for line in &mut instructions.lines {
                    rename_variables_in_line(line, var_mapping);
                }
            }
            Scope::FunctionCall(interna_function_call) => {
                for arg_value in &mut interna_function_call.arguments {
                    rename_variables_in_value(arg_value, var_mapping);
                }
                rename_list_variables(&mut interna_function_call.results, var_mapping);
            }
            Scope::FunctionReturn(function_return) => {
                for return_value in &mut function_return.returned_values {
                    rename_variables_in_value(return_value, var_mapping);
                }
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                rename_variables_in_scopes(instructions_if_true, var_mapping);
                rename_variables_in_scopes(instructions_if_false, var_mapping);
            }
            Scope::Loop { label: _ }
            | Scope::LoopContinue { label: _ }
            | Scope::Panic
            | Scope::Empty => (),
        }
    }
}

fn rename_function_labels_in_scopes(
    scopes: &mut Vec<VarScope>,
    label_mapping: &HashMap<FunctionLabel, FunctionLabel>,
) {
    for scope in scopes {
        match scope {
            Scope::FunctionCall(function_call) => {
                if let Some(new_label) = label_mapping.get(&function_call.get_label()) {
                    function_call.label = *new_label;
                }
            }
            Scope::FunctionReturn(function_return_with_vars) => {
                if let Some(new_label) = label_mapping.get(&function_return_with_vars.get_label()) {
                    function_return_with_vars.label = *new_label;
                }
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                rename_function_labels_in_scopes(instructions_if_true, label_mapping);
                rename_function_labels_in_scopes(instructions_if_false, label_mapping);
            }
            _ => (),
        }
    }
}

fn find_function_depth(
    scopes: &Vec<VarScope>,
    depth_per_function: &mut HashMap<FunctionLabel, usize>,
    starting_depth: usize,
) -> usize {
    // 'depth' is approximately proportional to to the line index at which an function is called for the first time
    let mut current_depth: usize = starting_depth;
    for scope in scopes {
        match scope {
            Scope::FunctionCall(function_call) => {
                if !depth_per_function.contains_key(&function_call.get_label()) {
                    depth_per_function.insert(function_call.get_label(), current_depth);
                }
                current_depth += 1;
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                current_depth =
                    find_function_depth(instructions_if_true, depth_per_function, current_depth);
                current_depth =
                    find_function_depth(instructions_if_false, depth_per_function, current_depth);
            }
            _ => (),
        }
    }
    return current_depth;
}

fn get_functions_ordered_by_depth_in_scopes(scopes: &Vec<VarScope>) -> Vec<FunctionLabel> {
    let mut depth_per_function: HashMap<FunctionLabel, usize> = HashMap::new();
    find_function_depth(scopes, &mut depth_per_function, 0);
    let mut function_and_depths: Vec<(&FunctionLabel, &usize)> =
        depth_per_function.iter().collect();
    function_and_depths.sort_by_key(|(_, depth)| *depth);
    return function_and_depths.iter().map(|(var, _)| **var).collect();
}
