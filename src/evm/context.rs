use crate::tools::stack::Stack;
use super::execution_state::ExecutionState;
use super::simple_expression::SimpleExpression;
use core::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Debug, Hash)]
pub struct Context{
    pub stack: Stack<SimpleExpression>,
    pub state: ExecutionState,
}

impl Context{
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

}
