use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::tools::{stack::Stack, utils::u256_to_hex};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum SimpleStackExpression {
    BYTES(U256),
    OTHER,
}

impl ToString for SimpleStackExpression {
    fn to_string(&self) -> String {
        return match self {
            SimpleStackExpression::OTHER => String::from("◻"),
            SimpleStackExpression::BYTES(value) => format!("{}", u256_to_hex(*value)),
        };
    }
}

pub type SimpleStack = Stack<SimpleStackExpression>;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum State {
    RUNNING,
    STOP,
    JUMP(Vec<usize>),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
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
