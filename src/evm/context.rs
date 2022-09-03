use crate::bytecode_reader::vopcode::Vopcode;

use super::execution_state::ExecutionState;
use super::expressions::expression::Expression;
use super::stack::Stack;
use core::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, Hash)]
pub struct Context<Expr: Expression> {
    pub stack: Stack<Expr>,
    pub state: ExecutionState<Expr>,
}

impl<Expr: Expression> Context<Expr> {
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

    pub fn run(&self, code: &[Vopcode]) -> Context<Expr> {
        let mut final_context: Context<Expr> = self.clone();
        for vopcode in code {
            Expr::apply_vopcode_on_context(&mut final_context, vopcode);
            if final_context.state != ExecutionState::RUNNING {
                break;
            }
        }
        return final_context;
    }
}
