pub mod bytecode_reader;
pub mod create_blocks;
pub mod create_graph;
pub mod detect_cycles;
pub mod detect_functions;
pub mod execution_flow;
pub mod research_and_development;
pub mod tools;
use crate::research_and_development::draw_graph_v1::draw_graph_to_str;
use crate::tools::utils::file_exists;
use bytecode_reader::bytecode::Bytecode;
use bytecode_reader::metadata;
use create_graph::graph::Graph;
use detect_cycles::acyclic_graph::AcyclicGraph;
use execution_flow::skeleton::skeleton::Skeleton;
use execution_flow::with_opcodes::flow_with_opcodes::{
    convert_skeleton_to_execution_flow, ExecutionFlowWithOpcodes,
};
use execution_flow::with_variables::flow_with_vars::{
    convert_opcode_flow_to_var_flow, ExecutionFlowWithVars,
};
use tools::utils::write_file;

fn main() {
    let info_msg: String = String::from(
        "Expecting 2 args: ./pellucid [mode] [bytecode]

modes: - decompile        (result will be stored in 'decompiled.txt')
       - disassemble      (result will be stored in 'opcodes.txt')
       - metadata         (result will be displayed in terminal)
       - graph            (result will be stored in 'graph.txt')

bytecode: the runtime bytecode of the contract (hex)",
    );

    if std::env::args().len() != 3 {
        println!("{}", info_msg);
        return;
    }

    let mode: String = std::env::args().nth(1).unwrap();
    let raw_bytecode: String = std::env::args().nth(2).unwrap();
    match mode.as_str() {
        "decompile" => {
            if file_exists("./decompiled.txt") {
                println!("./decompiled.txt already exists, please remove it.");
                return;
            }
            if let Some(bytecode) = Bytecode::from(&raw_bytecode) {
                print!("\nBuilding graph...");
                let mut graph: Graph = Graph::from(&bytecode);
                print!("\tDone\nDetecting loops...");
                let mut a_graph: AcyclicGraph = AcyclicGraph::from(&mut graph);
                print!("\tDone\nDetecting internal function...");
                let skeleton: Skeleton = Skeleton::build(&mut a_graph);
                print!("\tDone\nBuilding execution flow...");
                let execution_flow_with_opcodes: ExecutionFlowWithOpcodes =
                    convert_skeleton_to_execution_flow(&skeleton);
                let execution_flow_with_vars: ExecutionFlowWithVars =
                    convert_opcode_flow_to_var_flow(&execution_flow_with_opcodes);
                write_file("decompiled.txt", &execution_flow_with_vars.to_string());
                print!("\tDone\n");
                println!("Success, (pseudo) source code stored in ./decompiled.txt");
            } else {
                println!("Cannot read bytecode, please ensure it's hexadecimal.");
            }
        }
        "disassemble" => {
            if file_exists("./opcodes.txt") {
                println!("./opcodes.txt already exists, please remove it.");
                return;
            }
            if let Some(bytecode) = Bytecode::from(&raw_bytecode) {
                write_file("./opcodes.txt", &bytecode.to_string());
                println!("Success, opcodes stored in ./opcodes.txt");
            } else {
                println!("Cannot read bytecode, please ensure it's hexadecimal.");
            }
        }
        "metadata" => {
            let metadata = metadata::get_metadata(&raw_bytecode);
            let metadata_length = metadata.len();
            println!("{} metadata found.", metadata_length);
            for m in metadata {
                println!("  {}", m);
            }
        }
        "graph" => {
            if file_exists("./graph.txt") {
                println!("./graph.txt already exists, please remove it.");
                return;
            }
            if let Some(bytecode) = Bytecode::from(&raw_bytecode) {
                println!("\nDrawing graph... (May be long when there are a lot of loops)");
                let graph_str: String = draw_graph_to_str(&bytecode);
                write_file("graph.txt", &graph_str);
                println!("Success, graph stored in ./graph.txt");
            } else {
                println!("Cannot read bytecode, please ensure it's hexadecimal.");
            }
        }
        _ => {
            println!("{}", info_msg);
        }
    }
}
