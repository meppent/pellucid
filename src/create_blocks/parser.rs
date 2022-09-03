use std::collections::HashMap;

use crate::bytecode_reader::{bytecode::Bytecode, opcode::Opcode};

fn find_blocks(bytecode: &Bytecode) {
    let blocks: HashMap<usize, Block<'a>> = HashMap::new();
    let mut pc_start: Option<usize> = Some(0);
    let mut previous_pc: usize = 0;
    let mut delta: isize = 0;
    let mut delta_min: isize = 0;

    for vopcode in bytecode.iter(0, bytecode.get_last_pc()) {
        let opcode: Opcode = vopcode.opcode;
        let pc: usize = vopcode.pc;

        match pc_start {
            Some(pc_start_) => {
                // we are in a block, we search for the end

                // correct because the JUMPDEST delta is 0, todo: find a better way to do this
                delta += opcode.delta();
                delta_min = delta_min.min(delta);

                if vopcode.is_last || opcode.is_exiting() || opcode == Opcode::JUMP {
                    self.insert_connected_block(
                        bytecode,
                        pc_start_,
                        pc,
                        &mut delta,
                        &mut delta_min,
                    );
                    pc_start = None;
                } else if opcode == Opcode::JUMPI {
                    self.insert_connected_block(
                        bytecode,
                        pc_start_,
                        pc,
                        &mut delta,
                        &mut delta_min,
                    );
                    pc_start = Some(pc + 1);
                } else if opcode == Opcode::JUMPDEST {
                    self.insert_connected_block(
                        bytecode,
                        pc_start_,
                        previous_pc,
                        &mut delta,
                        &mut delta_min,
                    );
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
        previous_pc = pc;
    }
}
