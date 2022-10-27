use crate::{
    execution_flow::execution_flow::{AccessContent, GetFunctionLabel, Scope},
    tools::utils::{concat_to_str, shift_text},
};

use super::{flow_with_opcodes::ExecutionFlowWithOpcodes, scopes_with_opcodes::OpcodeScope};

impl ToString for ExecutionFlowWithOpcodes {
    fn to_string(&self) -> String {
        let mut res: String = String::new();
        res += &format!(
            "main()\n{}",
            shift_text(&concat_to_str(self.get_main_function().get_content(), "\n"))
        );

        for function in self.get_secondary_functions() {
            res += "\n";
            res += &format!(
                "function_{}({}, {:?})\n",
                function.get_label(),
                function.n_inputs,
                function.n_outputs
            );
            res += &shift_text(&concat_to_str(&function.content, "\n"));
        }

        return res;
    }
}

impl ToString for OpcodeScope {
    fn to_string(&self) -> String {
        match self {
            Scope::Instructions(instructions_with_opcodes) => {
                return concat_to_str(&instructions_with_opcodes.code, "\n");
            }
            Scope::FunctionCall(function_call_with_opcode) => {
                return format!("function_{}(", function_call_with_opcode.get_label());
            }
            Scope::FunctionReturn(function_return_with_opcodes) => {
                return format!("return_{}", function_return_with_opcodes.label);
            }
            Scope::Loop { label } => return format!("loop_{}", label),
            Scope::LoopContinue { label } => return format!("continue_{}", label),
            Scope::Condition {
                instructions_if_true,
                instructions_if_false,
            } => {
                let mut res: String = String::new();
                res += "{";
                res += &shift_text(&concat_to_str(instructions_if_true, "\n"));
                res += "}\n{";
                res += &shift_text(&concat_to_str(instructions_if_false, "\n"));
                res += "}";
                return res;
            }
            Scope::Panic => return "Panic".to_owned(),
            Scope::Empty => return "".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::{
        bytecode_reader::bytecode::Bytecode,
        create_graph::graph::Graph,
        detect_cycles::acyclic_graph::AcyclicGraph,
        execution_flow::{
            skeleton::skeleton::Skeleton,
            with_opcodes::flow_with_opcodes::convert_skeleton_to_execution_flow,
        },
    };

    use super::*;

    #[test]
    pub fn test_display_flow_with_opcodes() {
        //let path: String = String::from("./contracts/complex/bytecode.txt");
        let path: String = String::from("./contracts/simple/contract_0/bytecode.txt");
        println!("Loading {}", path);
        let bytecode_string: String = fs::read_to_string(path).expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let mut graph: Graph = Graph::from(&bytecode);
        let mut a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);

        let skeleton: Skeleton = Skeleton::build(&mut a_graph);
        let execution_flow_with_opcodes: ExecutionFlowWithOpcodes =
            convert_skeleton_to_execution_flow(&skeleton);
        // tools::utils::write_file(
        //     "flow_of_opcodes.txt",
        //     &execution_flow_with_opcodes.to_string(),
        // );
        //println!("{}", execution_flow_with_opcodes.to_string());
        let _ = execution_flow_with_opcodes;
    }
}
