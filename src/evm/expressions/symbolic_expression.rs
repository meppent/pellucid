use crate::{
    bytecode_reader::{opcode::Opcode, vopcode::Vopcode},
    evm::{context::Context, execution_state::ExecutionState, stack::Stack},
};
use primitive_types::U256;

use super::expression::{check_state_transition, Expression};

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SymbolicExpression {
    PUSH(U256),
    COMPOSE(Opcode, Vec<Box<SymbolicExpression>>),
}

impl PartialEq for Stack<SymbolicExpression> {
    fn eq(&self, other: &Self) -> bool {
        return self._get_data() == other._get_data();
    }
}

impl Expression for SymbolicExpression {
    fn apply_vopcode_on_context(context: &mut Context<Self>, vopcode: &Vopcode) {
        let opcode: Opcode = vopcode.opcode;
        check_state_transition(context, &opcode);
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
                let mut consumed_expressions: Vec<Box<Self>> = vec![];
                for _ in 0..opcode.stack_input() {
                    consumed_expressions.push(Box::new(context.stack.pop()));
                }
                if opcode.stack_output() > 0 {
                    context
                        .stack
                        .push(Self::COMPOSE(opcode, consumed_expressions));
                }
            }
        }
    }
}
