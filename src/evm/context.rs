use super::expression::Expression;
use super::stack::Stack;
use super::state::ExecutionState;
use crate::bytecode_reader::{opcode::Opcode, vopcode::Vopcode};
use core::fmt::Debug;
use std::{hash::Hash, vec};

#[derive(Clone, Debug, Hash)]
pub struct Context<Expr: Expression + Hash + Debug + Clone> {
    pub stack: Stack<Expr>,
    pub state: ExecutionState<Expr>,
}

impl<Expr: Expression + Hash + Debug + Clone> Context<Expr> {
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

    // pub fn apply_vopcode(&mut self, vopcode: &Vopcode) {
    //     if self.state != ExecutionState::RUNNING {
    //         panic!("Trying to run a stopped stack")
    //     }
    //     let opcode: Opcode = vopcode.opcode;

    //     if opcode.stack_input() > self.stack.len() {
    //         self.state = ExecutionState::REVERT;
    //         return;
    //     }

    //     if opcode.is_invalid() {
    //         self.state = ExecutionState::REVERT
    //     } else {
    //         self.state = match opcode {
    //             Opcode::STOP | Opcode::RETURN => ExecutionState::RETURN,
    //             Opcode::REVERT => ExecutionState::REVERT,
    //             Opcode::SELFDESTRUCT => ExecutionState::SELFDESTRUCT,
    //             Opcode::JUMP => ExecutionState::JUMP(self.stack.pop()),
    //             Opcode::JUMPI => ExecutionState::JUMPI(self.stack.pop(), self.stack.pop()),
    //             _ => ExecutionState::RUNNING,
    //         };
    //     }

    //     if self.state != ExecutionState::RUNNING {
    //         return;
    //     }

    //     match opcode {
    //         Opcode::PUSH { item_size: _ } => {
    //             if let Some(pushed) = vopcode.value {
    //                 self.stack.push(Expression::PUSH(pushed));
    //             } else {
    //                 self.state = ExecutionState::REVERT;
    //                 return;
    //             }
    //         }
    //         Opcode::DUP { depth } => {
    //             self.stack.dup(depth);
    //         }
    //         Opcode::SWAP { depth } => {
    //             self.stack.swap(depth);
    //         }
    //         _ => {
    //             let mut consumed_expressions: Vec<Box<Expression>> = vec![];
    //             for _ in 0..opcode.stack_input() {
    //                 consumed_expressions.push(Box::new(self.stack.pop()));
    //             }
    //             if opcode.stack_output() > 0 {
    //                 self.stack
    //                     .push(Expression::COMPOSE(opcode, consumed_expressions));
    //             }
    //         }
    //     }
    // }

    // pub fn run(&self, code: &[Vopcode]) -> Context<Expr> {
    //     let mut final_context: Context<Expr> = self.clone();
    //     for vopcode in code {
    //         final_context.apply_vopcode(vopcode);

    //         if final_context.state != ExecutionState::RUNNING {
    //             break;
    //         }
    //     }
    //     return final_context;
    // }
}
