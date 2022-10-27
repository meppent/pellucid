use itertools::Itertools;

use crate::{
    create_blocks::symbolic_expression::{Effect, StackExpression, SymbolicExpression},
    create_graph::block::Block,
    tools::utils::usize_to_hex,
};

pub trait DisplaySkeletonScope {
    fn scope_to_string(&self) -> String;
    fn get_alias(&self) -> Option<String>;
}

impl<'a> DisplaySkeletonScope for Block<'a> {
    fn scope_to_string(&self) -> String {
        let mut res = format!("Block {}", usize_to_hex(self.get_pc_start()));
        let symbolic_block = self.clone_symbolic_block();
        for effect in &symbolic_block.effects {
            res += &format!("\n -> {}", effect.to_string());
        }
        return res;
    }
    fn get_alias(&self) -> Option<String> {
        return None;
    }
}

impl ToString for StackExpression {
    fn to_string(&self) -> String {
        match self {
            StackExpression::BYTES(value) => format!("\"{}\"", value),
            StackExpression::COMPOSE(opcode, symbolic_expressions) => {
                let mut res: String = opcode.to_string();
                if symbolic_expressions.len() > 0 {
                    res += "[";
                    res += &symbolic_expressions
                        .iter()
                        .map(|expr| expr.to_string())
                        .join(", ");
                    res += "]";
                }
                res
            }
            StackExpression::ARG(index) => format!("input_stack[{}]", index),
        }
    }
}

impl ToString for SymbolicExpression {
    fn to_string(&self) -> String {
        // for now we do not consider its effect here
        return self.stack_expression.to_string();
    }
}

impl ToString for Effect {
    fn to_string(&self) -> String {
        let mut res = self.opcode.to_string();
        if self.symbolic_expressions.len() > 0 {
            res += "[";
            res += &self
                .symbolic_expressions
                .iter()
                .map(|expr| expr.to_string())
                .join(", ");
            res += "]";
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bytecode_reader::bytecode::Bytecode,
        create_graph::graph::Graph,
        detect_cycles::acyclic_graph::AcyclicGraph,
        execution_flow::skeleton::{skeleton::Skeleton, skeleton_scopes::SkeletonScope},
    };
    use std::fs;

    fn instructions_to_string<'a>(instructions: &Vec<SkeletonScope<'a>>) -> String {
        let mut res: String = String::new();
        for instruction in instructions {
            if let Some(alias) = instruction.get_alias() {
                res.push_str(&alias);
            } else {
                res.push_str(&instruction.to_string());
            }
            res.push_str("\n");
        }

        return res;
    }

    pub fn skeleton_to_string<'a, 'b>(skeleton: &Skeleton<'a, 'b>) -> String {
        let mut res: String = String::new();
        res += &instructions_to_string(&skeleton.main_instructions);
        for junction_scope in skeleton.junctions.values() {
            res += &SkeletonScope::Junction(junction_scope.clone()).to_string();
        }
        for function_scope in skeleton.functions.values() {
            res += &SkeletonScope::Function(function_scope.clone()).to_string();
        }
        return res;
    }

    #[test]
    pub fn test_display_skeleton() {
        //let path: String = String::from("./contracts/complex/bytecode.txt");
        let path: String = String::from("./contracts/simple/contract_0/bytecode.txt");
        println!("Loading {}", path);
        let bytecode_string: String = fs::read_to_string(path).expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string).unwrap();
        let mut graph: Graph = Graph::from(&bytecode);
        let mut a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);

        let skeleton: Skeleton = Skeleton::build(&mut a_graph);
        let skeleton_str: String = skeleton_to_string(&skeleton);
       // tools::utils::write_file("skeleton.txt", &skeleton_str);
       let _ = skeleton_str;
    }
}
