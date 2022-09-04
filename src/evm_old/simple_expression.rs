use primitive_types::U256;
use serde::Serialize;

use crate::tools::stack::Stack;

#[derive(Debug, PartialEq, Clone, Hash, Serialize)]
pub enum SimpleExpression {
    BYTES(U256),
    OTHER,
}

type SimpleStack = Stack<SimpleExpression>;
