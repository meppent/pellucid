use itertools::Itertools;

use crate::{
    bytecode_reader::opcode::calculation_to_str,
    execution_flow::execution_flow::{AccessContent, GetFunctionLabel, Scope},
    tools::utils::{concat_to_str, shift_text, u256_to_hex},
};

use super::{
    flow_with_vars::ExecutionFlowWithVars,
    incorporate_variables::{Line, Value, Variable, VariablesStack},
    scopes_with_vars::VarScope,
};

impl ToString for ExecutionFlowWithVars {
    fn to_string(&self) -> String {
        let mut res: String = String::new();
        res += &format!(
            "function main() external {{\n{}}}",
            shift_text(&scopes_with_var_to_string(
                self.get_main_function().get_content().as_slice()
            ))
        );

        for function in self.get_secondary_functions() {
            res += &format!("\n\nfunction fn_{}(", function.get_label());
            res += &concat_to_str(&function.input_vars, ", ");
            res += ") internal";
            if function.n_outputs > 0 {
                res += " returns(";
                res += &concat_to_str(
                    &(0..function.n_outputs)
                        .map(|index| format!("r{}", index))
                        .collect_vec(),
                    ", ",
                );
                res += ")";
            }
            res += " {\n";
            res += &shift_text(&scopes_with_var_to_string(&function.content));
            res += "}";
        }

        return res;
    }
}

fn scopes_with_var_to_string(scopes: &[VarScope]) -> String {
    let mut res: String = String::new();
    for (scope_index, scope) in scopes.iter().enumerate() {
        if let Scope::Loop { label } = scope {
            res += &format!("begin loop_{} {{\n", label);
            res += &shift_text(&scopes_with_var_to_string(&scopes[scope_index + 1..]));
            res += "}";
            return res;
        } else {
            res += &scope.to_string();
            if scope_index != scopes.len() - 1 && !scope.should_be_followed_by_condition_scope() {
                res += "\n";
            }
        }
    }
    return res;
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        return format!("var_{}", self.alias);
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        match self {
            Line::Assignement {
                receiving_var,
                assigned_value: assigned,
            } => {
                let mut res: String = String::new();
                if let Some(_receiving_var) = receiving_var {
                    res += &format!("{} = ", _receiving_var.to_string());
                }
                res += &assigned.to_string();
                return res;
            }
            Line::If { condition } => format!("if {}", condition.to_string()),
            Line::Empty => "".to_owned(),
        }
    }
}
impl ToString for VarScope {
    fn to_string(&self) -> String {
        match self {
            Scope::Instructions(instructions_with_vars) => {
                return concat_to_str(&instructions_with_vars.lines, "\n");
            }
            Scope::FunctionCall(function_call_with_vars) => {
                let mut res: String = String::new();
                if function_call_with_vars.results.len() > 0 {
                    if function_call_with_vars.results.len() > 1 {
                        res += "(";
                    }
                    res += &concat_to_str(&function_call_with_vars.results, ", ");
                    if function_call_with_vars.results.len() > 1 {
                        res += ")";
                    }
                    res += " = ";
                }
                res += &format!("fn_{}(", function_call_with_vars.get_label());
                res += &concat_to_str(&function_call_with_vars.arguments, ", ");
                res += ")";
                return res;
            }
            Scope::FunctionReturn(function_return_with_vars) => {
                if function_return_with_vars.returned_values.is_empty() {
                    return format!(
                        "// end of function {}",
                        function_return_with_vars.get_label()
                    );
                }
                let mut res: String = String::new();
                res += "(";
                res += &concat_to_str(
                    &(0..function_return_with_vars.returned_values.len())
                        .map(|index| format!("r{}", index))
                        .collect_vec(),
                    ", ",
                );
                res += ") = (";
                res += &concat_to_str(&function_return_with_vars.returned_values, ", ");
                res += ")";

                return res;
            }
            Scope::Loop { label } => return format!("begin loop_{}", label),
            Scope::LoopContinue { label } => return format!("continue loop_{}", label),
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                let mut res: String = String::new();
                res += " {\n";
                res += &shift_text(&scopes_with_var_to_string(instructions_if_true));
                res += "}\nelse {\n";
                res += &shift_text(&scopes_with_var_to_string(instructions_if_false));
                res += "}";
                return res;
            }
            Scope::Panic => return "// Panic".to_owned(),
            Scope::Empty => return "".to_owned(),
        }
    }
}

impl ToString for VariablesStack {
    fn to_string(&self) -> String {
        return format!(
            "[{}]",
            concat_to_str(self._get_data() as &Vec<Variable>, ", ")
        );
    }
}

fn value_to_string(value: &Value, is_nested: bool) -> String {
    match value {
        Value::Calculation { opcode, args } => {
            let mut res: String =
                calculation_to_str(*opcode, |arg_index: usize, is_nested: bool| {
                    value_to_string(&args[arg_index], is_nested)
                });
            if is_nested {
                res.insert(0, '(');
                res += ")";
            }
            return res;
        }
        Value::Existing(existing_var) => return existing_var.to_string(),
        Value::Bytes(bytes) => return u256_to_hex(*bytes),
        Value::FunctionReturnedValue {
            label,
            arguments,
            return_index,
        } => {
            let mut res = String::new();
            res += &format!("function_{}(", label);
            res += &concat_to_str(arguments, ", ");
            res += ")";
            res += &format!(".{}", return_index);
            return res;
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        return value_to_string(self, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bytecode_reader::bytecode::Bytecode,
        create_graph::graph::Graph,
        detect_cycles::acyclic_graph::AcyclicGraph,
        execution_flow::{
            skeleton::skeleton::Skeleton,
            with_opcodes::flow_with_opcodes::{
                convert_skeleton_to_execution_flow, ExecutionFlowWithOpcodes,
            },
            with_variables::flow_with_vars::convert_opcode_flow_to_var_flow,
        },
    };
    use std::fs;

    #[test]
    pub fn test_display_flow_with_vars() {
        let path: String = String::from("./contracts/simple/contract_0/bytecode.txt");
        //let path: String = String::from("./contracts/complex/bytecode.txt");
        println!("Loading {}", path);
        let bytecode_string: String = fs::read_to_string(path).expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let mut graph: Graph = Graph::from(&bytecode);
        let mut a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);
        let skeleton: Skeleton = Skeleton::build(&mut a_graph);
        let execution_flow_with_opcodes: ExecutionFlowWithOpcodes =
            convert_skeleton_to_execution_flow(&skeleton);
        let execution_flow_with_vars: ExecutionFlowWithVars =
            convert_opcode_flow_to_var_flow(&execution_flow_with_opcodes);
        crate::tools::utils::write_file("decompiled.txt", &execution_flow_with_vars.to_string());
        let _ = execution_flow_with_vars;
    }
}
