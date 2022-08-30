use super::expression::Expression;
use super::stack::Stack;
use super::state::ExecutionState;
use crate::bytecode_reader::{opcode, opcode::Opcode, vopcode::Vopcode};
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

        if opcode.stack_input > self.stack.len() {
            self.state = ExecutionState::REVERT;
            return;
        }

        if opcode.is_invalid() {
            self.state = ExecutionState::REVERT
        } else {
            self.state = match opcode {
                opcode::STOP | opcode::RETURN => ExecutionState::RETURN,
                opcode::REVERT => ExecutionState::REVERT,
                opcode::SELFDESTRUCT => ExecutionState::SELFDESTRUCT,
                opcode::JUMP => ExecutionState::JUMP(self.stack.pop()),
                opcode::JUMPI => ExecutionState::JUMPI(self.stack.pop(), self.stack.pop()),
                _ => ExecutionState::RUNNING,
            };
        }

        if self.state != ExecutionState::RUNNING {
            return;
        }

        if opcode.is_push() {
            if let Some(pushed) = vopcode.value {
                self.stack.push(Expression::VALUE(pushed));
            } else {
                self.state = ExecutionState::REVERT;
                return;
            }
        } else if opcode.is_dup() {
            self.stack.dup(opcode.n);
        } else if opcode.is_swap() {
            self.stack.swap(opcode.n);
        } else {
            let mut consumed_expressions: Vec<Box<Expression>> = vec![];
            for _ in 0..opcode.stack_input {
                consumed_expressions.push(Box::new(self.stack.pop()));
            }
            if opcode.stack_output > 0 {
                self.stack.push(
                    Expression::COMPOSE(opcode, consumed_expressions)
                );
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
