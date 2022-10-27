use crate::execution_flow::{
    execution_flow::Scope,
    with_variables::{
        incorporate_variables::{Line, Value, Variable},
        scopes_with_vars::VarScope,
    },
};
use std::collections::HashMap;

pub fn count_uses_per_var_in_line(line: &Line, uses_per_var: &mut HashMap<Variable, usize>) {
    match line {
        Line::Assignement {
            receiving_var,
            assigned_value,
        } => {
            if let Some(_receiving_var) = receiving_var {
                *uses_per_var.entry(*_receiving_var).or_insert(0) += 1;
            }
            count_uses_per_var_in_value(assigned_value, uses_per_var);
        }
        Line::If { condition } => count_uses_per_var_in_value(condition, uses_per_var),
        Line::Empty => (),
    }
}

pub fn count_uses_per_var_in_value(value: &Value, uses_per_var: &mut HashMap<Variable, usize>) {
    match value {
        Value::Calculation { opcode: _, args } => {
            for arg_value in args {
                count_uses_per_var_in_value(arg_value, uses_per_var);
            }
        }
        Value::Existing(var) => {
            *uses_per_var.entry(*var).or_insert(0) += 1;
        }
        Value::Bytes(_) => (),
        Value::FunctionReturnedValue {
            label: _,
            arguments,
            return_index: _,
        } => {
            for arg_value in arguments {
                count_uses_per_var_in_value(arg_value, uses_per_var);
            }
        }
    }
}

fn find_variable_depth(
    scopes: &Vec<VarScope>,
    depth_per_variable: &mut HashMap<Variable, usize>,
    starting_depth: usize,
) -> usize {
    // 'depth' represent approximately the first line index at which a var is defined
    let mut current_depth: usize = starting_depth;
    for scope in scopes {
        match scope {
            Scope::Instructions(instructions) => {
                for line in &instructions.lines {
                    if let Line::Assignement {
                        receiving_var: Some(receiving_var),
                        assigned_value: _,
                    } = line
                    {
                        if !depth_per_variable.contains_key(receiving_var) {
                            depth_per_variable.insert(*receiving_var, current_depth);
                        }
                    }
                    current_depth += 1;
                }
            }
            Scope::FunctionCall(function_call) => {
                for res_var in &function_call.results {
                    if !depth_per_variable.contains_key(res_var) {
                        depth_per_variable.insert(*res_var, current_depth);
                    }
                    current_depth += 1;
                }
            }
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                current_depth =
                    find_variable_depth(instructions_if_true, depth_per_variable, current_depth);
                current_depth =
                    find_variable_depth(instructions_if_false, depth_per_variable, current_depth);
            }
            _ => (),
        }
    }
    return current_depth;
}

pub fn get_vars_ordered_by_depth(scopes: &Vec<VarScope>) -> Vec<Variable> {
    let mut depth_per_variable: HashMap<Variable, usize> = HashMap::new();
    find_variable_depth(scopes, &mut depth_per_variable, 0);
    let mut var_and_depths: Vec<(&Variable, &usize)> = depth_per_variable.iter().collect();
    var_and_depths.sort_by_key(|(_, depth)| *depth);
    return var_and_depths.iter().map(|(var, _)| **var).collect();
}
