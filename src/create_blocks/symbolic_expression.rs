use crate::bytecode_reader::bytecode::Bytecode;
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::vopcode::Vopcode;
use crate::tools::stack::Stack;
use primitive_types::U256;

// pub(crate) struct SymbolicExpression{
//     stack_expression: StackExpression,
//     effect: Effect,
// }


// #[derive(Debug, PartialEq, Clone, Hash)]
// pub enum StackExpression {
//     BYTES(U256),
//     COMPOSE(Opcode, Vec<SymbolicExpression>),
//     ARG(usize),
// }

// struct Effect {
//     inner: Rc<>
// }


// impl SymbolicStack {}
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SymbolicStack {
    inner: Stack<StackExpression>,
    delta: usize,
    delta_max: usize,
}

// pub enum StackExpression {
//     BYTES(U256),
//     COMPOSE(Opcode, Vec<SymbolicExpression>),
//     ARG(usize),
// }
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum StackExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<StackExpression>),
    ARG(usize),
}
impl SymbolicStack {
    pub fn new() -> SymbolicStack {
        SymbolicStack {
            inner: Stack::new(),
            delta: 0,
            delta_max: 0,
        }
    }

    pub fn add_vopcode(&mut self, vopcode: Vopcode) {
        match vopcode.opcode {

            Opcode::PUSH {item_size: _} => self.push(
                StackExpression::BYTES(vopcode.value.unwrap())
            ),
            
            Opcode::DUP {depth} => self.dup(depth),
            
            Opcode::SWAP {depth} => self.swap(depth),

            Opcode::POP => {
                let _ = self.pop();
            },
                
            opcode => {
                let args = opcode.stack_input();
                let initial_len = self.len();
                let local_delta = if args > self.len() {args - initial_len} else { 0 };
                let mut vec_symbolic_expr = Vec::new();
                for i in 0..args {
                    if i < initial_len {
                        vec_symbolic_expr.push(self.pop());
                    } else {
                        vec_symbolic_expr.push(StackExpression::ARG(self.delta_max + i - initial_len + 1))
                    }
                }
                self.delta_max += local_delta;
                self.push(StackExpression::COMPOSE(opcode, vec_symbolic_expr))

            }
        }
    }


    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn peek(&self) -> &StackExpression {
        self.inner.peek()
    }

    pub fn pop(&mut self) -> StackExpression {
        self.inner.pop()
    }

    pub fn push(&mut self, expr: StackExpression) {
        self.inner.push(expr)
    }

    pub fn swap(&mut self, depth: usize) {
        self.inner.swap(depth)
    }

    pub fn dup(&mut self, depth: usize) {
        self.inner.dup(depth)
    }

    // pub fn from(bytecode: &[Vopcode]) -> SymbolicStack {
    //     let mut stack = SymbolicStack::new();
    //     for vopcode in bytecode {
    //         match vopcode.opcode {

    //         }
    //     }
    //     return stack;
    // }
}
