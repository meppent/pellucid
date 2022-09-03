use std::{fmt::Debug, hash::Hash};

use crate::{
    bytecode_reader::{opcode::Opcode, vopcode::Vopcode},
    evm::{context::Context, execution_state::ExecutionState, stack::Stack},
};
// something we put on the stack

pub trait Expression: Debug + PartialEq + Clone + Hash {
    fn apply_vopcode_on_context(context: &mut Context<Self>, vopcode: &Vopcode);
}

impl<Expr: Expression> PartialEq for Stack<Expr> {
    // we could also use separate PartialEq implem for each Expression
    fn eq(&self, other: &Self) -> bool {
        return self._get_data() == other._get_data();
    }
}

pub fn apply_state_transition<Expr: Expression>(context: &mut Context<Expr>, opcode: &Opcode) {
    if context.state != ExecutionState::RUNNING {
        panic!("Trying to run a stopped stack")
    }

    if opcode.stack_input() > context.stack.len() {
        context.state = ExecutionState::REVERT;
        return;
    }

    if opcode.is_invalid() {
        context.state = ExecutionState::REVERT
    } else {
        context.state = match opcode {
            Opcode::STOP | Opcode::RETURN => ExecutionState::RETURN,
            Opcode::REVERT => ExecutionState::REVERT,
            Opcode::SELFDESTRUCT => ExecutionState::SELFDESTRUCT,
            Opcode::JUMP => ExecutionState::JUMP {
                dest: context.stack.pop(),
            },
            Opcode::JUMPI => ExecutionState::JUMPI {
                dest: context.stack.pop(),
                condition: context.stack.pop(),
            },
            _ => ExecutionState::RUNNING,
        };
    }
}
