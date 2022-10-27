use std::collections::HashMap;

use crate::execution_flow::{
    execution_flow::Scope,
    with_variables::{
        incorporate_variables::{Line, Value, Variable},
        scopes_with_vars::VarScope,
    },
};

pub fn replace_vars_until_second_assignment(
    scopes: &mut Vec<VarScope>,
    to_replace: &HashMap<Variable, Value>,
) {
    _replace_var_in_scopes_until_second_assignment(scopes, to_replace, &mut HashMap::new());
}

fn _replace_var_in_scopes_until_second_assignment(
    scopes: &mut Vec<VarScope>,
    to_replace: &HashMap<Variable, Value>,
    init_counts: &mut HashMap<Variable, usize>,
) {
    for scope in scopes {
        replace_var_in_scope_second_assignment(scope, &to_replace, init_counts);
    }
}

fn replace_var_in_scope_second_assignment(
    scope: &mut VarScope,
    to_replace: &HashMap<Variable, Value>,
    init_counts: &mut HashMap<Variable, usize>,
) {
    match scope {
        Scope::Instructions(instructions) => {
            for line in &mut instructions.lines {
                replace_var_in_line_until_second_assignment(line, to_replace, init_counts);
            }
        }
        Scope::FunctionCall(_) => {
            replace_var_in_function_call_until_second_assignment(scope, to_replace, init_counts)
        }
        Scope::FunctionReturn(function_return) => {
            for returned_value in &mut function_return.returned_values {
                replace_var_in_value_until_second_assignment(
                    returned_value,
                    to_replace,
                    init_counts,
                )
            }
        }
        Scope::Loop { label: _ } => (),
        Scope::LoopContinue { label: _ } => (),
        Scope::Condition {
            instructions_if_true,
            instructions_if_false,
        } => {
            _replace_var_in_scopes_until_second_assignment(
                instructions_if_true,
                to_replace,
                init_counts,
            );

            _replace_var_in_scopes_until_second_assignment(
                instructions_if_false,
                to_replace,
                init_counts,
            );
        }
        Scope::Panic => (),
        Scope::Empty => (),
    }
}

fn replace_var_in_line_until_second_assignment(
    line: &mut Line,
    to_replace: &HashMap<Variable, Value>,
    init_counts: &mut HashMap<Variable, usize>,
) {
    match line {
        Line::Assignement {
            receiving_var,
            assigned_value,
        } => {
            replace_var_in_value_until_second_assignment(assigned_value, to_replace, init_counts);
            if receiving_var.is_none() {
                return;
            }
            let receiving_var: Variable = receiving_var.unwrap().clone();
            if to_replace.contains_key(&receiving_var) {
                if *init_counts.get(&receiving_var).unwrap_or(&0) == 0 {
                    // erase the first initialization of 'replaced_var'
                    *line = Line::Empty;
                }
                *init_counts.entry(receiving_var).or_insert(0) += 1;
            }
        }
        Line::If { condition } => {
            replace_var_in_value_until_second_assignment(condition, to_replace, init_counts)
        }
        Line::Empty => (),
    }
}

fn replace_var_in_function_call_until_second_assignment(
    function_call_scope: &mut VarScope,
    to_replace: &HashMap<Variable, Value>,
    init_counts: &mut HashMap<Variable, usize>,
) {
    if let Scope::FunctionCall(function_call) = function_call_scope {
        for (replaced_var, _) in to_replace {
            if function_call.results.contains(&replaced_var) {
                assert!(function_call.results.len()==1, "We should not replace a variable that is part of >= 2 returned values by anfunction, otherwise we should use something like `f(x).0` for `a` and `f(x).1` for `b` to replace `(a,b) = f(x)`");
                if *init_counts.get(replaced_var).unwrap_or(&0) == 0 {
                    *function_call_scope = Scope::Empty; // erase this call to a function
                    *init_counts.entry(*replaced_var).or_insert(0) += 1;
                    return;
                }
            }
        }
        for arg_value in &mut function_call.arguments {
            replace_var_in_value_until_second_assignment(arg_value, to_replace, init_counts)
        }
    } else {
        panic!();
    }
}

fn replace_var_in_value_until_second_assignment(
    value: &mut Value,
    to_replace: &HashMap<Variable, Value>,
    init_counts: &mut HashMap<Variable, usize>,
) {
    match value {
        Value::Calculation { opcode: _, args } => {
            for arg_value in args {
                replace_var_in_value_until_second_assignment(arg_value, to_replace, init_counts);
            }
        }
        Value::Existing(var) => {
            if let Some(replacing_value) = to_replace.get(var) {
                // assert!(init_counts > 0); TODO why this reverts
                if *init_counts.get(var).unwrap_or(&0) <= 1 {
                    *value = replacing_value.clone();
                }
            }
        }
        Value::Bytes(_) => (),
        Value::FunctionReturnedValue {
            label: _,
            arguments,
            return_index: _,
        } => {
            for arg_value in arguments {
                replace_var_in_value_until_second_assignment(arg_value, to_replace, init_counts);
            }
        }
    }
}

pub fn replace_vars_in_value(value: &mut Value, to_replace: &HashMap<Variable, Value>) {
    match value {
        Value::Calculation { opcode: _, args } => {
            for arg_value in args {
                replace_vars_in_value(arg_value, to_replace);
            }
        }
        Value::Existing(var) => {
            if let Some(replacing_value) = to_replace.get(var) {
                // assert!(init_counts > 0); TODO why this reverts
                *value = replacing_value.clone();
            }
        }
        Value::Bytes(_) => (),
        Value::FunctionReturnedValue {
            label: _,
            arguments,
            return_index: _,
        } => {
            for arg_value in arguments {
                replace_vars_in_value(arg_value, to_replace);
            }
        }
    }
}
