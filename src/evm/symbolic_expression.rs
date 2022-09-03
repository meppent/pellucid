use crate::bytecode_reader::bytecode::Bytecode;
use crate::bytecode_reader::opcode::Opcode;
use crate::bytecode_reader::vopcode::Vopcode;
use crate::tools::stack::Stack;
use primitive_types::U256;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SymbolicExpression {
    BYTES(U256),
    COMPOSE(Opcode, Vec<SymbolicExpression>),
    ARG(isize),
}

impl SymbolicExpression {}
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct SymbolicStack {
    inner: Stack<SymbolicExpression>,
}

impl SymbolicStack {
    pub fn new() -> SymbolicStack {
        SymbolicStack {
            inner: Stack::new(),
        }
    }

    pub fn add_vopcode(&mut self, vopcode: Vopcode) {
        match vopcode.opcode {
            Opcode::PUSH { item_size } => self
                .inner
                .push(SymbolicExpression::BYTES(vopcode.value.unwrap())),
        }
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
