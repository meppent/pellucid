use crate::bytecode_reader::bytecode::Bytecode;
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::vopcode::Vopcode;
use crate::tools::stack::Stack;
use primitive_types::U256;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SymbolicExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<SymbolicExpression>),
    ARG(usize),
}

impl SymbolicExpression {}
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SymbolicStack {
    inner: Stack<SymbolicExpression>,
    delta: usize,
    delta_max: usize,
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
                SymbolicExpression::BYTES(vopcode.value.unwrap())
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
                        vec_symbolic_expr.push(SymbolicExpression::ARG(self.delta_max + i - initial_len + 1))
                    }
                }
                self.delta_max += local_delta;
                self.push(SymbolicExpression::COMPOSE(opcode, vec_symbolic_expr))

            }
        }
    }


    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn peek(&self) -> &SymbolicExpression {
        self.inner.peek()
    }

    pub fn pop(&mut self) -> SymbolicExpression {
        self.inner.pop()
    }

    pub fn push(&mut self, expr: SymbolicExpression) {
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
