use crate::bytecode_reader::opcode::Opcode;
use primitive_types::U256;

pub trait Expression {
    // something we put on the stack
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SymbolicExpression {
    PUSH(U256),
    COMPOSE(Opcode, Vec<Box<SymbolicExpression>>),
}
