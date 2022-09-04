use std::ops::Deref;

use crate::bytecode_reader::opcode::Opcode;
use crate::tools::stack::Stack;
use primitive_types::U256;

use super::symbolic_expression::SymbolicExpression;

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SymbolicStack {
    stack: Stack<SymbolicExpression>,
    delta: usize,
    delta_max: usize,
}

impl Deref for SymbolicStack {
    type Target = Stack<SymbolicExpression>;

    fn deref(&self) -> &Self::Target {
        &self.stack
    }

}

impl SymbolicStack {
    pub fn new() -> SymbolicStack {
        SymbolicStack {
            stack: Stack::new(),
            delta: 0,
            delta_max: 0,
        }
    }
}
