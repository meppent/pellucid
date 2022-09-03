use crate::{
    bytecode_reader::{opcode::Opcode, vopcode::Vopcode},
    evm::{context::Context, execution_state::ExecutionState},
};

use primitive_types::U256;

use super::expression::{apply_state_transition, Expression};

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SparseExpression {
    PUSH(U256),
    OTHER,
}

impl Expression for SparseExpression {
    fn apply_vopcode_on_context(context: &mut Context<Self>, vopcode: &Vopcode) {
        let opcode: Opcode = vopcode.opcode;
        apply_state_transition(context, &opcode);
        if context.state != ExecutionState::RUNNING {
            return;
        }
        match opcode {
            Opcode::PUSH { item_size: _ } => {
                context.stack.push(Self::PUSH(vopcode.value.unwrap()));
            }
            Opcode::DUP { depth } => {
                context.stack.dup(depth);
            }
            Opcode::SWAP { depth } => {
                context.stack.swap(depth);
            }
            _ => {
                for _ in 0..opcode.stack_input() {
                    context.stack.pop();
                }
                if opcode.stack_output() > 0 {
                    context.stack.push(Self::OTHER);
                }
            }
        }
    }
}
