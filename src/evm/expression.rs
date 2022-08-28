#![allow(dead_code)]
use primitive_types::U256;

use crate::bytecode_reader::opcode::Opcode;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Expression {
    VALUE(U256),
    COMPOSE(Opcode, Vec<Box<Expression>>),
}

impl Expression {
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Expression::VALUE(n) => Some(*n != U256::zero()),
            _ => None,
        }
    }

    pub fn true_expr() -> Self {
        return Expression::VALUE(U256::from(1));
    }

    pub fn false_expr() -> Self {
        return Expression::VALUE(U256::from(0));
    }
}
