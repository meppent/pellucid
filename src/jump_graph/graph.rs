use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use itertools::Itertools;

use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::{bytecode::Bytecode, vopcode::Vopcode};
use crate::evm::context::Context;
use crate::evm::expression::Expression;
use crate::evm::state::ExecutionState;
use crate::utils::{remove_values_where, usize_to_hex};

// A block terminates with:
const STOP_OPCODES: [Opcode; 6] = [
    Opcode::RETURN,
    Opcode::REVERT,
    Opcode::SELFDESTRUCT,
    Opcode::STOP,
    Opcode::JUMP,
    Opcode::JUMPI,
];

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Position {
    UP,
    DOWN,
}
// ... or when the next opcode is JUMPDEST
#[derive(Clone, Copy)]
pub struct Location {
    pub pc_start: usize,
    pub context_index: usize,
    pub position: Position,
}

#[derive(Clone)]
pub struct Block<'a> {
    pub code: &'a [Vopcode],
    pub contexts: Vec<HashMap<Position, Context>>,
}

impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode]) -> Self {
        return Block {
            code,
            contexts: vec![],
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
    pub fn get_n_initial_contexts(&self) -> usize {
        return self.contexts.len();
    }

    pub fn is_jumpable_from(&self, vopcode: Vopcode) -> bool {
        match vopcode.opcode {
            Opcode::JUMP => self.code[0].opcode == Opcode::JUMPDEST,
            Opcode::JUMPI => {
                self.code[0].opcode == Opcode::JUMPDEST
                    || vopcode.pc + 1 == self.get_first_vopcode().pc
            }
            _ => {
                assert!(!STOP_OPCODES.contains(&vopcode.opcode));
                self.get_first_vopcode().opcode == Opcode::JUMPDEST
                    && vopcode.get_next_pc() == Some(self.get_first_vopcode().pc)
            }
        }
    }

    pub fn get_next_dests(&self, final_state: &ExecutionState) -> Vec<usize> {
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
        return self.get_index_of_initial_context(initial_context) != None;
    }

    fn get_initial_contexts(&self) -> Vec<Context> {
        return self
            .contexts
            .iter()
            .map(|tip_pair: &HashMap<Position, Context>| tip_pair[&Position::UP].clone())
            .collect::<Vec<Context>>();
    }

    fn get_index_of_initial_context(&self, initial_context: &Context) -> Option<usize> {
        for (index, my_initial_context) in self.get_initial_contexts().iter().enumerate() {
            if initial_context
                .stack
                .equals_on_bytes(&my_initial_context.stack)
            {
                return Some(index);
            }
        }
        return None;
    }

    pub fn get_index_of_incomming_initial_context(&self, initial_context: &Context) -> usize {
        match self.get_index_of_initial_context(initial_context) {
            Some(initial_context_index) => initial_context_index,
            None => self.get_n_initial_contexts(),
        }
    }

    pub fn add_initial_context(
        &mut self,
        initial_context: Context,
    ) -> (usize, Context, Vec<usize>) {
        // return (index at which the context was inserted, final context, next destinations)
        assert!(!self.contains_initial_context(&initial_context));
        let final_context: Context = initial_context.run(self.code);
        self.contexts.push(HashMap::from([
            (Position::UP, initial_context),
            (Position::DOWN, final_context.clone()),
        ]));
        let next_dests: Vec<usize> = self.get_next_dests(&final_context.state);
        return (self.contexts.len() - 1, final_context, next_dests);
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

struct ConnectedBlock<'a> {
    pub block: Block<'a>,
    pub links: Vec<HashMap<Position, Vec<Location>>>, // context index => {UP => list of its parents locations, DOWN => list of its children locations}
}

impl<'a> ConnectedBlock<'a> {
    pub fn new(block: Block<'a>) -> Self {
        return ConnectedBlock {
            block,
            links: vec![],
        };
    }
}

pub struct BlockSet<'a> {
    connected_blocks: HashMap<usize, ConnectedBlock<'a>>, // pc_start => connected block
}

impl<'a> BlockSet<'a> {
    pub fn contains_block_at(&self, pc: usize) -> bool {
        return self.connected_blocks.contains_key(&pc);
    }

    pub fn get_connected_block_mut(&mut self, pc_start: usize) -> &mut ConnectedBlock<'a> {
        return &mut self.connected_blocks.get_mut(&pc_start).unwrap();
    }

    pub fn get_block_mut(&mut self, pc_start: usize) -> &mut Block<'a> {
        return &mut self.get_connected_block_mut().block;
    }

    // pub fn get_block_tip_mut(&mut self, location: Location) -> &mut BlockTip {
    //     return self
    //         .blocks
    //         .get_mut(&location.pc_start)
    //         .unwrap()
    //         .tips
    //         .get_mut(location.context_index)
    //         .unwrap()
    //         .get_mut(&location.position)
    //         .unwrap();
    // }

    pub fn get_blocks(&self) -> Vec<Block<'a>> {
        return self
            .connected_blocks
            .values()
            .map(|connected_block: &ConnectedBlock| connected_block.block.clone())
            .collect::<Vec<Block<'a>>>();
    }

    pub fn get_pc_end_of_block(&self, pc_start: usize) -> usize {
        return self.connected_blocks[&pc_start].block.get_pc_end();
    }

    pub fn get_all_pc_starts(&self) -> Vec<usize> {
        return self.connected_blocks.keys().cloned().collect_vec();
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let mut edges: Vec<(usize, usize)> = vec![]; // (pc_start origin, pc_start dest)
                                                     // for (_, block) in &self.blocks {
                                                     //     let origin_pc_start: usize = block.block.get_pc_start();
                                                     //     for tips in &block.tips {
                                                     //         for dest_location in &tips[&Position::DOWN].locations {
                                                     //             let dest_pc_start: usize = dest_location.pc_start;
                                                     //             edges.push((origin_pc_start, dest_pc_start));
                                                     //         }
                                                     //     }
                                                     // }
        return edges;
    }

    pub fn new(bytecode: &'a Bytecode) -> Self {
        let mut block_set: BlockSet = BlockSet {
            connected_blocks: HashMap::new(),
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
            let next_opcode: Option<Opcode> = if let Some(next_pc) = vopcode.get_next_pc() {
                Some(bytecode.get_vopcode_at(next_pc).opcode)
            } else {
                None
            };
            if pc_start == None
                && (opcode == Opcode::JUMPDEST || previous_opcode == Some(Opcode::JUMPI))
            {
                // start a new block
                pc_start = Some(pc);
            }
            if pc_start != None
                && (STOP_OPCODES.contains(&opcode) || next_opcode == Some(Opcode::JUMPDEST))
                || vopcode.is_last
            {
                // end block
                self.connected_blocks.insert(
                    pc_start.unwrap(),
                    ConnectedBlock::new(Block::new(bytecode.slice_code(pc_start.unwrap(), pc))),
                );
                pc_start = None;
            }
            previous_opcode = Some(opcode);
        }
    }

    fn connect(&mut self, origin_location: Location, dest_location: Location){
        .links.get_mut(index)
    }

    fn connect_from(&mut self, initial_context: Context, pc_start: usize) {
        dbg!("connect_from");
        let block: &mut Block = self.get_block_mut(pc_start);
        if !block.contains_initial_context(&initial_context) {
            let (origin_context_index, final_context, mut next_dests): (
                usize,
                Context,
                Vec<usize>,
            ) = block.add_initial_context(initial_context);
            let last_vopcode: Vopcode = block.get_last_vopcode();
            remove_values_where(&mut next_dests, |jump_dest: &usize| {
                !self.contains_block_at(*jump_dest)
                    || !self.connected_blocks[jump_dest].block.is_jumpable_from(last_vopcode)
            }); // remove potential invalid destinations
            let dest_initial_context: Context = final_context.clean_state();
            let origin_location: Location = Location {
                pc_start,
                context_index: origin_context_index,
                position: Position::DOWN,
            };
            for next_dest in next_dests {
                let dest_block: &mut Block = self.get_block_mut(next_dest);
                let dest_context_index: usize =
                    dest_block.get_index_of_incomming_initial_context(&dest_initial_context);
                let next_location: Location = Location {
                    pc_start: next_dest,
                    context_index: dest_context_index,
                    position: Position::UP,
                };
                //dest_block.connect_from(initial_context, pc_start)
                self.connect_from(dest_initial_context.clone(), next_dest);
            }
        }
    }
}
