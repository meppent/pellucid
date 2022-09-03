use std::rc::Rc;

use primitive_types::U256;

use crate::{
    bytecode_reader::{opcode::Opcode, vopcode::Vopcode},
    tools::stack::Stack,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SymbolicExpression {
    stack_expression: StackExpression,
    effect: Option<Rc<Effect>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StackExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<StackExpression>),
    ARG(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Effect {
    COMPOSE(Opcode, Vec<StackExpression>),
}

pub struct SymbolicBlock {
    symbolic_expressions: Stack<SymbolicExpression>,
    effects: Vec<Effect>,
    delta: usize,
    delta_max: usize,
}

impl SymbolicBlock {
    pub fn from(bytecode: &[Vopcode]) -> Self {
        let mut symbolic_expressions: Stack<SymbolicExpression> = Stack::new();
        let mut effects: Vec<Effect> = Vec::new();
        let mut delta: usize = 0;
        let mut delta_max: usize = 0;

        return SymbolicBlock {
            symbolic_expressions,
            effects,
            delta,
            delta_max,
        };
    }

    pub fn pop(&
}
