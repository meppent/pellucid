use std::collections::HashMap;

use crate::{
    bytecode_reader::{bytecode::Bytecode, opcode::Opcode},
    create_graph::block::Block,
};

fn find_block_locations(bytecode: &Bytecode) -> Vec<(usize, usize)> {
    let mut block_locations: Vec<(usize, usize)> = Vec::new(); // (pc_start, pc_end)
    let mut pc_start: Option<usize> = Some(0);

    for vopcode in bytecode.iter(0, bytecode.get_last_pc()) {
        let opcode: Opcode = vopcode.opcode;
        let pc: usize = vopcode.pc;
        let is_last_vopcode: bool = pc == bytecode.get_last_pc();

        match pc_start {
            Some(pc_start_) => {
                // we already are in a block, we search for the end
                if is_last_vopcode || opcode.is_exiting() || opcode == Opcode::JUMP {
                    block_locations.push((pc_start_, pc));
                    pc_start = None;
                } else if opcode == Opcode::JUMPI {
                    block_locations.push((pc_start_, pc));
                    pc_start = Some(pc + 1);
                } else if opcode == Opcode::JUMPDEST {
                    block_locations.push((
                        pc_start_,
                        if let Some(prev_pc) = bytecode.get_previous_pc(pc) {
                            prev_pc
                        } else {
                            pc_start_ // rare case of a JUMPDEST at pc 0
                        },
                    ));
                    pc_start = Some(pc);
                }
            }
            None => {
                // we are not in a block, we search for a new block
                if opcode == Opcode::JUMPDEST {
                    pc_start = Some(pc);
                }
            }
        };
    }
    return block_locations;
}

pub fn find_blocks<'a>(bytecode: &'a Bytecode) -> HashMap<usize, Block<'a>> {
    let mut blocks: HashMap<usize, Block<'a>> = HashMap::new();
    for (pc_start, pc_end) in find_block_locations(bytecode) {
        blocks.insert(
            pc_start,
            Block::new(bytecode.slice_code(pc_start, pc_end), None),
        );
    }
    return blocks;
}
