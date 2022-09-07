use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    bytecode_reader::{bytecode::Bytecode, opcode::Opcode, vopcode::{Vopcode, self}},
    create_graph::block::{Block, BlockRef},
};

use super::symbolic_block::SymbolicBlock;

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
                    block_locations.push((pc_start_, pc)); // TODO if bytecode start by jumpdest
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

// TODO put directly in BlockRef implem
pub fn new_block_ref<'a>(code: &'a [Vopcode])->BlockRef<'a>{
    let mut symbolic_block: SymbolicBlock = SymbolicBlock::new();
    for vopcode in code{
        symbolic_block.apply_vopcode(vopcode);
    } 
    return BlockRef::new(code, Rc::new(symbolic_block));
}

pub fn find_blocks<'a>(bytecode: &'a Bytecode) -> HashMap<usize, BlockRef<'a>> {
    let mut blocks: HashMap<usize, BlockRef<'a>> = HashMap::new();
    for (pc_start, pc_end) in find_block_locations(bytecode){
        blocks.insert(pc_start, new_block_ref(bytecode.slice_code(pc_start, pc_end)));
    }
    return blocks;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{bytecode_reader::vopcode::Vopcode, tools::utils::read_file};
    use itertools::Itertools;
    use std::{fs, rc::Rc};

    #[test]
    pub fn test_symbolic_blocks() {
        let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string);
        let mut blocks: HashMap<usize, BlockRef> = find_blocks(&bytecode);
        let mut symbolic_blocks: HashMap<usize, Rc<SymbolicBlock>> = HashMap::new();
        let pc_starts: Vec<usize> = blocks.keys().into_iter().map(|pc| *pc).collect_vec();
        for pc_start in &pc_starts {
            symbolic_blocks.insert(
                *pc_start,
                blocks.remove(pc_start).unwrap().get_symbolic_block(),
            );
        }
        let target_symbolic_blocks: HashMap<usize, Rc<SymbolicBlock>> = serde_json::from_str(
            &read_file("./assets/contracts/simple_contract/symbolic_blocks.json"),
        )
        .unwrap();
        assert!(target_symbolic_blocks == symbolic_blocks);

        // to overwrite the dest json:
        //write_file("./assets/contracts/simple_contract/symbolic_blocks.json", &serde_json::to_string(&symbolic_blocks).unwrap());
    }

    #[test]
    pub fn test_block_bytecodes() {
        let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode: Bytecode = Bytecode::from(&bytecode_string);
        let mut blocks: HashMap<usize, BlockRef> = find_blocks(&bytecode);
        let mut block_bytecodes: HashMap<usize, Vec<Vopcode>> = HashMap::new();
        let pc_starts: Vec<usize> = blocks.keys().into_iter().map(|pc| *pc).collect_vec();
        for pc_start in &pc_starts {
            block_bytecodes.insert(
                *pc_start,
                blocks.remove(pc_start).unwrap().get_code().to_vec(),
            );
        }
        let target_block_bytecodes: HashMap<usize, Vec<Vopcode>> = serde_json::from_str(
            &read_file("./assets/contracts/simple_contract/block_bytecodes.json"),
        )
        .unwrap();
        assert!(target_block_bytecodes == block_bytecodes);

        //to overwrite the dest json:
        //write_file("./assets/contracts/simple_contract/block_bytecodes.json", &serde_json::to_string(&block_bytecodes).unwrap());
    }
}
