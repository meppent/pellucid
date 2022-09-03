use std::collections::HashMap;
use std::fmt::Debug;

use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::vopcode::Vopcode;
use crate::evm::context::Context;
use crate::evm::optional_push::Expression;
use crate::evm::state::ExecutionState;
use crate::utils::usize_to_hex;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Position {
    INITIAL,
    FINAL,
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
    pub delta: isize,
    pub delta_min: isize,
}

/* The concept of Block:
                                              index 0                    index 1
                                                 |                          |
                                                 V                          V

            Position::INITIAL    ―>     ┌―――――――――――――――――――┐     ┌―――――――――――――――――――┐
                                        |  initial context  |     |  initial context  |   ...
                ┌――┬―――――――――――――┬―――――――――――――――――――――――――――――――――――――――――――――――――――――――――┐
  pc_start -->  |b6|  JUMPDEST   |                                                         |
                |b7|  DUP        |                |                         |              |
                |b8|  MLOAD      |                |                         |              |
                |..|  ...        |                |                         |              |
                |  |             |                |                         |              |
                |  |             |                |                         |              |
                |  |             |                V                         V              |
  pc_end   -->  |  |             |                                                         |
                └――┴―――――――――――――┴―――――――――――――――――――――――――――――――――――――――――――――――――――――――――┘
                                        |  final context    |     |  final context    |  ...
            Position::FINAL    -->      └―――――――――――――――――――┘     └―――――――――――――――――――┘
*/
impl<'a> Block<'a> {
    pub fn new(code: &'a [Vopcode], delta: isize, delta_min: isize) -> Self {
        return Block {
            code,
            contexts: vec![],
            delta,
            delta_min,
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

    pub fn get_input_size(&self) -> isize {
        return self.delta_min;
    }

    pub fn get_output_size(&self) -> isize {
        return self.delta - self.delta_min;
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
                assert!(!vopcode.opcode.is_exiting() && !vopcode.opcode.is_jump());
                self.get_first_vopcode().opcode == Opcode::JUMPDEST
                    && vopcode.get_next_pc() == Some(self.get_first_vopcode().pc)
            }
        }
    }

    pub fn get_next_dests(&self, final_state: &ExecutionState) -> Vec<usize> {
        let mut next_jump_dests: Vec<usize> = vec![];
        match final_state {
            ExecutionState::JUMP(Expression::PUSH(jump_dest)) => {
                next_jump_dests.push(jump_dest.as_usize());
            }
            ExecutionState::JUMPI(dest_expr, _) => {
                if let Expression::PUSH(jump_dest) = dest_expr {
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
            .map(|tip_pair: &HashMap<Position, Context>| tip_pair[&Position::INITIAL].clone())
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
            (Position::INITIAL, initial_context),
            (Position::FINAL, final_context.clone()),
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
