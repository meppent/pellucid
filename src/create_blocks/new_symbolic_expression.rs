use std::rc::Rc;

use primitive_types::U256;

use crate::{
    bytecode_reader::{
        opcode::Opcode,
        vopcode::{self, Vopcode},
    },
    tools::stack::Stack,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SymbolicExpression {
    stack_expression: StackExpression,
    effect: Option<Rc<Effect>>,
}

impl SymbolicExpression {
    pub fn new(stack_expression: StackExpression, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression {
            stack_expression,
            effect,
        };
    }

    pub fn new_bytes(value: U256, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::BYTES(value), effect);
    }

    pub fn new_compose(opcode: Opcode, args: Vec<StackExpression>, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::COMPOSE(opcode, args), effect);
    }

    pub fn new_arg(index: usize, effect: Option<Rc<Effect>>) -> Self {
        return SymbolicExpression::new(StackExpression::ARG(index), effect);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StackExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<StackExpression>),
    ARG(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Effect {
    COMPOSE(Opcode, Vec<StackExpression>),
}

pub struct SymbolicBlock {
    symbolic_expressions: Stack<SymbolicExpression>,
    effects: Vec<Effect>,
    delta: usize,
    delta_max: usize,
}

impl SymbolicBlock {
    pub fn from(code: &[Vopcode]) -> Self {
        let mut symbolic_block: SymbolicBlock = SymbolicBlock {
            symbolic_expressions: Stack::new(),
            effects: Vec::new(),
            delta: 0,
            delta_max: 0,
        };

        for vopcode in code {
            symbolic_block.add_vopcode(vopcode);
        }

        return symbolic_block;
    }

    pub fn add_vopcode(&mut self, vopcode: &Vopcode) {
        match vopcode.opcode {
            Opcode::PUSH { item_size: _ } => {
                self.symbolic_expressions
                    .push(SymbolicExpression::new_bytes(vopcode.value.unwrap(), None));
            }

            Opcode::DUP { depth } => self.dup(depth),

            Opcode::SWAP { depth } => self.swap(depth),
            opcode => {
                if !opcode.has_effect(){

                }
                else{
                    let args = opcode.stack_input();
                    let initial_len = self.len();
                    let local_delta = if args > self.len() {args - initial_len} else { 0 };
                    let mut vec_symbolic_expr = Vec::new();
                    for i in 0..args {
                        if i < initial_len {
                            vec_symbolic_expr.push(self.pop());
                        } else {
                            vec_symbolic_expr.push(StackExpression::new_arg(self.delta_max + i - initial_len + 1, None))
                        }
                    }
                    self.delta_max += local_delta;
                    self.push(SymbolicExpression::new_compose(opcode, vec_symbolic_expr, None))
    
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.symbolic_expressions.len();
    }

    pub fn pop(&mut self) -> SymbolicExpression {
        return self.symbolic_expressions.pop();
    }

    pub fn push(&mut self, symbolic_expression: SymbolicExpression) {
        self.symbolic_expressions.push(symbolic_expression);
    }

    pub fn peek(&self) -> &SymbolicExpression {
        return self.symbolic_expressions.peek();
    }

    pub fn swap(&mut self, depth: usize) {
        self.symbolic_expressions.swap(depth);
    }

    pub fn dup(&mut self, depth: usize) {
        self.symbolic_expressions.dup(depth);
        let mut to_change = self.symbolic_expressions.pop();
        to_change.effect = None;
        self.symbolic_expressions.push(to_change);
    }
}
