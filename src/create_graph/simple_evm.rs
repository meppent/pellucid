use std::rc::Rc;

use primitive_types::U256;
use serde::Serialize;

use crate::{tools::stack::Stack, create_blocks::symbolic_expression::Effect};

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub enum SimpleStackExpression {
    BYTES(U256),
    OTHER,
}

pub type SimpleStack = Stack<SimpleStackExpression>;

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub enum State{
    RUNNING,
    STOP,
    JUMP(Vec<U256>),
}

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub struct SimpleContext{
    pub stack: SimpleStack,
    pub state: State

}

