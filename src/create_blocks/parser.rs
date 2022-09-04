use std::{collections::HashMap, cell::RefCell};

use crate::{
    bytecode_reader::{bytecode::Bytecode, opcode::Opcode},
    create_graph::block::Block,
};

use super::symbolic_block::SymbolicBlock;

fn find_blocks<'a>(bytecode: &'a Bytecode) -> HashMap<usize, Block<'a>> {
    let mut blocks: HashMap<usize, Block<'a>> = HashMap::new();

    let mut vopcode_iterator = bytecode.iter(0, bytecode.get_last_pc()).peekable();

    // Invariant: When we enter this loop, we are at the begning of a block
    'new_block: while let Some(vopcode_start) = vopcode_iterator.next() {
        let mut current_vopcode = vopcode_start;
        let symbolic_block: RefCell<SymbolicBlock> = RefCell::new(SymbolicBlock::new());

        let mut insert_block = || {
            let pc_start = vopcode_start.pc;
            let mut block_to_add = Block::new(bytecode.slice_code(pc_start, current_vopcode.pc));
            block_to_add.attach_symbolic_block(symbolic_block.take());
            blocks.insert(
                pc_start,
                block_to_add
            );
        };

        // Invariant: When we exit this loop, it is at the end of a block
        'same_block: loop {
            // We are in a block, we modify the symbolic stack, and we search for the end of the block
            RefCell::borrow_mut(&symbolic_block).apply_vopcode(current_vopcode);

            // It's the end of a block, and there is no block after
            if current_vopcode.is_last || current_vopcode.opcode.is_exiting() || current_vopcode.opcode == Opcode::JUMP {
                insert_block();
                break 'same_block;
            // It's the end of a block, and there is a new block after
            } else if current_vopcode.opcode == Opcode::JUMPI {
                insert_block();
                continue 'new_block;
            } 

            match vopcode_iterator.peek() {
                Some(next_vopcode) if next_vopcode.opcode == Opcode::JUMPDEST => {
                    insert_block();
                    continue 'new_block;
                },
                _ => (),
            }

            match vopcode_iterator.next() {
                Some(vopcode) => {
                    current_vopcode = vopcode;
                    continue 'same_block;
                }
                None => break 'new_block,
            }
        }

        // Invariant: When we are in this loop, we are not in a block
        'no_block: loop {
            match vopcode_iterator.peek() {
                Some(vopcode)  if vopcode.opcode == Opcode::JUMPDEST=> break 'no_block,
                None => break 'no_block,
                _ => {
                    vopcode_iterator.next();
                },
            }
        }
        
    }
    return blocks;
}


#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    pub fn test_parser(){
        let bytecode_string: String =
                fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                    .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string);
        //let vopcodes = bytecode.slice_code(16, 25);
        //dbg!(vopcodes);
        //let block = SymbolicBlock::from(vopcodes);
        let blocks = find_blocks(&bytecode);
        dbg!(blocks);
        
    }
}
