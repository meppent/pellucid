use std::rc::Rc;

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::bytecode_reader::opcode::Opcode;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum StackExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<SymbolicExpression>),
    ARG(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effect {
    pub opcode: Opcode,
    pub symbolic_expressions: Vec<SymbolicExpression>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicExpression {
    pub stack_expression: StackExpression,
    pub origin_effect: Option<Rc<Effect>>,
}

impl StackExpression {
    pub fn compute_value(&self) -> Option<U256> {
        match self {
            StackExpression::BYTES(value) => Some(*value),
            StackExpression::ARG(_) => None,
            StackExpression::COMPOSE(opcode, symbolic_expressions) => {
                let eval_function = opcode.get_function()?;
                let mut params: Vec<U256> = Vec::new();
                for expr in symbolic_expressions {
                    params.push(expr.stack_expression.compute_value()?)
                }
                return Some(eval_function(params));
            }
        }
    }
}

impl SymbolicExpression {
    pub fn new(stack_expression: StackExpression, origin_effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression {
            stack_expression,
            origin_effect,
        };
    }

    pub fn new_bytes(value: U256, origin_effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::BYTES(value), origin_effect);
    }

    pub fn new_compose(
        opcode: Opcode,
        args: Vec<SymbolicExpression>,
        origin_effect: Option<Rc<Effect>>,
    ) -> Self {
        return SymbolicExpression::new(StackExpression::COMPOSE(opcode, args), origin_effect);
    }

    pub fn new_arg(index: usize, origin_effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::ARG(index), origin_effect);
    }
}
