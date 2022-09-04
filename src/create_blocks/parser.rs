use std::{collections::HashMap, cell::RefCell};

use crate::{
    bytecode_reader::{bytecode::Bytecode, opcode::Opcode},
    create_graph::block::Block,
};

use super::symbolic_block::SymbolicBlock;

fn find_blocks<'a>(bytecode: &'a Bytecode) -> HashMap<usize, Block<'a>> {
    let mut blocks: HashMap<usize, Block<'a>> = HashMap::new();

    let mut vopcode_iterator = bytecode.iter(0, bytecode.get_last_pc()).peekable();

    // Invariant: When we enter this loop, we are at the beginning of a block
    'new_block: while let Some(vopcode_start) = vopcode_iterator.next() {
        let mut current_vopcode = vopcode_start;
        let symbolic_block: RefCell<SymbolicBlock> = RefCell::new(SymbolicBlock::new());

        let mut insert_block = |pc_end: usize| {
            let pc_start = vopcode_start.pc;
            let mut block_to_add = Block::new(bytecode.slice_code(pc_start, pc_end));
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

            // It's the end of the block, and there is no block after
            if vopcode_iterator.peek() == None || current_vopcode.opcode.is_exiting() || current_vopcode.opcode == Opcode::JUMP {
                insert_block(current_vopcode.pc);
                break 'same_block;
            // It's the end of the block, and there is a new block after
            } else if current_vopcode.opcode == Opcode::JUMPI {
                insert_block(current_vopcode.pc);
                continue 'new_block;
            }

            match vopcode_iterator.peek() {
                // It's the end of the block, and there is a new block after
                Some(next_vopcode) if next_vopcode.opcode == Opcode::JUMPDEST => {
                    insert_block(current_vopcode.pc);
                    continue 'new_block;
                },
                _ => (),
            }

            match vopcode_iterator.next() {
                // It's not the end of the block
                Some(vopcode) => {
                    current_vopcode = vopcode;
                    continue 'same_block;
                }
                // We already checked that the next one is not None
                _ => unreachable!()
            }
        }

        // Invariant: When we are in this loop, we are not in a block
        'no_block: loop {
            // We are not in a block, we search for a new block
            match vopcode_iterator.peek() {
                // There is a new block after
                Some(vopcode) if vopcode.opcode == Opcode::JUMPDEST=> break 'no_block,
                // There is no new block after
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
    use itertools::Itertools;
    use crate::{tools::utils::{read_file, write_file}, bytecode_reader::vopcode::Vopcode};
    use super::*;

    #[test]
    pub fn test_symbolic_blocks(){
        let bytecode_string: String =
                fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                    .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string);
        let mut blocks: HashMap<usize, Block> = find_blocks(&bytecode);
        let mut symbolic_blocks: HashMap<usize, SymbolicBlock> = HashMap::new();
        let pc_starts: Vec<usize> = blocks.keys().into_iter().map(|pc|*pc).collect_vec();
        for pc_start in &pc_starts{
            symbolic_blocks.insert(*pc_start, blocks.remove(pc_start).unwrap().symbolic_block);
        }
        let target_symbolic_blocks: HashMap<usize, SymbolicBlock> = serde_json::from_str(&read_file("./assets/contracts/simple_contract/symbolic_blocks.json")).unwrap();
        assert!(target_symbolic_blocks == symbolic_blocks);

        // to overwrite the dest json:
        //write_file("temp.json", &serde_json::to_string(&symbolic_blocks).unwrap());
    }

    #[test]
    pub fn test_block_bytecodes(){
        let bytecode_string: String =
                fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                    .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string);
        let mut blocks: HashMap<usize, Block> = find_blocks(&bytecode);
        let mut block_bytecodes: HashMap<usize, Vec<Vopcode>> = HashMap::new();
        let pc_starts: Vec<usize> = blocks.keys().into_iter().map(|pc|*pc).collect_vec();
        for pc_start in &pc_starts{
            block_bytecodes.insert(*pc_start, blocks.remove(pc_start).unwrap().code.to_vec());
        }
        let target_block_bytecodes: HashMap<usize, Vec<Vopcode>> = serde_json::from_str(&read_file("./assets/contracts/simple_contract/block_bytecodes.json")).unwrap();
        assert!(target_block_bytecodes == block_bytecodes);

        //to overwrite the dest json:
        //write_file("./assets/contracts/simple_contract/block_bytecodes.json", &serde_json::to_string(&block_bytecodes).unwrap());
    }

    
}
