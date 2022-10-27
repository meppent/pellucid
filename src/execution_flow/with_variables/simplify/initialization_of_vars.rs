use crate::execution_flow::{
    execution_flow::Scope,
    with_variables::{
        flow_with_vars::ExecutionFlowWithVars,
        incorporate_variables::{Line, Value, Variable},
        scopes_with_vars::VarScope,
    },
};
use std::collections::{HashMap, HashSet};

pub fn enumerate_var_initializations(scopes: &Vec<VarScope>) -> HashMap<Variable, usize> {
    let mut n_initializations_per_var: HashMap<Variable, usize> = find_vars_used_in_scopes(scopes)
        .iter()
        .map(|var| (*var, 0))
        .collect();
    _enumerate_var_initializations(scopes, &mut n_initializations_per_var);
    return n_initializations_per_var;
}

fn _enumerate_var_initializations(
    scopes: &Vec<VarScope>,
    n_initializations_per_var: &mut HashMap<Variable, usize>,
) {
    // At the of this function `initial_values_per_var[X]` is the list of all the different values with which the variable X was initialized in `scopes`.
    // A variable may be initialized more than once in case of an If/Else scope: if(...){...let X = value_a ...}else{...let X = value_b ...}

    for scope in scopes {
        match scope {
            Scope::Loop { label: _ } => (),
            Scope::LoopContinue { label: _ } => (),
            Scope::Panic => (),
            Scope::Instructions(var_instructions) => {
                for line in &var_instructions.lines {
                    if let Line::Assignement {
                        receiving_var: Some(_receiving_var),
                        assigned_value: _,
                    } = line
                    {
                        *n_initializations_per_var.get_mut(_receiving_var).unwrap() += 1;
                    }
                }
            }
            Scope::FunctionCall(function_call) => {
                for var in &function_call.results {
                    *n_initializations_per_var.get_mut(var).unwrap() += 1;
                }
            }
            Scope::FunctionReturn(_) => (),
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                _enumerate_var_initializations(instructions_if_true, n_initializations_per_var);
                _enumerate_var_initializations(instructions_if_false, n_initializations_per_var);
            }
            Scope::Empty => (),
        }
    }
}

pub fn find_vars_used_in_scopes(scopes: &Vec<VarScope>) -> HashSet<Variable> {
    let mut used_vars: HashSet<Variable> = HashSet::new();
    ExecutionFlowWithVars::apply_on_scopes(scopes, &mut |scope| match scope {
        Scope::Instructions(instructions) => {
            for line in &instructions.lines {
                match line {
                    Line::Assignement {
                        receiving_var,
                        assigned_value,
                    } => {
                        if let Some(_receiving_var) = receiving_var {
                            used_vars.insert(*_receiving_var);
                        }
                        used_vars.extend(find_vars_used_in_value(assigned_value))
                    }
                    Line::If { condition } => used_vars.extend(find_vars_used_in_value(condition)),
                    Line::Empty => (),
                }
            }
        }
        Scope::FunctionCall(function_call) => {
            used_vars.extend(&function_call.results);
            used_vars.extend(find_vars_used_in_values(&function_call.arguments))
        }
        Scope::FunctionReturn(function_return) => {
            used_vars.extend(find_vars_used_in_values(&function_return.returned_values))
        }
        _ => (),
    });
    return used_vars;
}

fn find_vars_used_in_values(values: &Vec<Value>) -> HashSet<Variable> {
    let mut used_vars: HashSet<Variable> = HashSet::new();
    for value in values {
        used_vars.extend(find_vars_used_in_value(value));
    }
    return used_vars;
}

fn find_vars_used_in_value(value: &Value) -> HashSet<Variable> {
    let mut used_vars: HashSet<Variable> = HashSet::new();
    match value {
        Value::Calculation { opcode: _, args } => used_vars.extend(find_vars_used_in_values(args)),
        Value::Existing(variable) => {
            used_vars.insert(*variable);
        }
        Value::FunctionReturnedValue {
            label: _,
            arguments,
            return_index: _,
        } => used_vars.extend(find_vars_used_in_values(arguments)),
        Value::Bytes(_) => (),
    }
    return used_vars;
}
