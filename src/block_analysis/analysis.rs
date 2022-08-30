use crate::jump_graph::graph::{Block, Context, ExecutionState, Opcode, Vopcode};

pub fn get_stack_delta(block: &Block<'a>) -> usize {
    let mut delta: usize = 0;
    for vopcode in block.code {
        delta += vopcode.opcode.delta();
    }
    return delta;
}