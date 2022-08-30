use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;

use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::bytecode_reader::opcode::{Opcode, self};
use crate::evm::context::Context;
use crate::evm::expression::Expression;
use crate::evm::state::ExecutionState;
use crate::utils::{remove_values_where, usize_to_hex};

// A block terminates with:
const STOP_OPCODES: [Opcode; 6] = [
    opcode::RETURN,
    opcode::REVERT,
    opcode::SELFDESTRUCT,
    opcode::STOP,
    opcode::JUMP,
    opcode::JUMPI,
];
// ... or when the next opcode is JUMPDEST

#[derive(Clone)]
pub struct Block<'a> {
    pub code: &'a [Vopcode],
    pub initial_contexts: Vec<Context>,
}

impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode]) -> Self {
        return Block {
            code,
            initial_contexts: vec![],
        };
    }

    pub fn get_pc_start(&self) -> usize {
        return self.code[0].pc;
    }

    pub fn get_pc_end(&self) -> usize {
        return self.code[self.code.len() - 1].pc;
    }

    pub fn get_last_vopcode(&self) -> Vopcode {
        return self.code[self.code.len() - 1];
    }
    pub fn get_first_vopcode(&self) -> Vopcode {
        return self.code[0];
    }

    pub fn is_jumpable_from(&self, vopcode: Vopcode) -> bool {
        match vopcode.opcode {
            opcode::JUMP => self.code[0].opcode == opcode::JUMPDEST,
            opcode::JUMPI => {
                self.code[0].opcode == opcode::JUMPDEST
                    || vopcode.pc + 1 == self.get_first_vopcode().pc
            }

            _ => {
                assert!(!STOP_OPCODES.contains(&vopcode.opcode));
                self.get_first_vopcode().opcode == opcode::JUMPDEST
                    && vopcode.get_next_pc() == Some(self.get_first_vopcode().pc)
            }
        }
    }

    pub fn get_next_jump_dests(&self, final_state: &ExecutionState) -> Vec<usize> {
        let mut next_jump_dests: Vec<usize> = vec![];
        match final_state {
            ExecutionState::JUMP(Expression::VALUE(jump_dest)) => {
                next_jump_dests.push(jump_dest.as_usize());
            }
            ExecutionState::JUMPI(dest_expr, _) => {
                if let Expression::VALUE(jump_dest) = dest_expr {
                    next_jump_dests.push(jump_dest.as_usize());
                }
                next_jump_dests.push(self.get_pc_end() + 1); // TODO handle when JUMPI is the last opcode of the whole bytecode
            }

            ExecutionState::RUNNING => {
                next_jump_dests.push(self.get_last_vopcode().get_next_pc().unwrap())
            } // we are before a jump dest
            _ => (),
        }

        return next_jump_dests;
    }

    pub fn contains_initial_context(&self, initial_context: &Context) -> bool {
        return self
            .initial_contexts
            .iter()
            .any(|context: &Context| context.stack.equals_on_bytes(&initial_context.stack));
    }

    pub fn add_initial_context(&mut self, initial_context: Context) -> (Context, Vec<usize>) {
        assert!(!self.contains_initial_context(&initial_context));
        let final_context: Context = initial_context.run(self.code);
        self.initial_contexts.push(initial_context);
        let next_jump_dests: Vec<usize> = self.get_next_jump_dests(&final_context.state);
        return (final_context, next_jump_dests);
    }
}

impl<'a> Debug for Block<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("pc_start", &usize_to_hex(self.get_pc_start()))
            .field("pc_end", &usize_to_hex(self.get_pc_end()))
            .finish()
    }
}

pub struct BlockSet<'a> {
    blocks: HashMap<usize, Block<'a>>,    // pc_start => block
    children: HashMap<usize, Vec<usize>>, // pc_start => {all the children's pc_start}
}

impl<'a> BlockSet<'a> {
    pub fn contains_block_at(&self, pc: usize) -> bool {
        return self.blocks.contains_key(&pc);
    }

    pub fn get_blocks(&self) -> std::collections::hash_map::Values<'_, usize, Block<'_>> {
        return self.blocks.values();
    }

    pub fn get_pc_end_of_block(&self, pc_start: usize) -> usize {
        return self.blocks[&pc_start].get_pc_end();
    }

    pub fn get_all_pc_starts(&self) -> Vec<usize> {
        return self.blocks.keys().cloned().collect_vec();
    }

    pub fn insert_new_block(&mut self, block: Block<'a>) {
        self.children.insert(block.get_pc_start(), Vec::new());
        self.blocks.insert(block.get_pc_start(), block);
    }
    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = vec![]; // (pc_start origin, pc_start dest)
        for (pc_start_origin, children) in &self.children {
            for pc_start_dest in children {
                edges.push((*pc_start_origin, *pc_start_dest));
            }
        }
        return edges;
    }

    pub fn new(bytecode: &'a Bytecode) -> Self {
        let mut block_set: BlockSet = BlockSet {
            blocks: HashMap::new(),
            children: HashMap::new(),
        };
        block_set.find_blocks(bytecode);
        block_set.connect_from(Context::new(), 0);
        return block_set;
    }

    fn find_blocks(&mut self, bytecode: &'a Bytecode) {
        let mut pc_start: Option<usize> = Some(0);

        let mut previous_opcode: Option<Opcode> = None;
        for vopcode in bytecode.iter(0, bytecode.get_last_pc()) {
            let opcode: Opcode = vopcode.opcode;
            let pc: usize = vopcode.pc;
            println!("{}", vopcode);
            let next_opcode: Option<Opcode> = if let Some(next_pc) = vopcode.get_next_pc() {
                Some(bytecode.get_vopcode_at(next_pc).opcode)
            } else {
                None
            };
            if pc_start == None
                && (opcode == opcode::JUMPDEST || previous_opcode == Some(opcode::JUMPI))
            {
                // start a new block
                pc_start = Some(pc);
            }
            if pc_start != None
                && (STOP_OPCODES.contains(&opcode) || next_opcode == Some(opcode::JUMPDEST))
                || vopcode.is_last
            {
                // end block
                self.insert_new_block(Block::new(bytecode.slice_code(pc_start.unwrap(), pc)));
                pc_start = None;
            }
            previous_opcode = Some(opcode);
        }
    }

    fn connect_from(&mut self, initial_context: Context, pc_start: usize) {
        let block: &mut Block = self.blocks.get_mut(&pc_start).unwrap();
        if !block.contains_initial_context(&initial_context) {
            let (final_stack, mut next_jump_dests): (Context, Vec<usize>) =
                block.add_initial_context(initial_context);
            let last_vopcode: Vopcode = block.get_last_vopcode();
            remove_values_where(&mut next_jump_dests, |jump_dest: &usize| {
                !self.contains_block_at(*jump_dest)
                    || !self.blocks[jump_dest].is_jumpable_from(last_vopcode)
            }); // remove potential invalid jumps
            for jump_dest in next_jump_dests {
                self.children.get_mut(&pc_start).unwrap().push(jump_dest);
                self.connect_from(final_stack.clean_state(), jump_dest);
            }
        }
    }
}
