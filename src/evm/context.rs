use super::expression::Expression;
use super::stack::Stack;
use super::state::ExecutionState;
use crate::bytecode_reader::{bytecode::Vopcode, opcode::Opcode};
use core::fmt::Debug;
use std::vec;

#[derive(Clone, Debug)]
pub struct Context {
    pub stack: Stack,
    pub state: ExecutionState,
}

impl Context {
    pub fn new() -> Self {
        return Context {
            stack: Stack::new(),
            state: ExecutionState::RUNNING,
        };
    }

    pub fn clean_state(&self) -> Self {
        return Context {
            stack: self.stack.clone(),
            state: ExecutionState::RUNNING,
        };
    }

    pub fn apply_vopcode(&mut self, vopcode: &Vopcode) {
        if self.state != ExecutionState::RUNNING {
            panic!("Trying to run a stopped stack")
        }
        let opcode: Opcode = vopcode.opcode;

        if opcode.n_stack_input() > self.stack.len() {
            self.state = ExecutionState::REVERT;
            return;
        }

        self.state = match opcode {
            Opcode::STOP | Opcode::RETURN => ExecutionState::RETURN,
            Opcode::REVERT | Opcode::INVALID => ExecutionState::REVERT,
            Opcode::SELFDESTRUCT => ExecutionState::SELFDESTRUCT,
            Opcode::JUMP => ExecutionState::JUMP(self.stack.pop()),
            Opcode::JUMPI => ExecutionState::JUMPI(self.stack.pop(), self.stack.pop()),
            _ => ExecutionState::RUNNING,
        };

        if self.state != ExecutionState::RUNNING {
            return;
        }

        if let Some(_) = opcode.as_push() {
            self.stack.push(Expression::VALUE(vopcode.value.unwrap()));
        } else if let Some(index) = opcode.as_dup() {
            self.stack.dup(index);
        } else if let Some(index) = opcode.as_swap() {
            self.stack.swap(index);
        } else {
            let mut consumed_expressions: Vec<Box<Expression>> = vec![];
            for _ in 0..opcode.n_stack_input() {
                consumed_expressions.push(Box::new(self.stack.pop()));
            }
            if opcode.has_stack_output() {
                self.stack
                    .push(Expression::COMPOSE(opcode, consumed_expressions));
            }
        }
    }

    pub fn run(&self, code: &[Vopcode]) -> Context {
        let mut final_context: Context = self.clone();
        for vopcode in code {
            final_context.apply_vopcode(vopcode);

            if final_context.state != ExecutionState::RUNNING {
                break;
            }
        }
        return final_context;
    }
}
