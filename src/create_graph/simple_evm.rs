use primitive_types::U256;
use serde::Serialize;

use crate::tools::stack::Stack;

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub enum SimpleStackExpression {
    BYTES(U256),
    OTHER,
}

pub type SimpleStack = Stack<SimpleStackExpression>;

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub enum State {
    RUNNING,
    STOP,
    JUMP(Vec<usize>),
}

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub struct SimpleContext {
    pub stack: SimpleStack,
    pub state: State,
}

impl SimpleContext {
    pub fn new() -> Self {
        return SimpleContext {
            stack: SimpleStack::new(),
            state: State::RUNNING,
        };
    }

}
