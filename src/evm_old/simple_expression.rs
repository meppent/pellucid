use primitive_types::U256;

use crate::tools::stack::Stack;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SimpleExpression {
    BYTES(U256),
    OTHER,
}

type SimpleStack = Stack<SimpleExpression>;
